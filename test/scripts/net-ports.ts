// net-ports.ts
// Node >=18 recommended (fs/promises, stable ESM). Linux-only (/proc).
/*
README
======

A tiny Unix-y CLI to inspect TCP listeners from /proc, to find port conflicts for litep2p.

SO_REUSEPORT allows multiple processes to listen on the same TCP port. This utility is to check if that's happening in
zombienet tests, because if 2 processes are listening on the same p2p port, they won't be able to connect to each other.

USAGE (quick):
  pnpm net-ports check-conflicts --all
  pnpm net-ports by-pid --names "tanssi-node,tanssi-relay,polkadot"
  pnpm net-ports by-port -p 30335
  pnpm net-ports probe-reuseport -p 30335
  pnpm net-ports connections-between --pids "123,456,789"
  pnpm net-ports connections-between --names "tanssi-node,tanssi-relay,polkadot"

NAME MATCHING:
  --names / --name → exact match on process *name* (i.e., /proc/<pid>/comm). No grep, no args. Case-sensitive.

EXAMPLES:
  net-ports by-port -p 30335
  net-ports by-port -p 30335 --json
  net-ports by-pid --names "tanssi-node,tanssi-relay"
  net-ports by-pid --name tanssi-node
  net-ports check-conflicts --all
  net-ports check-conflicts --pids "123,456" --exit-code
  net-ports probe-reuseport -p 30335
  net-ports connections-between --pids "111,222" --json

FALLBACK (Linux utilities)
-------------------------
If this script ever breaks, you can get the same information using stock tools like `ss` (and a bit of awk).

• List listeners on a specific port (like `by-port`):
    ss -Hltpn 'sport = :30335'
  (Shows PID/command of processes listening on TCP 30335.)

• Show listening ports for a given PID (like `by-pid`):
    pid=1234; ss -Hltpn | awk -v pid="$pid" '
      /LISTEN/ && match($0, /pid=([0-9]+)/, p) && p[1]==pid {
        if (match($4, /:([0-9]+)$/, m)) print m[1]
      }' | sort -n | uniq

• Detect ports listened by >1 unique PID (SO_REUSEPORT conflicts; like `check-conflicts`):
    ss -Hltpn | awk '
      /LISTEN/ {
        if (match($4, /:([0-9]+)$/, m) && match($0, /pid=([0-9]+)/, p)) {
          seen[m[1], p[1]] = 1
        }
      }
      END {
        for (k in seen) { split(k, a, SUBSEP); cnt[a[1]]++ }
        PROCINFO["sorted_in"]="@ind_num_asc"
        printf "%-6s %-1s\n","PORT","N"
        printf "%-6s %-1s\n","----","-"
        for (port in cnt) if (cnt[port] > 1) printf "%-6d %d\n", port, cnt[port]
      }'
*/

import { execFile } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import net from "node:net";
import yargs from "yargs";
import type { Options, ArgumentsCamelCase, InferredOptionTypes } from "yargs";
import { hideBin } from "yargs/helpers";

/** A single row parsed from /proc/net/tcp* */
type TcpRow = {
    inode: number;
    localPort: number;
    protocol: "tcp4" | "tcp6";
    stateHex: string; // "0A" = LISTEN
    localHex: string; // 8-hex little-endian IPv4 (from /proc)
    remHex: string; // 8-hex little-endian IPv4 (from /proc)
    remPort: number; // remote port
};

type Listener = { ip: string; port: number };

type ConnEntry = {
    inode: number;
    protocol: "tcp4";
    stateHex: string;
    state: string;
    localIp: string; // dotted (pretty)
    localPort: number;
    remIp: string; // dotted (pretty)
    remPort: number;
    localHex: string; // 8-hex LE (e.g., "0100007F")
    remHex: string; // 8-hex LE
};

type ProbeError = { code?: string; message: string };

type JsonConnectionsBetween = {
    command: "connections-between";
    nodes: Array<{
        pid: number;
        name: string;
        netns: string;
        listeners_v4: Listener[];
    }>;
    edges: Array<{
        aPid: number;
        bPid: number;
        a: ConnEntry;
        b: ConnEntry;
    }>;
};

type JsonProbeReusePort = {
    command: "probe-reuseport";
    port: number;
    host: string;
    node: string;
    before: { pid: number; name: string }[];
    bound: boolean;
    error?: ProbeError;
};

type JsonOut =
    | { command: "by-port"; port: number; listeners: { pid: number; name: string }[] }
    | { command: "by-pid"; entries: { pid: number; name: string; ports: number[] }[] }
    | { command: "check-conflicts"; conflicts: { port: number; pids: number[]; names: Record<number, string> }[] }
    | JsonProbeReusePort
    | JsonConnectionsBetween;

/** Standardized exit codes for nice shell ergonomics. */
const EC = {
    OK: 0,
    NO_MATCH: 1, // used with --exit-code on conflicts (diff-like)
    USAGE: 64, // EX_USAGE
    NOPERM: 77, // EX_NOPERM
    OSERR: 71, // EX_OSERR
    PLATFORM: 78, // wrong OS (/proc missing)
} as const;

const globalOptions = {
    json: {
        type: "boolean",
        default: false,
        describe: "Output machine-readable JSON.",
    },
} as const;

type GlobalOptions = InferredOptionTypes<typeof globalOptions>;

