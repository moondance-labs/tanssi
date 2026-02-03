#!/usr/bin/env python3
"""
Check Cargo manifests for workspace key usage.

This script scans all tracked `Cargo.toml` files in the current Git repository
(excluding the repository root `Cargo.toml`) and verifies that each contains:

    [lints]
    workspace = true

Why this matters:
If a crate-level `Cargo.toml` does not opt into workspace lints, tools such as
`cargo clippy` will use default lint settings instead of the repository’s
workspace lint configuration, which can lead to inconsistent diagnostics.

Behavior:
- Prints: `Missing [lints] workspace = true in <path>` for each offending file.
- Exits with:
  - 0 if all checked manifests have `lints.workspace = true`
  - 1 if any manifest is missing it, or if a manifest cannot be read/parsed
  - a non-zero Git return code if the repository root or file list cannot be
    determined via Git

Requirements:
- Python 3.11+ (for `tomllib`)
- `git` available on PATH
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Iterable, Sequence

import tomllib


def _run_git(
    args: Sequence[str],
    *,
    cwd: Path,
    capture_stdout: bool = True,
) -> subprocess.CompletedProcess[bytes]:
    """
    Run a git command in `cwd`, returning the CompletedProcess.

    Stdout/stderr are captured as bytes for robustness.
    """
    return subprocess.run(
        ["git", *args],
        cwd=str(cwd),
        check=False,
        stdout=subprocess.PIPE if capture_stdout else None,
        stderr=subprocess.PIPE,
    )


def get_repo_root(start_dir: Path) -> Path:
    """
    Return the root path of the current git repository
    """
    proc = _run_git(["rev-parse", "--show-toplevel"], cwd=start_dir)
    if proc.returncode != 0:
        msg = (proc.stderr or b"").decode("utf-8", errors="replace").strip()
        if not msg:
            msg = "failed to determine repository root (git rev-parse --show-toplevel)"
        raise RuntimeError(msg)
    root = proc.stdout.decode("utf-8", errors="replace").strip()
    if not root:
        raise RuntimeError("git rev-parse returned an empty repository root path")
    return Path(root).resolve()


def iter_tracked_cargo_tomls(repo_root: Path) -> Iterable[str]:
    """
    Yield tracked Cargo.toml paths (Git-relative, POSIX-style) excluding root Cargo.toml.

    Uses `git ls-files -z` for correct handling of special characters in filenames.
    """
    proc = _run_git(["ls-files", "-z"], cwd=repo_root)
    if proc.returncode != 0:
        msg = (proc.stderr or b"").decode("utf-8", errors="replace").strip()
        if not msg:
            msg = "failed to list tracked files (git ls-files)"
        raise RuntimeError(msg)

    # Split NUL-separated paths; ignore the trailing empty chunk if present.
    raw_paths = proc.stdout.split(b"\0")
    for raw in raw_paths:
        if not raw:
            continue
        rel = raw.decode("utf-8", errors="surrogateescape")

        # Match all Cargo.toml files except the root.
        # This excludes the repository root Cargo.toml because it doesn't start with "/".
        if rel.endswith("/Cargo.toml"):
            yield rel


def manifest_has_workspace_lints(manifest_path: Path) -> bool:
    """
    Return True iff the TOML manifest contains:
        [lints]
        workspace = true
    """
    with manifest_path.open("rb") as f:
        data = tomllib.load(f)

    lints = data.get("lints")
    if not isinstance(lints, dict):
        return False
    return lints.get("workspace") is True


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Verify that all tracked Cargo.toml files (excluding the repo root) "
            "contain `[lints]` with `workspace = true`."
        )
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=None,
        help=(
            "Optional explicit repository root. If omitted, uses "
            "`git rev-parse --show-toplevel` starting from this script’s directory."
        ),
    )
    args = parser.parse_args(argv)

    script_dir = Path(__file__).resolve().parent
    try:
        repo_root = args.repo_root.resolve() if args.repo_root else get_repo_root(script_dir)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1

    failure = 0

    try:
        for rel in iter_tracked_cargo_tomls(repo_root):
            abs_path = repo_root / Path(rel)  # rel uses POSIX separators; Path handles this fine.
            try:
                ok = manifest_has_workspace_lints(abs_path)
            except FileNotFoundError:
                print(f"Error: file not found: {rel}", file=sys.stderr)
                ok = False
            except PermissionError:
                print(f"Error: permission denied reading: {rel}", file=sys.stderr)
                ok = False
            except tomllib.TOMLDecodeError as te:
                print(f"Error: invalid TOML in {rel}: {te}", file=sys.stderr)
                ok = False
            except OSError as oe:
                print(f"Error: could not read {rel}: {oe}", file=sys.stderr)
                ok = False

            if not ok:
                print(f"Missing [lints] workspace = true in {rel}")
                failure = 1

    except RuntimeError as e:
        # Git failures for listing tracked files should be surfaced and fail the run.
        print(f"Error: {e}", file=sys.stderr)
        return 1

    return 1 if failure else 0


if __name__ == "__main__":
    raise SystemExit(main())
