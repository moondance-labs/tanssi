#!/usr/bin/env python3

import os
import stat
import glob
import argparse
import re
import logging
from pathlib import Path
from dataclasses import dataclass
from typing import Optional, List, Dict

@dataclass
class CovArgs:
    include_build_script: bool

@dataclass
class Args:
    workspace_root: Path
    target_dir: Path
    doctests_dir: Path
    target: Optional[str]
    release: bool
    cargo_profile: Optional[str]
    doctests: bool
    cov: CovArgs
    build_script_pattern: str

@dataclass
class Target:
    name: str

@dataclass
class Package:
    name: str
    targets: List[Target]

@dataclass
class Metadata:
    # IDs of workspace members (e.g. package names or paths)
    workspace_members: List[str]
    # map from member ID to Package
    packages: Dict[str, Package]

@dataclass
class Workspace:
    root: Path
    target_dir: Path
    doctests_dir: Path
    metadata: Metadata

class Context:
    def __init__(self, args: Args):
        self.args = args
        self.ws = Workspace(
            root=args.workspace_root,
            target_dir=args.target_dir,
            doctests_dir=args.doctests_dir,
            metadata=self._load_metadata()
        )
        self.build_script_re = re.compile(args.build_script_pattern)

    def _load_metadata(self) -> Metadata:
        # TODO: load real cargo metadata here (e.g. via `cargo metadata --format-version 1`)
        # Stubbed empty metadata:
        return Metadata(workspace_members=[], packages={})

def parse_args() -> Args:
    parser = argparse.ArgumentParser(
        description="Find object files in a Cargo workspace"
    )
    parser.add_argument(
        "--workspace-root",
        type=Path,
        default=Path.cwd(),
        help="Path to the workspace root (default: current dir)",
    )
    parser.add_argument(
        "--target-dir",
        type=Path,
        default=Path.cwd() / "target",
        help="Path to the Cargo target directory (default: ./target)",
    )
    parser.add_argument(
        "--doctests-dir",
        type=Path,
        default=Path.cwd() / "doctests",
        help="Path to doctest outputs",
    )
    parser.add_argument(
        "--target",
        type=str,
        help="Optional Rust compilation target (e.g. x86_64-unknown-linux-gnu)",
    )
    parser.add_argument(
        "--release",
        action="store_true",
        help="Use the release profile",
    )
    parser.add_argument(
        "--cargo-profile",
        type=str,
        choices=["dev", "test", "release", "bench"],
        help="Explicit Cargo profile to use",
    )
    parser.add_argument(
        "--doctests",
        action="store_true",
        help="Include doctest artifacts",
    )
    parser.add_argument(
        "--include-build-script",
        action="store_true",
        dest="include_build_script",
        help="Also include build-script outputs",
    )
    parser.add_argument(
        "--build-script-pattern",
        type=str,
        default=".*",
        help="Regex for allowed build-script output dirs",
    )
    ns = parser.parse_args()
    return Args(
        workspace_root=ns.workspace_root,
        target_dir=ns.target_dir,
        doctests_dir=ns.doctests_dir,
        target=ns.target,
        release=ns.release,
        cargo_profile=ns.cargo_profile,
        doctests=ns.doctests,
        cov=CovArgs(include_build_script=ns.include_build_script),
        build_script_pattern=ns.build_script_pattern,
    )

def object_files(cx) -> list[str]:
    """
    Python port of the Rust `object_files` function,
    without Windows, Nextest or Trybuild support.
    """

    # --- pkg-hash regex builder --- #
    def pkg_hash_re(ws) -> re.Pattern:
        targets: set[str] = set()
        for member_id in ws.metadata.workspace_members:
            pkg = ws.metadata.packages[member_id]
            targets.add(pkg.name)
            for t in pkg.targets:
                targets.add(t.name)
        parts = [t.replace('-', '(-|_)') for t in targets]
        pattern = rf'^(lib)?({"|".join(parts)})(-[0-9a-f]+)?$'
        return re.compile(pattern)

    # --- strip all extensions (foo.tar.gz → "foo") --- #
    def file_stem_recursive(path: Path) -> str:
        p = path
        while p.suffix:
            p = p.with_suffix('')
        return p.name

    # --- detect nested “build” directories for build-script outputs --- #
    def in_build_dir(p: Path) -> bool:
        return bool(p.parent and p.parent.parent and p.parent.parent.name == 'build')

    # --- filesystem walker with optional build-script pruning --- #
    def walk_target_dir(cx, target_dir: Path):
        skip_dirs = {'incremental', '.fingerprint'}
        skip_dirs.add('out' if cx.args.cov.include_build_script else 'build')

        for root, dirs, files in os.walk(target_dir, followlinks=False):
            dirs[:] = [d for d in dirs if d not in skip_dirs]
            for fname in files:
                p = Path(root) / fname
                if cx.args.cov.include_build_script and in_build_dir(p):
                    stem = p.stem
                    if (stem == 'build-script-build' or
                        stem.startswith('build_script_build-')):
                        if not cx.build_script_re.match(p.parent.name):
                            continue
                    else:
                        continue
                yield p

    # --- object-file predicate (Unix only) --- #
    def is_object(f: Path) -> bool:
        ext = f.suffix.lstrip('.')
        if ext in ('d', 'rlib', 'rmeta') or f.name.endswith('.cargo-lock'):
            return False
        if not f.is_file():
            return False
        mode = f.stat().st_mode
        return bool(mode & (stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH))

    # TODO: this results in empty output
    #re_pkg = pkg_hash_re(cx.ws)
    re_pkg = re.compile(r'.*')
    results: list[str] = []
    searched_dirs: list[str] = []

    # --- determine target_dir --- #
    target_dir = Path(cx.ws.target_dir)
    if cx.args.target:
        target_dir = target_dir / cx.args.target

    # choose profile
    cp = cx.args.cargo_profile
    if cp is None and cx.args.release:
        profile = 'release'
    elif cp in ('release', 'bench'):
        profile = 'release'
    elif cp is None or cp in ('dev', 'test'):
        profile = 'debug'
    else:
        profile = cp
    target_dir = target_dir / profile

    # --- scan target tree --- #
    for p in walk_target_dir(cx, target_dir):
        if is_object(p):
            stem = file_stem_recursive(p)
            if re_pkg.match(stem):
                rel = os.path.relpath(p, start=Path(cx.ws.root))
                results.append(rel)
    searched_dirs.append(str(target_dir))

    # --- optional: doctests directory scan --- #
    if cx.args.doctests:
        pattern = os.path.join(cx.ws.doctests_dir, '*', 'rust_out')
        for p in glob.glob(pattern):
            p = Path(p)
            if is_object(p):
                rel = os.path.relpath(p, start=Path(cx.ws.root))
                results.append(rel)
        searched_dirs.append(cx.ws.doctests_dir)

    # sort & warn if empty
    results.sort()
    if not results:
        logging.warning(f"no object files found (searched: {','.join(searched_dirs)})")

    return results

def main():
    args = parse_args()
    cx = Context(args)
    files = object_files(cx)
    print(" -object ".join(files))

if __name__ == "__main__":
    main()