// Parse --names "tanssi-node,polkadot" into ["tanssi-node", "polkadot"]
function parseStringList(input: string | string[]): string[] {
    const raw = Array.isArray(input) ? input.join(",") : input;

    return raw
        .split(/[,\s]+/) // split on commas *or* whitespace
        .filter(Boolean); // drop empty chunks
}

// Parse --pids "1,2,3" into [1, 2, 3]
function parseNumberList(input: string | string[]): number[] {
    return parseStringList(input).map((s) => {
        const n = Number(s);
        if (!Number.isFinite(n)) {
            throw new Error(`Invalid pid '${s}' in list`);
        }
        return n;
    });
}

const LISTEN_HEX = "0A";
const TCP_STATES: Record<string, string> = {
    "01": "ESTABLISHED",
    "02": "SYN_SENT",
    "03": "SYN_RECV",
    "04": "FIN_WAIT1",
    "05": "FIN_WAIT2",
    "06": "TIME_WAIT",
    "07": "CLOSE",
    "08": "CLOSE_WAIT",
    "09": "LAST_ACK",
    "0A": "LISTEN",
    "0B": "CLOSING",
    "0C": "NEW_SYN_RECV",
};

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// ---------- small utils ----------
const pad = (s: string, w: number) => (s.length >= w ? s : s + " ".repeat(w - s.length));

const nameCache = new Map<number, string>();

/** Print error and exit. */
function die(msg: string, code = EC.USAGE) {
    console.error("error:", msg);
    process.exit(code);
}

/** Guard: this tool relies on /proc, so Linux-only. */
function ensureLinux() {
    if (process.platform !== "linux") {
        console.error("This CLI requires Linux (/proc).");
        process.exit(EC.PLATFORM);
    }
}

/** True iff row is in LISTEN state. */
function isListen(r: TcpRow) {
    return r.stateHex.toUpperCase() === LISTEN_HEX;
}

/** Quick feature flag for reusePort support in Node. */
function nodeSupportsReusePort(): boolean {
    const [majStr, minStr] = process.versions.node.split(".");
    const maj = Number(majStr) || 0;
    const min = Number(minStr) || 0;
    if (maj >= 23) return true;
    if (maj === 22 && min >= 12) return true;
    return false;
}

/**
 * Resolve a PID's display name.
 * Prefers /proc/<pid>/comm; falls back to full cmdline; caches results.
 */
async function procName(pid: number): Promise<string> {
    const cached = nameCache.get(pid);
    if (cached) return cached;
    try {
        const comm = await fs.readFile(`/proc/${pid}/comm`, "utf8");
        const trimmed = comm.replace(/\n+$/, "");
        if (trimmed) {
            nameCache.set(pid, trimmed);
            return trimmed;
        }
    } catch {}
    try {
        const cmd = await fs.readFile(`/proc/${pid}/cmdline`);
        const s = Buffer.from(cmd).toString("utf8").replace(/\0/g, " ").trim();
        if (s) {
            nameCache.set(pid, s);
            return s;
        }
    } catch {}
    nameCache.set(pid, "?");
    return "?";
}

/** Read the netns symlink for a PID (for context). */
async function readNetNs(pid: number): Promise<string> {
    try {
        return await fs.readlink(`/proc/${pid}/ns/net`);
    } catch {
        return "?";
    }
}

// ---------- /proc parsing ----------
/** Read both tcp4 and (optionally) tcp6 tables. */
async function readTcpTables(include6: boolean): Promise<TcpRow[]> {
    const rows4 = await parseProcNetTcp("/proc/net/tcp", "tcp4");
    if (!include6) return rows4;
    const rows6 = await parseProcNetTcp("/proc/net/tcp6", "tcp6");
    return rows4.concat(rows6);
}

/** Read tcp tables from a specific PID's network namespace. */
async function readTcpTablesForPid(pid: number, include6: boolean): Promise<TcpRow[]> {
    const rows4 = await parseProcNetTcp(`/proc/${pid}/net/tcp`, "tcp4");
    if (!include6) return rows4;
    const rows6 = await parseProcNetTcp(`/proc/${pid}/net/tcp6`, "tcp6");
    return rows4.concat(rows6);
}

/** Convert little-endian hex IPv4 (e.g. "0100007F") to dotted "127.0.0.1". */
export function hexToIPv4(hex: string): string {
    const s = hex.trim();

    // Strict: exactly 8 hex digits, no prefix, no separators.
    if (!/^[0-9a-fA-F]{8}$/.test(s)) {
        throw new Error(`Invalid IPv4 hex: "${hex}"`);
    }

    // /proc/net/tcp-style is little-endian: 01 00 00 7F => 127.0.0.1
    const octets = [
        parseInt(s.slice(6, 8), 16), // last byte -> first octet
        parseInt(s.slice(4, 6), 16),
        parseInt(s.slice(2, 4), 16),
        parseInt(s.slice(0, 2), 16), // first byte -> last octet
    ];

    return octets.join(".");
}

/** Parse /proc/net/tcp or /proc/net/tcp6 into structured rows. */

