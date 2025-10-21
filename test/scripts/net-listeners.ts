// net-listeners.ts
// Node >=18 recommended (fs/promises, stable ESM). Linux-only (/proc).
/*
README
======

A tiny Unix-y CLI to inspect TCP listeners from /proc, to find port conflicts for litep2p.

SO_REUSEPORT allows multiple processes to listen on the same TCP port. This utility is to check if that's happening in
zombienet tests, because if 2 processes are listening on the same p2p port, they won't be able to connect to each other.

USAGE (quick):
  pnpm net-listeners check-conflicts --all
  pnpm net-listeners by-pid --names "tanssi-node,tanssi-relay,polkadot"
  pnpm net-listeners by-port -p 30335
  pnpm net-listeners probe-reuseport -p 30335

NAME MATCHING:
  --names / --name → exact match on process *name* (i.e., /proc/<pid>/comm). No grep, no args. Case-sensitive.

EXAMPLES:
  net-listeners by-port -p 30335
  net-listeners by-port -p 30335 --json
  net-listeners by-pid --names "tanssi-node,tanssi-relay"
  net-listeners by-pid --name tanssi-node
  net-listeners check-conflicts --all
  net-listeners check-conflicts --pids "123,456" --exit-code
  net-listeners probe-reuseport -p 30335
*/

import fs from "node:fs/promises";
import path from "node:path";
import net from "node:net";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";

/** A single row parsed from /proc/net/tcp* */
type TcpRow = {
    inode: number;
    localPort: number;
    protocol: "tcp4" | "tcp6";
    stateHex: string; // "0A" = LISTEN
    localHex: string; // hex address (kept for future)
};

type JsonOut =
    | { command: "by-port"; port: number; listeners: { pid: number; name: string }[] }
    | { command: "by-pid"; entries: { pid: number; name: string; ports: number[] }[] }
    | { command: "check-conflicts"; conflicts: { port: number; pids: number[]; names: Record<number, string> }[] };

/** Standardized exit codes for nice shell ergonomics. */
const EC = {
    OK: 0,
    NO_MATCH: 1, // used with --exit-code on conflicts (diff-like)
    USAGE: 64, // EX_USAGE
    NOPERM: 77, // EX_NOPERM
    OSERR: 71, // EX_OSERR
    PLATFORM: 78, // wrong OS (/proc missing)
} as const;

const LISTEN_HEX = "0A";
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// ---------- small utils (no color) ----------
const pad = (s: string, w: number) => (s.length >= w ? s : s + " ".repeat(w - s.length));
const asBool = (x: unknown, def = false) => (typeof x === "boolean" ? x : def);
const toNum = (x: unknown) =>
    typeof x === "number" && Number.isFinite(x) ? x : typeof x === "string" && /^\d+$/.test(x) ? Number(x) : undefined;

const nameCache = new Map<number, string>();

/** Print error and exit. */
function die(msg: string, code = EC.USAGE) {
    console.error("error:", msg);
    process.exit(code);
}

