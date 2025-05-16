#!/usr/bin/env python3
import subprocess
import re
import os
import sys

HELP_MSG = """
Usage: show-me-the-weights.py [OPTIONS]

Scan for heavy `Weight::from_parts(...)` calls in the codebase.

Positional arguments:
  diff          Only inspect added lines in `git diff origin/master`
                (ignores deletions, and unchanged lines).

Options:
  -h, --help    Show this help message and exit.

Description:
  • Without any arguments, runs `ripgrep` on the repository, then lists the top 10 calls by
    ref-time and proof-size, the enclosing function name and emojis according to severity.

  • With `diff` arg, runs `git diff origin/master`, filters to only newly added lines and prints
    problematic values. In case of no problems nothing is printed.

Examples:
  # Full scan of repository:
  ./show-me-the-weights.py

  # Only check newly added lines against origin/master:
  ./show-me-the-weights.py diff
"""

# Max block weights from
# polkadot/runtime/common/src/lib.rs
WEIGHT_REF_TIME_PER_SECOND = 1_000_000_000_000
MAX_REF_TIME = 2 * 1_000_000_000_000 # 2 seconds
MAX_PROOF_SIZE = 5_242_880 # around 5MB

def find_function_name(filepath, lineno):
    # Read file and search backwards from lineno-1 for the nearest fn declaration
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
    except OSError:
        return '<unknown>'
    # zero-based index for line number
    idx = lineno - 1
    fn_pattern = re.compile(r'^.*\s*fn\s+([A-Za-z0-9_]+)')
    for i in range(idx, -1, -1):
        match = fn_pattern.match(lines[i])
        if match:
            return match.group(1)
    return '<unknown>'

def format_number_with_underscores(n):
    # Use Python's formatting to insert underscores every three digits
    return format(n, '_')

def get_emoji(size, limit):
    if size < 0.1 * limit:
        emoji = "✅ "
    elif size < 0.5 * limit:
        emoji = "⚠️  "
    else:
        emoji = "🚨 "
    return emoji

def diff_mode(git_ref):
    """
    Apply regex to added lines in git diff origin/master.
    Outputs each problematic match as: {emoji}{matching_line}.
    If all matches are green, outputs nothing.
    """
    try:
        # -U0 to show only the changed lines, no context
        result = subprocess.run(
            ['git', 'diff', '-U0', git_ref],
            check=True,
            capture_output=True,
            text=True
        )
    except subprocess.CalledProcessError as e:
        print(f"Error running git diff: {e}: (exit {e.returncode})", file=sys.stderr)
        if e.stdout:
            print(e.stdout, file=sys.stderr)
        if e.stderr:
            print(e.stderr, file=sys.stderr)
        sys.exit(e.returncode)
    diff_lines = result.stdout.splitlines()
    pattern = re.compile(r'Weight::from_parts\(\s*([0-9_,]+)\s*,\s*([0-9_,]+)\s*\)')

    any_issues = False
    for line in diff_lines:
        # Only consider added lines
        if not line.startswith('+'):
            continue
        m = pattern.search(line)
        if not m:
            continue
        arg1 = int(m.group(1).replace('_', ''))
        arg2 = int(m.group(2).replace('_', ''))
        emoji_ref = get_emoji(arg1, MAX_REF_TIME)
        emoji_proof = get_emoji(arg2, MAX_PROOF_SIZE)
        # Use worst severity
        if emoji_ref == "🚨 " or emoji_proof == "🚨 ":
            emoji = "🚨 "
        elif emoji_ref == "⚠️  " or emoji_proof == "⚠️  ":
            emoji = "⚠️  "
        else:
            emoji = "✅ "
        # Only record non-green issues
        if emoji != "✅ ":
            if any_issues == False:
                # We rely on the keyword "Found problematic" in the benchmarking schedule run
                print("🚨 Found problematic weights in PR diff. Check them to ensure all extrinsics can still be called.")
            any_issues = True
            # Strip leading '+' for clarity
            print(f'{line[1:]}')

def ripgrep_mode():
    """
    Apply regex to all files in repo using ripgrep
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))
    parent_dir = os.path.dirname(script_dir)
    try:
        result = subprocess.run([
            'rg', '-Hno', r'Weight::from_parts\([0-9_,]+\s*,\s*[0-9_,]+\)', parent_dir
        ], check=True, capture_output=True, text=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running ripgrep: {e}")
        return

    pattern = re.compile(
        r'^(.*?):(\d+):Weight::from_parts\(\s*([0-9_,]+)\s*,\s*([0-9_,]+)\s*\)'
    )
    matches = []
    for line in result.stdout.splitlines():
        m = pattern.match(line)
        if not m:
            continue
        filepath, lineno, arg1_str, arg2_str = m.groups()
        arg1 = int(arg1_str.replace('_', ''))
        arg2 = int(arg2_str.replace('_', ''))
        matches.append({
            'file': filepath,
            'line': int(lineno),
            'arg1': arg1,
            'arg2': arg2,
            'arg1_str': arg1_str,
            'arg2_str': arg2_str,
            'text': line
        })

    if not matches:
        print("No matches found.")
        return

    # Print top 10 by ref time
    print("Top calls by ref time:")
    for i, m in enumerate(sorted(matches, key=lambda x: x['arg1'], reverse=True)):
        print(m['text'])
        fn_name = find_function_name(m['file'], m['line'])
        formatted = format_number_with_underscores(m['arg1'])
        emoji = get_emoji(m['arg1'], MAX_REF_TIME)
        print(f"{emoji}{formatted:>27}: {fn_name}")
        if i+2 > 10 and m['arg1'] < MAX_REF_TIME:
            break

    print("")
    # Print top 10 by proof size
    print("Top calls by proof size:")
    for i, m in enumerate(sorted(matches, key=lambda x: x['arg2'], reverse=True)):
        print(m['text'])
        fn_name = find_function_name(m['file'], m['line'])
        formatted = format_number_with_underscores(m['arg2'])
        emoji = get_emoji(m['arg2'], MAX_PROOF_SIZE)
        print(f"{emoji}{formatted:>27}: {fn_name}")
        if i+2 > 10 and m['arg2'] < MAX_PROOF_SIZE:
            break

def main():
    #
    if len(sys.argv) > 1 and sys.argv[1] in ['--help', '-h']:
        print(HELP_MSG)
        return

    # If called with "diff", use diff mode
    if len(sys.argv) > 1 and sys.argv[1] == 'diff':
        ref = "origin/master"
        if len(sys.argv) > 2:
            ref = sys.argv[2]
        diff_mode(ref)
        return

    ripgrep_mode()

if __name__ == '__main__':
    main()