async function parseProcNetTcp(file: string, protocol: "tcp4" | "tcp6"): Promise<TcpRow[]> {
    let txt = "";
    try {
        txt = await fs.readFile(file, "utf8");
    } catch {
        return [];
    }
    const out: TcpRow[] = [];
    const lines = txt.split(/\r?\n/);
    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;
        const cols = line.split(/\s+/);
        if (cols.length < 10) continue;

        const local = cols[1]; // "HEXADDR:PORTHEX"
        const rem = cols[2]; // "HEXADDR:PORTHEX"
        const stateHex = cols[3]?.toUpperCase() || "";
        const inodeStr = cols[9];

        const colonIdxL = local.lastIndexOf(":");
        const colonIdxR = rem.lastIndexOf(":");
        if (colonIdxL < 0 || colonIdxR < 0) continue;

        const localHex = local.slice(0, colonIdxL).toUpperCase();
        const portHex = local.slice(colonIdxL + 1);
        const remHex = rem.slice(0, colonIdxR).toUpperCase();
        const remPortHex = rem.slice(colonIdxR + 1);

        const localPort = Number.parseInt(portHex, 16);
        const remPort = Number.parseInt(remPortHex, 16);
        const inode = Number.parseInt(inodeStr, 10);

        if (!Number.isFinite(localPort) || !Number.isFinite(inode)) continue;

        out.push({
            inode,
            localPort,
            protocol,
            stateHex,
            localHex,
            remHex,
            remPort: Number.isFinite(remPort) ? remPort : -1,
        });
    }
    return out;
}

/**
 * Given a set of target socket inodes, find all PIDs owning them.
 * Returns a Map(pid -> Set(inode)) for matched sockets. Efficient for --all.
 */
async function pidsOwningInodes(targetInodes: Set<number>, verbose = false): Promise<Map<number, Set<number>>> {
    const targets = new Set([...targetInodes].map((i) => `socket:[${i}]`));
    const result = new Map<number, Set<number>>();

    let procDirs: string[] = [];
    try {
        const entries = await fs.readdir("/proc", { withFileTypes: true });
        procDirs = entries.filter((d) => d.isDirectory()).map((d) => d.name);
    } catch (e: any) {
        console.error(`warning: cannot read /proc: ${String(e?.message ?? e)}`);
        return result;
    }

    const pidDirs = procDirs.filter((name) => /^\d+$/.test(name));
    for (const pidStr of pidDirs) {
        const pid = Number(pidStr);
        const fdDir = path.join("/proc", pidStr, "fd");
        let fds: string[] = [];
        try {
            const entries = await fs.readdir(fdDir, { withFileTypes: true });
            fds = entries.map((e) => e.name);
        } catch {
            continue; // process exited or insufficient perms
        }
        const reads = await Promise.allSettled(fds.map((fd) => fs.readlink(path.join(fdDir, fd))));
        for (const r of reads) {
            if (r.status === "fulfilled" && targets.has(r.value.toString())) {
                const m = /^socket:\[(\d+)\]$/.exec(r.value.toString());
                if (!m) continue;
                const inode = Number(m[1]);
                let set = result.get(pid);
                if (!set) {
                    set = new Set();
                    result.set(pid, set);
                }
                set.add(inode);
                if (verbose) console.error(`  ✔ PID ${pid} owns ${r.value.toString()}`);
            }
        }
    }
    return result;
}

/** Get all socket inodes opened by a single PID. */
async function inodesForPid(pid: number): Promise<Set<number>> {
    const result = new Set<number>();
    const fdDir = path.join("/proc", String(pid), "fd");
    let fds: string[] = [];

    // Read /proc/<pid>/fd directory
    try {
        const entries = await fs.readdir(fdDir, { withFileTypes: true });
        fds = entries.map((e) => e.name);
    } catch (err) {
        console.warn(`Failed to read fd dir for PID ${pid}:`, err);
        return result;
    }

    try {
        const reads = await Promise.allSettled(fds.map((fd) => fs.readlink(path.join(fdDir, fd))));

        for (const r of reads) {
            if (r.status === "fulfilled") {
                const m = /^socket:\[(\d+)\]$/.exec(r.value.toString());
                if (m) {
                    const ino = Number(m[1]);
                    if (Number.isFinite(ino)) {
                        result.add(ino);
                    }
                }
            } else {
                const e = r.reason as NodeJS.ErrnoException;
                console.warn(`Failed to readlink fd for PID ${pid}:`, e);
            }
        }
    } catch (err) {
        // Promise.allSettled shouldn't reject, but guard anyway
        console.warn(`Unexpected error while inspecting fds for PID ${pid}:`, err);
        return result;
    }

    return result;
}

// ---------- PID/name collection (EXACT by default) ----------

/**
 * Collect a de-duplicated, sorted PID list from:
 *   --pid/--pids (numbers)
 *   --names / --name      (EXACT comm matches only; case-sensitive)
 */
async function collectPidsExact(
    argv: ByPidArgvType | CheckConflictsArgvType | ConnectionsBetweenArgvType
): Promise<number[]> {
    // numeric PIDs from --pids or --pid
    const pidSet = new Set<number>(argv.pids);

    // exact process name matches: --names or --name
    const namesExact = new Set<string>(argv.names);

    if (namesExact.size > 0) {
        const pids = await execPidof(Array.from(namesExact));
        for (const pid of pids) {
            pidSet.add(pid);
        }
    }

    return [...pidSet].sort((a, b) => a - b);
}

/** Resolve PIDs by *exact* match using the `pidof` executable. */
export async function execPidof(names: string[]): Promise<number[]> {
    if (names.length === 0) return [];

    return await new Promise<number[]>((resolve, reject) => {
        execFile("pidof", names, (error, stdout) => {
            if (error) {
                reject(error);
                return;
            }
            // Empty output: behave like "no PIDs"
            if (!stdout) {
                resolve([]);
                return;
            }

            const pids = stdout
                .trim()
                .split(/\s+/)
                .map((s) => Number.parseInt(s, 10))
                .filter((n) => Number.isFinite(n));

            // Unique + sorted, similar to per-name sorted arrays then flattened
            const uniqueSorted = Array.from(new Set(pids)).sort((a, b) => a - b);
            resolve(uniqueSorted);
        });
    });
}

