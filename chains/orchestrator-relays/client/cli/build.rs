// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

fn main() {
    if let Ok(profile) = std::env::var("PROFILE") {
        println!("cargo:rustc-cfg=build_type=\"{}\"", profile);
    }
    #[cfg(feature = "include-git-hash-in-version")]
    {
        substrate_build_script_utils::generate_cargo_keys();
        // For the node/worker version check, make sure we always rebuild the node and binary workers
        // when the version changes.
        substrate_build_script_utils::rerun_if_git_head_changed();
    }
    #[cfg(not(feature = "include-git-hash-in-version"))]
    {
        let commit = "dev";
        println!("cargo:rustc-env=SUBSTRATE_CLI_COMMIT_HASH={commit}");
        println!(
            "cargo:rustc-env=SUBSTRATE_CLI_IMPL_VERSION={}",
            get_version(&commit)
        );
        // Never re-run this build script
        println!("cargo:rerun-if-changed=build.rs");
    }
}

#[cfg(not(feature = "include-git-hash-in-version"))]
fn get_version(impl_commit: &str) -> String {
    let commit_dash = if impl_commit.is_empty() { "" } else { "-" };

    format!(
        "{}{}{}",
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
        commit_dash,
        impl_commit
    )
}
