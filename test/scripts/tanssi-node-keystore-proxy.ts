/**
 * tanssi-node-keystore-proxy.ts
 *
 * Ensures container (Tanssi) args and relay-chain (Polkadot) args
 * are in the right order, and allows:
 *   • early flags like --set-relay-arg=<arg>
 *   • switching which keystore-path is overridden
 *   • duplicate relay-arg suppression
 */

import { spawn } from 'child_process';

//─── globals ────────────────────────────────────────────────────────────────────
let overridePolkadotArgs: string[] = [];
let tanssiArgs: string[] = [];
let polkadotArgs: string[] = [];
let changeRelayKeystore = false;
const relayChainId = 'dancelight_local_testnet';

//─── usage helper ────────────────────────────────────────────────────────────────
function printUsage(): never {
  console.error(`
Usage: proxy [--set-relay-arg=<arg>] [--change-relay-keystore-path] \\
  -- <command> <tanssi_args>... -- <polkadot_args>...

  --set-relay-arg=--KEY=VAL        Append or override --KEY=VAL in the relay args
  --change-relay-keystore-path     Override the relay-chain’s keystore path
`);
  process.exit(1);
}

//─── parse_script_flags ──────────────────────────────────────────────────────────
function parseScriptFlags(args: string[]): string[] {
  overridePolkadotArgs = [];
  changeRelayKeystore = false;

  let i = 0;
  while (i < args.length && args[i] !== '--') {
    const arg = args[i];
    if (arg.startsWith('--set-relay-arg=')) {
      overridePolkadotArgs.push(arg.slice('--set-relay-arg='.length));
    } else if (arg === '--change-relay-keystore-path') {
      changeRelayKeystore = true;
    } else {
      console.error(`Error: unknown flag ${arg}`);
      printUsage();
    }
    i++;
  }

  if (i >= args.length || args[i] !== '--') {
    printUsage();
  }
  // skip the `--`
  return args.slice(i + 1);
}

//─── split_command_sections ─────────────────────────────────────────────────────
function splitCommandSections(args: string[]): void {
  if (args.length === 0) {
    console.error('Error: missing command to execute');
    printUsage();
  }

  tanssiArgs = [args[0]];
  let idx = 1;
  while (idx < args.length && args[idx] !== '--') {
    tanssiArgs.push(args[idx]);
    idx++;
  }
  // skip the `--` if present
  if (idx < args.length && args[idx] === '--') idx++;
  polkadotArgs = args.slice(idx);
}

//─── get_arg_value ───────────────────────────────────────────────────────────────
function getArgValue(key: string, arr: string[]): string {
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] === key && i + 1 < arr.length) {
      return arr[i + 1];
    }
  }
  throw new Error(`missing relay ${key}`);
}

//─── compute_relay_keystore_path ─────────────────────────────────────────────────
function computeRelayKeystorePath(): string {
  const base = getArgValue('--base-path', polkadotArgs);
  return `${base}/tmp_keystore_zombie_test`;
}

//─── override_relay_args ─────────────────────────────────────────────────────────
function overrideRelayArgs(arr: string[]): void {
  if (overridePolkadotArgs.length === 0) {
    console.error('override_relay_args: nothing to do');
    return;
  }

  for (const overrideEntry of overridePolkadotArgs) {
    console.error(`--> processing override: '${overrideEntry}'`);
    const [key] = overrideEntry.split('=', 1);

    const tmp: string[] = [];
    let removed = false;

    for (let i = 0; i < arr.length; ) {
      const elem = arr[i];
      if (elem.startsWith(`${key}=`)) {
        removed = true;
        console.error(`    strip key=value form: '${elem}'`);
        i++;
      } else if (elem === key) {
        removed = true;
        const next = arr[i + 1] ?? '<MISSING>';
        console.error(`    strip bare key+value: '${elem}' + '${next}'`);
        i += 2;
      } else {
        tmp.push(elem);
        i++;
      }
    }

    if (!removed) {
      console.error(`    no existing '${key}' entries found`);
    }
    tmp.push(overrideEntry);
    console.error(`    after append, tmp = [${tmp.join(' ')}]`);
    // write back
    arr.length = 0;
    arr.push(...tmp);
    console.error(`    arr now = [${arr.join(' ')}]`);
  }
}

//─── override_keystore ───────────────────────────────────────────────────────────
function overrideKeystore(arr: string[], keystore: string): void {
  const old = [...overridePolkadotArgs];
  overridePolkadotArgs = [`--keystore-path=${keystore}`];
  try {
    overrideRelayArgs(arr);
  } catch (e) {
    console.error('Error: failed to apply keystore override');
    overridePolkadotArgs = old;
    process.exit(1);
  }
  overridePolkadotArgs = old;
}

//─── debug_print ─────────────────────────────────────────────────────────────────
function debugPrint(): void {
  console.error('DEBUG:');
  console.error('  overridePolkadotArgs:', overridePolkadotArgs);
  console.error('  tanssiArgs:       ', tanssiArgs);
  console.error('  polkadotArgs:     ', polkadotArgs);
  console.error('  changeRelayKeystore:', changeRelayKeystore);
  console.error('');
}

//─── main flow ─────────────────────────────────────────────────────────────────
async function main() {
  // 1) peel off our script-only flags
  const remain = parseScriptFlags(process.argv.slice(2));

  // 2) split into tanssi vs polkadot args
  splitCommandSections(remain);
  debugPrint();

  if (changeRelayKeystore) {
    const path = computeRelayKeystorePath();
    overrideKeystore(polkadotArgs, path);
  }

  overrideRelayArgs(polkadotArgs);
  debugPrint();

  // 3) exec the command
  const cmd = tanssiArgs[0];
  const args = [...tanssiArgs.slice(1), '--', ...polkadotArgs];
  const child = spawn(cmd, args, { stdio: 'inherit' });

  child.on('exit', (code, signal) => {
    if (signal) {
      process.exit(1);
    } else {
      process.exit(code ?? 1);
    }
  });
}

main().catch(err => {
  console.error('Unexpected error:', err);
  process.exit(1);
});