// ---------- pretty printing ----------
/** Print a minimal, aligned table without hard dependencies. */
function printTable(headers: string[], rows: string[][]) {
    const widths = headers.map((h, idx) => Math.max(h.length, ...rows.map((r) => (r[idx] ? r[idx].length : 0))));
    console.log(headers.map((h, i) => pad(h, widths[i])).join("  "));
    console.log(widths.map((w) => "-".repeat(w)).join("  "));
    for (const r of rows) console.log(r.map((c, i) => pad(c, widths[i])).join("  "));
}

function tcpStateName(hex: string): string {
    return TCP_STATES[hex.toUpperCase()] ?? hex;
}

async function listenersForPidV4(pid: number): Promise<Listener[]> {
    const pidInodes = await inodesForPid(pid);
    const rows = await readTcpTablesForPid(pid, false /* IPv4 only */);
    const s = new Set<string>();
    for (const r of rows) {
        if (r.stateHex !== LISTEN_HEX) continue;
        if (!pidInodes.has(r.inode)) continue;
        const ip = hexToIPv4(r.localHex);
        s.add(`${ip}:${r.localPort}`);
    }
    return [...s]
        .map((s) => {
            const [ip, port] = s.split(":");
            return { ip, port: Number(port) };
        })
        .sort((a, b) => (a.ip === b.ip ? a.port - b.port : a.ip.localeCompare(b.ip)));
}

async function connsForPidV4(pid: number): Promise<ConnEntry[]> {
    const pidInodes = await inodesForPid(pid);
    const rows = await readTcpTablesForPid(pid, false /* IPv4 only */);
    const out: ConnEntry[] = [];
    for (const r of rows) {
        if (r.stateHex === LISTEN_HEX) continue;
        if (!pidInodes.has(r.inode)) continue;
        out.push({
            inode: r.inode,
            protocol: "tcp4",
            stateHex: r.stateHex,
            state: tcpStateName(r.stateHex),
            localIp: hexToIPv4(r.localHex),
            localPort: r.localPort,
            remIp: hexToIPv4(r.remHex),
            remPort: r.remPort,
            localHex: r.localHex,
            remHex: r.remHex,
        });
    }
    return out;
}

// Separate yargs options definitions as const values, to be able to get their type

const byPortOptions = {
    port: {
        type: "number",
        alias: ["p"] as const,
        demandOption: true,
        describe: "TCP port (e.g. 30335)",
    },
    ipv6: {
        type: "boolean",
        alias: ["6", "IncludeIPv6"] as const,
        default: false,
        describe: "Also include IPv6 (tcp6)",
    },
    verbose: {
        type: "boolean",
        alias: ["v"] as const,
        default: false,
        describe: "Verbose logs to stderr",
    },
} as const;

const byPidOptions = {
    pids: {
        type: "string",
        alias: ["p", "pid"],
        describe: "Space/comma separated: --pids 1 2 3 or --pids '1,2,3'. Repeatable: -p 1 -p 2 -p 3",
        coerce: parseNumberList,
    },
    names: {
        type: "array",
        alias: ["n", "name"],
        describe: "Exact process names (comm). Comma separated or repeated args: --names 'tanssi-node,polkadot'",
        coerce: parseStringList,
    },
    ipv6: {
        type: "boolean",
        alias: ["6", "IncludeIPv6"],
        default: false,
        describe: "Also include IPv6 (tcp6)",
    },
    verbose: {
        type: "boolean",
        alias: ["v"],
        default: false,
        describe: "Verbose logs to stderr",
    },
} as const;

const checkConflictsOptions = {
    pids: {
        type: "string",
        alias: ["p", "pid"],
        describe: "Space/comma separated: --pids 1 2 3 or --pids '1,2,3'. Repeatable: -p 1 -p 2 -p 3",
        coerce: parseNumberList,
    },
    names: {
        type: "array",
        alias: ["n", "name"],
        describe: "Exact process names (comm). Comma separated or repeated args: --names 'tanssi-node,polkadot'",
        coerce: parseStringList,
    },
    ipv6: {
        type: "boolean",
        alias: ["6", "IncludeIPv6"],
        default: false,
        describe: "Also include IPv6 (tcp6)",
    },
    all: {
        type: "boolean",
        default: false,
        describe: "Check conflicts among ALL running PIDs (system-wide)",
    },
    verbose: {
        type: "boolean",
        alias: ["v"],
        default: false,
        describe: "Verbose logs to stderr",
    },
    "exit-code": {
        type: "boolean",
        default: false,
        describe: "Exit 1 if conflicts found (0 if none).",
    },
} as const;

const probeReusePortOptions = {
    port: {
        type: "number",
        alias: ["p"],
        demandOption: true,
        describe: "TCP port (e.g. 30335)",
    },
    host: { type: "string", describe: "Bind host (default 0.0.0.0 or :: if --ipv6)" },
    ipv6: {
        type: "boolean",
        alias: ["6", "IncludeIPv6"],
        default: false,
        describe: "Prefer IPv6 host (::)",
    },
    ipv6only: { type: "boolean", describe: "Set ipv6Only when binding to :: (no dual-stack)" },
    timeout: { type: "number", default: 400, describe: "How long to hold the socket open (ms)" },
    verbose: {
        type: "boolean",
        alias: ["v"],
        default: false,
        describe: "Verbose logs to stderr",
    },
    "exit-code": { type: "boolean", default: false, describe: "Exit 1 if the bind fails." },
} as const;

