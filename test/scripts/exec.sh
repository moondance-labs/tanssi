#!/usr/bin/env bash
# Wrapper script to exec a command
# Used for zombienet hacks because zombienet expects the binary to be a file
# So instead of "pnpm" we execute "scripts/exec.sh pnpm"
exec "$@"