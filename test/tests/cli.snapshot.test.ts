import { describe, it, expect } from 'vitest'
import { execa } from 'execa'
import stripAnsi from 'strip-ansi'
import path from 'node:path'
import fs from 'node:fs/promises'
import os from 'node:os'

const BIN = path.resolve(process.cwd(), '../target/release/tanssi-relay')

function scrub(text: string) {
  const root = process.cwd().replace(/[-/\\^$*+?.()|[\]{}]/g, '\\$&')
  return stripAnsi(text)
    .replace(/\r\n/g, '\n')                            // normalize EOL
    .replace(new RegExp(root, 'g'), '<ROOT>')          // hide absolute paths
    .replace(/\b\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z\b/g, '<ISO>')
    .replace(/\b\d+(?:\.\d+)?ms\b/g, '<MS>')
    .replace(/\b0x[0-9a-fA-F]{8,}\b/g, '<HEX>')
}

async function run(args: string[], opts: { input?: string; env?: Record<string, string> } = {}) {
  const res = await execa(BIN, args, {
    env: { NO_COLOR: '1', CLICOLOR: '0', ...opts.env },
    reject: false,                // don't throw on non-zero exit
    timeout: 60_000,
    input: opts.input,
    all: true,                    // combined output in res.all
  })
  return {
    code: res.exitCode,
    stdout: scrub(res.stdout ?? ''),
    stderr: scrub(res.stderr ?? ''),
    all: scrub(res.all ?? ''),
  }
}

describe('tanssi-relay CLI', () => {
  it('shows help', async () => {
    const { code, all } = await run(['--help'])
    expect(code).toBe(0)
    // File snapshot keeps a pretty .txt file you can read in PRs
    await expect(all).toMatchFileSnapshot('snapshots/help.txt')
  })

  it('shows version', async () => {
    const { code, all } = await run(['--version'])
    expect(code).toBe(0)
    // Inline snapshot is handy for short strings
    expect(all).toMatchInlineSnapshot(`
      "tanssi 0.16.0-dev
      "
    `)
  })

  it('unknown flag produces an error', async () => {
    const { code, all } = await run(['--definitely-not-a-flag'])
    expect(code).not.toBe(0)
    await expect(all).toMatchFileSnapshot('snapshots/unknown-flag.txt')
  })

  it('unknown value produces an error', async () => {
    const { code, all } = await run(['a'])
    expect(code).not.toBe(0)
    await expect(all).toMatchFileSnapshot('snapshots/unknown-value.txt')
  })
  it('unknown value number produces an error', async () => {
    const { code, all } = await run(['1'])
    expect(code).not.toBe(0)
    await expect(all).toMatchFileSnapshot('snapshots/unknown-value-number.txt')
  })

    it('prints error message when node key cannot be parsed', async () => {
      const tmpBase = await fs.mkdtemp(path.join(os.tmpdir(), 'tanssi-relay-test-'))
      const keyPath = path.join(tmpBase, 'node-key')

      try {
        // Write an intentionally invalid Ed25519 secret key
        await fs.writeFile(keyPath, 'not-a-valid-ed25519-key', 'utf8')

        const { code, all } = await run([
          '--chain=tanssi',
          '--tmp',
          '--validator',
          `--node-key-file=${keyPath}`,
        ])

        expect(code).not.toBe(0)

        // Validate the specific error message
        expect(all).toMatch(/failed to parse ed25519 secret key/i)

        // Redact the temp path so the snapshot is stable across runs
        const redacted = all.replace(
          new RegExp(tmpBase.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'g'),
          '<TMP>'
        )

        // Store the whole output for future diffs
        await expect(redacted).toMatchFileSnapshot('snapshots/bad-node-key.txt')
      } finally {
        // Always clean up
        await fs.rm(tmpBase, { recursive: true, force: true })
      }
    }, 30_000)
})