const connectionsBetweenOptions = {
    pids: {
        type: "string",
        alias: ["p", "pid"],
        describe: "Space/comma separated: --pids 1 2 3 or --pids '1,2,3'. Repeatable: -p 1 -p 2 -p 3",
        coerce: parseNumberList,
    },
    names: {
        type: "array",
        alias: ["n", "name"],
        describe: "Exact process names (comm). Comma separated or repeated args: --names 'tanssi-node,polkadot'",
        coerce: parseStringList,
    },
} as const;

// Helper to get a type from options object
type CommandArgv<Opts extends { [key: string]: Options }> = ArgumentsCamelCase<
    GlobalOptions & InferredOptionTypes<Opts>
>;
// Types for each command handler
type ByPortArgvType = CommandArgv<typeof byPortOptions>;
type ByPidArgvType = CommandArgv<typeof byPidOptions>;
type CheckConflictsArgvType = CommandArgv<typeof checkConflictsOptions>;
type ProbeReusePortArgvType = CommandArgv<typeof probeReusePortOptions>;
type ConnectionsBetweenArgvType = CommandArgv<typeof connectionsBetweenOptions>;

// ---------- command impls (logic moved out) ----------

async function cmdByPort(argv: ByPortArgvType) {
    ensureLinux();
    const port = argv.port;
    if (!port) die("Invalid --port.", EC.USAGE);
    const include6 = argv.ipv6;
    const verbose = argv.verbose;

    const rows = await readTcpTables(include6);
    const listeners = new Set(rows.filter((r) => isListen(r) && r.localPort === port).map((r) => r.inode));

    if (listeners.size === 0) {
        if (argv.json) {
            const out: JsonOut = { command: "by-port", port, listeners: [] };
            console.log(JSON.stringify(out));
        } else {
            console.log(`No listeners found on port ${port}.`);
        }
        process.exit(EC.OK);
    }

    const pidMap = await pidsOwningInodes(new Set(listeners), verbose);
    const pairs: { pid: number; name: string }[] = [];
    for (const pid of [...pidMap.keys()].sort((a, b) => a - b)) pairs.push({ pid, name: await procName(pid) });

    if (argv.json) {
        const out: JsonOut = { command: "by-port", port, listeners: pairs };
        console.log(JSON.stringify(out));
        process.exit(EC.OK);
    }

    console.log(`Listeners on port ${port}:`);
    printTable(
        ["PID", "NAME"],
        pairs.map((p) => [String(p.pid), p.name])
    );
}

async function cmdByPid(argv: ByPidArgvType) {
    ensureLinux();
    const include6 = argv.ipv6;
    const verbose = argv.verbose;

    const pidList = await collectPidsExact(argv);
    if (pidList.length === 0) die("Provide PIDs via --pid/--pids or names via --names/--name.", EC.USAGE);

    const rows = await readTcpTables(include6);
    const inodeToRow = new Map<number, TcpRow>(rows.map((r) => [r.inode, r]));

    const entries: { pid: number; name: string; ports: number[] }[] = [];
    for (const pid of pidList) {
        const pidInodes = await inodesForPid(pid);
        const listenPorts = new Set<number>();
        for (const ino of pidInodes) {
            const row = inodeToRow.get(ino);
            if (row && isListen(row)) listenPorts.add(row.localPort);
        }
        const name = await procName(pid);
        entries.push({ pid, name, ports: [...listenPorts].sort((a, b) => a - b) });

        if (verbose) {
            for (const ino of pidInodes) {
                const row = inodeToRow.get(ino);
                if (row)
                    console.error(
                        `  ↳ ${pid} ${row.protocol} inode=${row.inode} port=${row.localPort} state=${row.stateHex}`
                    );
            }
        }
    }

    if (argv.json) {
        const out: JsonOut = { command: "by-pid", entries };
        console.log(JSON.stringify(out));
        process.exit(EC.OK);
    }

    printTable(
        ["PID", "NAME", "LISTENING_PORTS"],
        entries.map((e) => [String(e.pid), e.name, e.ports.length ? e.ports.join(", ") : "(none)"])
    );
}

