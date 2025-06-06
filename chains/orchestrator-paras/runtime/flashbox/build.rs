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

use substrate_wasm_builder::WasmBuilder;

#[cfg(feature = "metadata-hash")]
fn main() {
    WasmBuilder::new()
        .with_current_project()
        .enable_metadata_hash("FLASH", 12)
        .export_heap_base()
        .import_memory()
        .build()
}

#[cfg(not(feature = "metadata-hash"))]
fn main() {
    WasmBuilder::new()
        .with_current_project()
        .export_heap_base()
        .import_memory()
        .build()
}