/** Print warning to stderr (non-fatal). */
function warn(msg: string) {
    console.error("warning:", msg);
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

/** Parse /proc/net/tcp or /proc/net/tcp6 into structured rows. */
async function parseProcNetTcp(file: string, protocol: "tcp4" | "tcp6"): Promise<TcpRow[]> {
    let txt = "";
    try {
        txt = await fs.readFile(file, "utf8");
    } catch {
        // File can be missing (restricted containers). Be permissive.
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
        const stateHex = cols[3];
        const inodeStr = cols[9];

        const colonIdx = local.lastIndexOf(":");
        if (colonIdx < 0) continue;
        const portHex = local.slice(colonIdx + 1);
        const port = Number.parseInt(portHex, 16);
        const inode = Number.parseInt(inodeStr, 10);
        if (!Number.isFinite(port) || !Number.isFinite(inode)) continue;

        out.push({
            inode,
            localPort: port,
            protocol,
            stateHex,
            localHex: local.slice(0, colonIdx),
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
        warn(`cannot read /proc: ${String(e?.message ?? e)}`);
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
    try {
        const entries = await fs.readdir(fdDir, { withFileTypes: true });
        fds = entries.map((e) => e.name);
    } catch {
        return result;
    }
    const reads = await Promise.allSettled(fds.map((fd) => fs.readlink(path.join(fdDir, fd))));
    for (const r of reads) {
        if (r.status === "fulfilled") {
            const m = /^socket:\[(\d+)\]$/.exec(r.value.toString());
            if (m) result.add(Number(m[1]));
        }
    }
    return result;
}

// ---------- PID/name collection (EXACT by default) ----------
/** Normalize yargs values into string tokens. */
function tokensFromYargs(x: unknown): string[] {
    if (Array.isArray(x)) return x.map(String);
    if (x == null) return [];
    return [String(x)];
}

/** Split on comma/whitespace and trim. */
function explodeToken(t: string): string[] {
    return t
        .split(/[,\s]+/)
        .map((s) => s.trim())
        .filter(Boolean);
}

/**
 * Collect a de-duplicated, sorted PID list from:
 *   --pid/--pids (numbers)
 *   --names / --name      (EXACT comm matches only; case-sensitive)
 */
async function collectPidsExact(argv: any): Promise<number[]> {
    // numeric PIDs
    const pidTokens = [...tokensFromYargs(argv.pid ?? argv.Pid), ...tokensFromYargs(argv.pids ?? argv.Pids)]
        .flatMap(explodeToken)
        .filter((t) => /^\d+$/.test(t))
        .map(Number);
    const pidSet = new Set<number>(pidTokens);

    // exact comm matches: --names and --name (both exact now)
    const namesExact = [...tokensFromYargs(argv.names ?? argv.Names), ...tokensFromYargs(argv.name ?? argv.Name)]
        .flatMap(explodeToken)
        .filter(Boolean);

    if (namesExact.length > 0) {
        const map = await pidsFromCommExact(namesExact);
        for (const pids of map.values()) for (const pid of pids) pidSet.add(pid);
    }

    return [...pidSet].sort((a, b) => a - b);
}

/** Resolve PIDs by *exact* match of /proc/<pid>/comm against each token. Case-sensitive. */
async function pidsFromCommExact(names: string[]): Promise<Map<string, number[]>> {
    const map = new Map<string, number[]>();
    for (const n of names) map.set(n, []);
    let procDirs: string[] = [];
    try {
        const entries = await fs.readdir("/proc", { withFileTypes: true });
        procDirs = entries.filter((d) => d.isDirectory()).map((d) => d.name);
    } catch {
        return map;
    }
    const pidDirs = procDirs.filter((name) => /^\d+$/.test(name));
    await Promise.all(
        pidDirs.map(async (pidStr) => {
            const pid = Number(pidStr);
            let comm = "";
            try {
                comm = (await fs.readFile(`/proc/${pid}/comm`, "utf8")).trim();
            } catch {
                return; // skip this pid if /proc/<pid>/comm isn't readable
            }
            if (!comm) return;

            for (const wanted of names) {
                if (comm === wanted) {
                    let list = map.get(wanted);
                    if (!list) {
                        list = [];
                        map.set(wanted, list);
                    }
                    list.push(pid);
                }
            }
        })
    );

    // sort PIDs for each name
    for (const [k, v] of map) {
        v.sort((a, b) => a - b);
    }

    return map;
}

// ---------- pretty printing ----------
/** Print a minimal, aligned table without hard dependencies. */
function printTable(headers: string[], rows: string[][]) {
    const widths = headers.map((h, idx) => Math.max(h.length, ...rows.map((r) => (r[idx] ? r[idx].length : 0))));
    console.log(headers.map((h, i) => pad(h, widths[i])).join("  "));
    console.log(widths.map((w) => "-".repeat(w)).join("  "));
    for (const r of rows) console.log(r.map((c, i) => pad(c, widths[i])).join("  "));
}

// ---------- command impls (logic moved out) ----------
async function cmdByPort(argv: any) {
    ensureLinux();
    const port = toNum(argv.port ?? argv.Port);
    if (!port) die("Invalid --port.", EC.USAGE);
    const include6 = asBool(argv.ipv6 ?? argv.IncludeIPv6);
    const verbose = asBool(argv.verbose ?? argv.Verbose);

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

async function cmdByPid(argv: any) {
    ensureLinux();
    const include6 = asBool(argv.ipv6 ?? argv.IncludeIPv6);
    const verbose = asBool(argv.verbose ?? argv.Verbose);

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

async function cmdCheckConflicts(argv: any) {
    ensureLinux();
    const include6 = asBool(argv.ipv6 ?? argv.IncludeIPv6);
    const verbose = asBool(argv.verbose ?? argv.Verbose);
    const useAll = asBool(argv.all);

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
    if (useAll) {
        const mapping = await pidsOwningInodes(listenInodes, verbose);
        for (const [pid, inodes] of mapping) {
            const ports = new Set<number>();

            for (const ino of inodes) {
                const row = inodeToRow.get(ino);
                if (row) ports.add(row.localPort);
            }

            for (const port of ports) {
                let set = portToPids.get(port);
                if (!set) {
                    set = new Set<number>();
                    portToPids.set(port, set);
                }
                set.add(pid);
            }
        }
    } else {
        for (const pid of pidList) {
            const pidInodes = await inodesForPid(pid);
            const ports = new Set<number>();

            for (const ino of pidInodes) {
                const row = inodeToRow.get(ino);
                if (row && isListen(row)) ports.add(row.localPort);
            }

            for (const port of ports) {
                let set = portToPids.get(port);
                if (!set) {
                    set = new Set<number>();
                    portToPids.set(port, set);
                }
                set.add(pid);
            }
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
async function cmdProbeReusePort(argv: any) {
    ensureLinux();
    const port = toNum(argv.port ?? argv.Port);
    if (!port) die("Invalid --port.", EC.USAGE);

    const explicitHost = typeof argv.host === "string" && argv.host.trim() ? String(argv.host).trim() : undefined;
    const useIPv6 = asBool(argv.ipv6 ?? argv.IncludeIPv6) || (explicitHost ? explicitHost.includes(":") : false);
    const host = explicitHost ?? (useIPv6 ? "::" : "0.0.0.0");
    const ipv6Only = asBool(argv.ipv6only ?? argv.ipv6Only ?? argv.IPv6Only);
    const holdMs = toNum(argv.timeout ?? argv.Timeout) ?? 400; // keep the socket briefly
    const verbose = asBool(argv.verbose ?? argv.Verbose);
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
        warn(
            `Node ${process.versions.node} may not support server.listen({reusePort}). Proceeding anyway (may behave as disabled).`
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

// ---------- CLI ----------
yargs(hideBin(process.argv))
    .scriptName("net-listeners")
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
  net-listeners by-port -p 30335
  net-listeners by-port -p 30335 --json
  net-listeners by-pid --names "tanssi-node,tanssi-relay"
  net-listeners by-pid --name tanssi-node
  net-listeners check-conflicts --all
  net-listeners check-conflicts --pids "3333528 3333375 3333215" --exit-code`
    )

    // ---- by-port ----
    .command(
        "by-port",
        "List processes listening on a TCP port",
        (y) =>
            y
                .example("$0 by-port -p 30335", "Human-friendly table")
                .example("$0 by-port -p 30335 --json", "JSON for scripting")
                .options({
                    port: {
                        type: "number",
                        alias: ["p", "Port"],
                        demandOption: true,
                        describe: "TCP port (e.g. 30335)",
                    },
                    ipv6: {
                        type: "boolean",
                        alias: ["6", "IncludeIPv6"],
                        default: false,
                        describe: "Also include IPv6 (tcp6)",
                    },
                    verbose: {
                        type: "boolean",
                        alias: ["v", "Verbose"],
                        default: false,
                        describe: "Verbose logs to stderr",
                    },
                }),
        (argv) => void cmdByPort(argv).catch((e: any) => handleTopLevelError(e))
    )

    // ---- by-pid ----
    .command(
        "by-pid",
        "List listening TCP ports for one or more PIDs (or names)",
        (y) =>
            y
                .example("$0 by-pid --names 'tanssi-node,tanssi-relay'", "Exact comm matches only")
                .example("$0 by-pid --name tanssi-node", "Exact comm match (case-sensitive)")
                .options({
                    pid: { type: "array", alias: ["p", "Pid"], describe: "Repeatable: --pid 123 --pid 456" },
                    pids: {
                        type: "array",
                        alias: ["Pids"],
                        describe: "Space/comma separated: --pids 1 2 3 or --pids '1,2,3'",
                    },
                    // IMPORTANT SEMANTICS: both are exact comm now
                    names: {
                        type: "array",
                        alias: ["Names"],
                        describe: "Exact process names (comm). No grep, no args.",
                    },
                    name: {
                        type: "array",
                        alias: ["n", "Name"],
                        describe: "Exact process names (comm). No grep, no args.",
                    },
                    ipv6: {
                        type: "boolean",
                        alias: ["6", "IncludeIPv6"],
                        default: false,
                        describe: "Also include IPv6 (tcp6)",
                    },
                    verbose: {
                        type: "boolean",
                        alias: ["v", "Verbose"],
                        default: false,
                        describe: "Verbose logs to stderr",
                    },
                }),
        (argv) => void cmdByPid(argv).catch((e: any) => handleTopLevelError(e))
    )

    // ---- check-conflicts ----
    .command(
        "check-conflicts",
        "Print TCP ports that are LISTENed by >1 PID in the set",
        (y) =>
            y
                .example("$0 check-conflicts --all", "Scan all running processes")
                .example("$0 check-conflicts --pids '111,222,333'", "Limit to a specific set of PIDs")
                .example("$0 check-conflicts -n tanssi-node -n tanssi-relay", "Exact comm matches")
                .options({
                    pid: { type: "array", alias: ["p", "Pid"], describe: "Repeatable: --pid 123 --pid 456" },
                    pids: {
                        type: "array",
                        alias: ["Pids"],
                        describe: "Space/comma separated: --pids 1 2 3 or --pids '1,2,3'",
                    },
                    names: {
                        type: "array",
                        alias: ["Names"],
                        describe: "Exact process names (comm). No grep, no args.",
                    },
                    name: {
                        type: "array",
                        alias: ["n", "Name"],
                        describe: "Exact process names (comm). No grep, no args.",
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
                        alias: ["v", "Verbose"],
                        default: false,
                        describe: "Verbose logs to stderr",
                    },
                    "exit-code": {
                        type: "boolean",
                        default: false,
                        describe: "Exit 1 if conflicts found (0 if none).",
                    },
                }),
        (argv) => void cmdCheckConflicts(argv).catch((e: any) => handleTopLevelError(e))
    )

    // ---- probe-reuseport ----
    .command(
        "probe-reuseport",
        "Try to bind a TCP port with SO_REUSEPORT (Linux). Exits 0 on success.",
        (y) =>
            y
                .example("$0 probe-reuseport -p 30335", "Attempt to share port 30335 on 0.0.0.0")
                .example("$0 probe-reuseport -p 30335 --host :: --ipv6only", "IPv6-only probe on ::")
                .options({
                    port: {
                        type: "number",
                        alias: ["p", "Port"],
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
                        alias: ["v", "Verbose"],
                        default: false,
                        describe: "Verbose logs to stderr",
                    },
                    "exit-code": { type: "boolean", default: false, describe: "Exit 1 if the bind fails." },
                }),
        (argv) => void cmdProbeReusePort(argv).catch((e: any) => handleTopLevelError(e))
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