async function cmdCheckConflicts(argv: CheckConflictsArgvType) {
    ensureLinux();
    const include6 = argv.ipv6;
    const verbose = argv.verbose;
    const useAll = argv.all;

    const rows = await readTcpTables(include6);
    const inodeToRow = new Map<number, TcpRow>(rows.map((r) => [r.inode, r]));
    const listenRows = rows.filter(isListen);
    const listenInodes = new Set(listenRows.map((r) => r.inode));

    let pidList: number[];
    if (useAll) {
        const mapping = await pidsOwningInodes(listenInodes, verbose);
        pidList = [...mapping.keys()].sort((a, b) => a - b);
    } else {
        pidList = await collectPidsExact(argv);
        if (pidList.length === 0) die("Provide PIDs or names, or use --all.", EC.USAGE);
    }

    const portToPids = new Map<number, Set<number>>();

    const addPidPorts = (pid: number, inodes: Iterable<number>, filterListen: boolean) => {
        const ports = new Set<number>();

        for (const ino of inodes) {
            const row = inodeToRow.get(ino);
            if (!row) continue;
            if (filterListen && !isListen(row)) continue;
            ports.add(row.localPort);
        }

        for (const port of ports) {
            let set = portToPids.get(port);
            if (!set) {
                set = new Set<number>();
                portToPids.set(port, set);
            }
            set.add(pid);
        }
    };

    if (useAll) {
        const mapping = await pidsOwningInodes(listenInodes, verbose);
        for (const [pid, inodes] of mapping) {
            // TODO: in all mode we don't filter for isListen, is this a bug?
            addPidPorts(pid, inodes, /* filterListen */ false);
        }
    } else {
        for (const pid of pidList) {
            const pidInodes = await inodesForPid(pid);
            addPidPorts(pid, pidInodes, /* filterListen */ true);
        }
    }

    const conflicts = [...portToPids.entries()].filter(([, s]) => s.size > 1).sort((a, b) => a[0] - b[0]);

    if (conflicts.length === 0) {
        if (argv.json) {
            const out: JsonOut = { command: "check-conflicts", conflicts: [] };
            console.log(JSON.stringify(out));
        } else {
            console.log("No conflicts found.");
        }
        if (argv["exit-code"]) process.exit(EC.OK);
        process.exit(EC.OK);
    }

    // Resolve names for pretty output / JSON
    const allPids = new Set<number>();
    for (const [, s] of conflicts) for (const pid of s) allPids.add(pid);
    const nameMap = new Map<number, string>();
    for (const pid of allPids) nameMap.set(pid, await procName(pid));

    if (argv.json) {
        const out: JsonOut = {
            command: "check-conflicts",
            conflicts: conflicts.map(([port, set]) => ({
                port,
                pids: [...set].sort((a, b) => a - b),
                names: Object.fromEntries([...set].map((pid) => [pid, nameMap.get(pid) ?? "?"])),
            })),
        };
        console.log(JSON.stringify(out));
        if (argv["exit-code"]) process.exit(EC.NO_MATCH);
        process.exit(EC.OK);
    }

    console.log("Processes with conflicting listeners:");

    // Build rows: one entry per (port, pid), N = number of PIDs on that port
    const rowsOut: string[][] = [];
    for (const [port, set] of conflicts) {
        const n = set.size;
        for (const pid of [...set].sort((a, b) => a - b)) {
            rowsOut.push([String(port), String(pid), nameMap.get(pid) ?? "?", String(n)]);
        }
    }

    // Sort by PORT asc, then PID asc
    rowsOut.sort((a, b) => {
        const pa = Number(a[0]);
        const pb = Number(b[0]);
        if (pa !== pb) return pa - pb;
        return Number(a[1]) - Number(b[1]);
    });

    printTable(["PORT", "PID", "NAME", "N"], rowsOut);

    if (verbose) {
        for (const [port, set] of conflicts) {
            for (const pid of [...set].sort((a, b) => a - b)) {
                const pidInodes = await inodesForPid(pid);
                for (const ino of pidInodes) {
                    const row = inodeToRow.get(ino);
                    if (row && isListen(row) && row.localPort === port) {
                        console.error(`  ↳ ${pid} ${row.protocol} inode=${row.inode} port=${row.localPort}`);
                    }
                }
            }
        }
    }

    if (argv["exit-code"]) process.exit(EC.NO_MATCH);
}

// ---- probe-reuseport ----
async function cmdProbeReusePort(argv: ProbeReusePortArgvType) {
    ensureLinux();
    const port = argv.port;
    if (!port) die("Invalid --port.", EC.USAGE);

    const explicitHost = typeof argv.host === "string" && argv.host.trim() ? String(argv.host).trim() : undefined;
    const useIPv6 = argv.ipv6 || (explicitHost ? explicitHost.includes(":") : false);
    const host = explicitHost ?? (useIPv6 ? "::" : "0.0.0.0");
    const ipv6Only = argv.ipv6only;
    const holdMs = argv.timeout ?? 400; // keep the socket briefly
    const verbose = argv.verbose;
    const supports = nodeSupportsReusePort();

    // Check existing listeners on this port (so we can confirm sharing works)
    const rows = await readTcpTables(useIPv6);
    const beforeInodes = new Set(rows.filter((r) => isListen(r) && r.localPort === port).map((r) => r.inode));
    const beforePidMap = await pidsOwningInodes(beforeInodes, false);
    const before: { pid: number; name: string }[] = [];
    for (const pid of [...beforePidMap.keys()].sort((a, b) => a - b)) {
        before.push({ pid, name: await procName(pid) });
    }

    if (!supports) {
        console.error(
            `warning: Node ${process.versions.node} may not support server.listen({reusePort}). Proceeding anyway (may behave as disabled).`
        );
    }

    let bound = false;
    let bindErr: any;
    const server = net.createServer({ allowHalfOpen: false });

    await new Promise<void>((resolve) => {
        server.once("error", (e) => {
            bindErr = e;
            resolve();
        });
        try {
            const listenOpts: any = { port, host, reusePort: true, ipv6Only };
            server.listen(listenOpts, () => {
                bound = true;
                resolve();
            });
        } catch (e: any) {
            bindErr = e;
            resolve();
        }
    });

    if (bound && holdMs > 0) await sleep(holdMs);
    if (bound) {
        try {
            server.close();
        } catch {}
    }

    if (argv.json) {
        const out: JsonOut = {
            command: "probe-reuseport",
            port,
            host,
            node: process.versions.node,
            before,
            bound,
            error: bindErr ? { code: bindErr.code, message: String(bindErr.message ?? bindErr) } : undefined,
        };
        console.log(JSON.stringify(out));
        process.exit(bound ? EC.OK : EC.NO_MATCH);
    }

    if (bound) {
        if (before.length > 0) {
            console.log(
                `Success: bound ${host}:${port} with SO_REUSEPORT while ${before.length} listener(s) already existed:`
            );
            printTable(
                ["PID", "NAME"],
                before.map((p) => [String(p.pid), p.name])
            );
        } else {
            console.log(`Success: bound ${host}:${port} with SO_REUSEPORT (no pre-existing listeners detected).`);
        }
        if (verbose) {
            console.error(`  node=${process.versions.node} ipv6Only=${ipv6Only} host=${host}`);
        }
        process.exit(EC.OK);
    } else {
        console.error(
            `Failed to bind ${host}:${port} with SO_REUSEPORT: ${bindErr?.code ?? ""} ${bindErr?.message ?? bindErr}`
        );
        if (!supports) {
            console.error(`Hint: your Node version may not support 'reusePort'.`);
        }
        if (argv["exit-code"]) process.exit(EC.NO_MATCH);
        process.exit(EC.OSERR);
    }
}

