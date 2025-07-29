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

type Flags = {
  overridePolkadotArgs: string[];
  changeRelayKeystore: boolean;
  remainArgs: string[];
};

type Sections = {
  tanssiArgs: string[];
  polkadotArgs: string[];
};

function printUsage(): never {
  console.error(`
Usage: proxy [--set-relay-arg=<arg>] [--change-relay-keystore-path] \\
  -- <command> <tanssi_args>... -- <polkadot_args>...

  --set-relay-arg=--KEY=VAL        Append or override --KEY=VAL in the relay args
  --change-relay-keystore-path     Override the relay-chain’s keystore path
`);
  process.exit(1);
}

// Parse script args until the first '--'
// These are the args for the script, before any tanssi-node args
// Returns remaining args after the first '--' in `remainArgs`
function parseScriptFlags(argv: string[]): Flags {
  const overridePolkadotArgs: string[] = [];
  let changeRelayKeystore = false;
  let idx = 0;

  while (idx < argv.length && argv[idx] !== '--') {
    const arg = argv[idx++];
    if (arg.startsWith('--set-relay-arg=')) {
      overridePolkadotArgs.push(arg.slice('--set-relay-arg='.length));
    } else if (arg === '--change-relay-keystore-path') {
      changeRelayKeystore = true;
    } else {
      console.error(`Error: unknown flag ${arg}`);
      printUsage();
    }
  }

  if (idx >= argv.length || argv[idx] !== '--') {
    printUsage();
  }

  return {
    overridePolkadotArgs,
    changeRelayKeystore,
    remainArgs: argv.slice(idx + 1),
  };
}

// Parse args into tanssi args or polkadot args by finding the first `--`
function splitCommandSections(remainArgs: string[]): Sections {
  if (remainArgs.length === 0) {
    console.error('Error: missing command to execute');
    printUsage();
  }

  const [cmd, ...rest] = remainArgs;
  const tanssiArgs: string[] = [cmd];
  let splitIndex = rest.indexOf('--');
  if (splitIndex < 0) splitIndex = rest.length;

  tanssiArgs.push(...rest.slice(0, splitIndex));
  const polkadotArgs = rest.slice(splitIndex + (splitIndex < rest.length ? 1 : 0));

  return { tanssiArgs, polkadotArgs };
}

/**
 * Retrieves the value for a key in either "--key value" or "--key=value" form.
 * @throws if the key (in either form) is not found.
 */
function getArgValue(key: string, arr: string[]): string {
  for (let i = 0; i < arr.length; i++) {
    const el = arr[i];
    // form: --key value
    if (el === key) {
      if (i + 1 < arr.length) {
        return arr[i + 1];
      } else {
        throw new Error(`missing value for relay ${key}`);
      }
    }
    // form: --key=value
    if (el.startsWith(`${key}=`)) {
      return el.slice(key.length + 1);
    }
  }
  throw new Error(`missing relay ${key}`);
}

// Compute a keystore path that depends on base path, for testing
function computeRelayKeystorePath(polkadotArgs: string[]): string {
  const base = getArgValue('--base-path', polkadotArgs);
  return `${base}/tmp_keystore_zombie_test`;
}

// Override args in place.
// For each override in the form `--key=value`, looks for the first existing occurrence
// (either --key, and the value is the next element, or --key=value) and replaces it in‑place.
//
// If nothing was found, push the new `--key=value` at the end.
function overrideArgs(
    arr: string[],
    overrides: readonly string[]
) {
  for (const entry of overrides) {
    const [key] = entry.split("=", 1);
    let replaced = false;

    for (let i = 0; i < arr.length; i++) {
      const elem = arr[i];

      if (elem === key) {
        // bare key followed by its value → replace both with the single "key=val"
        arr.splice(i, 2, entry);
        replaced = true;
        break;
      }
      else if (elem.startsWith(`${key}=`)) {
        // inline "key=val" form → just overwrite
        arr[i] = entry;
        replaced = true;
        break;
      }
    }

    if (!replaced) {
      // no existing arg → append at end
      arr.push(entry);
    }
  }
}

// Override `--keystore-path` arg with new value
function overrideKeystore(
    polkadotArgs: string[],
    keystorePath: string
) {
  overrideArgs(polkadotArgs, [`--keystore-path=${keystorePath}`]);
}

function debugPrint(state: {
  overridePolkadotArgs: string[];
  tanssiArgs: string[];
  polkadotArgs: string[];
  changeRelayKeystore: boolean;
}) {
  console.error('DEBUG:');
  console.error('  overridePolkadotArgs:', state.overridePolkadotArgs);
  console.error('  tanssiArgs:       ', state.tanssiArgs);
  console.error('  polkadotArgs:     ', state.polkadotArgs);
  console.error('  changeRelayKeystore:', state.changeRelayKeystore);
  console.error('');
}

async function main() {
  try {
    // 1) parse script flags
    const {
      overridePolkadotArgs,
      changeRelayKeystore,
      remainArgs,
    } = parseScriptFlags(process.argv.slice(2));

    // 2) split into sections
    let { tanssiArgs, polkadotArgs } = splitCommandSections(remainArgs);

    debugPrint({ overridePolkadotArgs, tanssiArgs, polkadotArgs, changeRelayKeystore });

    // 3) optional keystore override
    if (changeRelayKeystore) {
      const ks = computeRelayKeystorePath(polkadotArgs);
      overrideKeystore(polkadotArgs, ks);
    }

    // 4) apply any remaining --set-relay-arg overrides
    overrideArgs(polkadotArgs, overridePolkadotArgs);

    debugPrint({ overridePolkadotArgs, tanssiArgs, polkadotArgs, changeRelayKeystore });

    // 5) exec
    const cmd = tanssiArgs[0];
    const args = [...tanssiArgs.slice(1), '--', ...polkadotArgs];
    const proc = spawn(cmd, args, { stdio: 'inherit' });
    // Forward shutdown signals to child process.
    // Fixes bug of tanssi-node processes being alive after running `pnpm moonwall test zombie_tanssi`
    for (const signal of ["SIGINT", "SIGTERM"]) {
      process.on(signal, () => {
        console.log("zombienetRestart: got ", signal);
        if (proc) {
          proc.kill(signal as NodeJS.Signals);
        }
        process.exit();
      });
    }

    proc.on('exit', (code, signal) => {
      process.exit(signal ? 1 : code ?? 1);
    });
  } catch (err) {
    console.error('Error:', (err as Error).message);
    process.exit(1);
  }
}

main();
