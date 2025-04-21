#!/usr/bin/env python3
import subprocess
import re
import os

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


def main():
    # Run ripgrep to find all Weight::from_parts(arg1, arg2) occurrences
    try:
        result = subprocess.run([
            'rg', '-Hno', r'Weight::from_parts\([0-9_,]+\s*,\s*[0-9_,]+\)', '.'
        ], check=True, capture_output=True, text=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running ripgrep: {e}")
        return

    # Regex to parse file, line, arg1, arg2
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

    # Print top 10 by arg1
    print("Top calls by ref time:")
    for i, m in enumerate(sorted(matches, key=lambda x: x['arg1'], reverse=True)):
        print(m['text'])
        fn_name = find_function_name(m['file'], m['line'])
        formatted = format_number_with_underscores(m['arg1'])
        emoji = get_emoji(m['arg1'], MAX_REF_TIME)
        print(f"{emoji}{formatted:>27}: {fn_name}")
        if i+2 > 10 and m['arg1'] < MAX_REF_TIME:
            break

    # Print top 10 by arg2
    print("")
    print("Top calls by proof size:")
    for i, m in enumerate(sorted(matches, key=lambda x: x['arg2'], reverse=True)):
        print(m['text'])
        fn_name = find_function_name(m['file'], m['line'])
        formatted = format_number_with_underscores(m['arg2'])
        emoji = get_emoji(m['arg2'], MAX_PROOF_SIZE)
        print(f"{emoji}{formatted:>27}: {fn_name}")
        if i+2 > 10 and m['arg2'] < MAX_PROOF_SIZE:
            break

if __name__ == '__main__':
    main()