// ---- connections-between ----
// Given a set of PIDs (or names), find all direct TCP/IPv4 connections among them.
async function cmdConnectionsBetween(argv: ConnectionsBetweenArgvType) {
    ensureLinux();

    const pidList = await collectPidsExact(argv);
    if (pidList.length < 2) die("Provide at least two PIDs or names.", EC.USAGE);

    const nodesMeta = new Map<
        number,
        { name: string; netns: string; listeners_v4: Listener[]; conns_v4: ConnEntry[] }
    >();

    for (const pid of pidList) {
        try {
            await fs.stat(`/proc/${pid}`);
        } catch {
            die(`PID ${pid} not found (no /proc/${pid}).`, EC.USAGE);
        }
        let listeners_v4: Listener[] = [];
        let conns_v4: ConnEntry[] = [];
        try {
            listeners_v4 = await listenersForPidV4(pid);
            conns_v4 = await connsForPidV4(pid);
        } catch (e: any) {
            if (e?.code === "EACCES" || e?.code === "EPERM") {
                die(`Permission denied reading sockets for PID ${pid}. Try elevated privileges.`, EC.NOPERM);
            }
            die(`Error reading sockets for PID ${pid}: ${String(e?.message ?? e)}`, EC.OSERR);
        }
        nodesMeta.set(pid, {
            name: await procName(pid),
            netns: await readNetNs(pid),
            listeners_v4,
            conns_v4,
        });
    }

    // Index by raw-hex tuple; this mirrors the Python behavior precisely.
    type Key = string;
    const keyHex = (e: ConnEntry): Key => `${e.localHex}|${e.localPort}|${e.remHex}|${e.remPort}`;
    const revKeyHex = (e: ConnEntry): Key => `${e.remHex}|${e.remPort}|${e.localHex}|${e.localPort}`;

    const index = new Map<Key, Array<{ pid: number; e: ConnEntry }>>();

    for (const pid of pidList) {
        const meta = nodesMeta.get(pid);
        if (!meta) continue;

        for (const e of meta.conns_v4) {
            const k = keyHex(e);
            let arr = index.get(k);
            if (!arr) {
                arr = [];
                index.set(k, arr);
            }
            arr.push({ pid, e });
        }
    }

    const edges: Array<{ aPid: number; bPid: number; a: ConnEntry; b: ConnEntry }> = [];
    const dedupe = new Set<string>(); // minInode|maxInode

    for (const aPid of pidList) {
        const metaA = nodesMeta.get(aPid);
        if (!metaA) continue;

        for (const a of metaA.conns_v4) {
            const rk = revKeyHex(a);
            const candidates = (index.get(rk) ?? []).filter((x) => x.pid !== aPid);
            if (candidates.length === 0) continue;

            // Prefer ESTABLISHED<->ESTABLISHED if present
            const best = candidates.find((c) => a.stateHex === "01" && c.e.stateHex === "01") ?? candidates[0];

            const bPid = best.pid;
            const b = best.e;

            const idA = Math.min(a.inode, b.inode);
            const idB = Math.max(a.inode, b.inode);
            const dKey = `${idA}|${idB}`;
            if (dedupe.has(dKey)) continue;
            dedupe.add(dKey);

            edges.push({ aPid, bPid, a, b });
        }
    }

    if (argv.json) {
        const out: JsonOut = {
            command: "connections-between",
            nodes: pidList.map((pid) => {
                const m = nodesMeta.get(pid);
                return {
                    pid,
                    name: m?.name ?? "unknown",
                    netns: m?.netns ?? null, // use a neutral fallback if your type allows it
                    listeners_v4: m?.listeners_v4 ?? [], // empty list if missing
                };
            }),
            edges,
        };
        console.log(JSON.stringify(out));
        process.exit(EC.OK);
    }

    // Pretty print
    for (const pid of pidList) {
        const m = nodesMeta.get(pid);
        if (!m) {
            console.log(`PID ${pid}: unknown  netns=?`);
            console.log("  Listening (IPv4): none");
            console.log();
            continue;
        }

        console.log(`PID ${pid}: ${m.name}  netns=${m.netns}`);
        if (!m.listeners_v4 || m.listeners_v4.length === 0) {
            console.log("  Listening (IPv4): none");
        } else {
            console.log("  Listening (IPv4):");
            for (const { ip, port } of m.listeners_v4) console.log(`    - ${ip}:${port}`);
        }
        console.log();
    }

    console.log("Direct TCP/IPv4 connections among provided PIDs:");
    if (edges.length === 0) {
        console.log("  none found");

        // Build a set of namespaces from the PIDs we *do* have metadata for
        const nsSet = new Set<number | string>();
        for (const pid of pidList) {
            const meta = nodesMeta.get(pid);
            if (meta && meta.netns !== undefined && meta.netns !== null) {
                nsSet.add(meta.netns);
            }
        }

        if (nsSet.size > 1) {
            console.log("\nNote: Some PIDs are in different network namespaces;");
            console.log("      they typically cannot connect directly unless bridged.");
        }
        process.exit(EC.OK);
    } else {
        let i = 1;
        for (const { aPid, bPid, a, b } of edges) {
            const metaA = nodesMeta.get(aPid);
            const metaB = nodesMeta.get(bPid);
            const nameA = metaA?.name ?? "unknown";
            const nameB = metaB?.name ?? "unknown";

            console.log(
                `  [${i}] ${aPid}(${nameA}) ${a.localIp}:${a.localPort}  <--${a.state}/${b.state}-->  ${a.remIp}:${a.remPort}  ${bPid}(${nameB})`
            );
            console.log(`       (A inode=${a.inode}, B inode=${b.inode})`);
            i++;
        }
        process.exit(EC.OK);
    }
}

// ---------- CLI ----------
yargs(hideBin(process.argv))
    .scriptName("net-ports")
    .usage("Usage: $0 <command> [options]")
    .version("2.2.0")
    .parserConfiguration({ "camel-case-expansion": true, "dot-notation": false })
    .fail((msg, err, y) => {
        if (err) {
            console.error("error:", err.message || String(err));
            process.exit(EC.OSERR);
        }
        if (msg) {
            console.error("error:", msg);
            console.error();
            console.error(y.help());
            process.exit(EC.USAGE);
        }
    })
    .option("json", { type: "boolean", default: false, describe: "Output machine-readable JSON." })
    .epilog(
        `Examples:
  net-ports by-port -p 30335
  net-ports by-port -p 30335 --json
  net-ports by-pid --names "tanssi-node,tanssi-relay"
  net-ports by-pid --name tanssi-node
  net-ports check-conflicts --all
  net-ports check-conflicts --pids "3333528 3333375 3333215" --exit-code`
    )
    .command(
        "by-port",
        "List processes listening on a TCP port",
        (y) =>
            y
                .example("$0 by-port -p 30335", "Human-friendly table")
                .example("$0 by-port -p 30335 --json", "JSON for scripting")
                .options(byPortOptions),
        async (argv) => {
            try {
                await cmdByPort(argv);
            } catch (e) {
                handleTopLevelError(e);
            }
        }
    )
    .command(
        "by-pid",
        "List listening TCP ports for one or more PIDs (or names)",
        (y) =>
            y
                .example("$0 by-pid --names 'tanssi-node,tanssi-relay'", "Exact comm matches only")
                .example("$0 by-pid --name tanssi-node", "Exact comm match (case-sensitive)")
                .options(byPidOptions),
        async (argv) => {
            try {
                await cmdByPid(argv);
            } catch (e) {
                handleTopLevelError(e);
            }
        }
    )
    .command(
        "check-conflicts",
        "Print TCP ports that are LISTENed by >1 PID in the set",
        (y) =>
            y
                .example("$0 check-conflicts --all", "Scan all running processes")
                .example("$0 check-conflicts --pids '111,222,333'", "Limit to a specific set of PIDs")
                .example("$0 check-conflicts -n tanssi-node -n tanssi-relay", "Exact comm matches")
                .options(checkConflictsOptions),
        async (argv) => {
            try {
                await cmdCheckConflicts(argv);
            } catch (e) {
                handleTopLevelError(e);
            }
        }
    )
    .command(
        "probe-reuseport",
        "Try to bind a TCP port with SO_REUSEPORT (Linux). Exits 0 on success.",
        (y) =>
            y
                .example("$0 probe-reuseport -p 30335", "Attempt to share port 30335 on 0.0.0.0")
                .example("$0 probe-reuseport -p 30335 --host :: --ipv6only", "IPv6-only probe on ::")
                .options(probeReusePortOptions),
        async (argv) => {
            try {
                await cmdProbeReusePort(argv);
            } catch (e) {
                handleTopLevelError(e);
            }
        }
    )
    .command(
        "connections-between",
        "Find direct TCP/IPv4 connections among the given PIDs (or exact names via --names/--name).",
        (y) =>
            y
                .example("$0 connections-between --pids '111,222'", "Compare two specific PIDs")
                .example(
                    "$0 connections-between --names 'tanssi-node,tanssi-relay'",
                    "Match exact process names (comm)"
                )
                .options(connectionsBetweenOptions),
        async (argv) => {
            try {
                await cmdConnectionsBetween(argv);
            } catch (e) {
                handleTopLevelError(e);
            }
        }
    )
    .demandCommand(1)
    .strict()
    .help()
    .wrap(Math.min(100, process.stdout.columns || 100))
    .parse();

// ---------- shared error handler ----------
function handleTopLevelError(e: any) {
    if (e && /EACCES|EPERM/i.test(String(e.code ?? ""))) {
        die("Permission denied while reading /proc. Try elevated privileges.", EC.NOPERM);
    }
    die(e?.message ?? String(e), EC.OSERR);
}
