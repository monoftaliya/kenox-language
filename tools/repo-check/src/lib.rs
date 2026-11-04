//! Repository checks for KENOX: design documents, the example corpus and the C
//! sources of the FFI examples.
//!
//! Every check returns human-readable problems (in Russian, for the user); an
//! empty list means the check passed. `cargo test` runs all checks
//! (`tests/repo.rs`); `cargo run -p kenox-repo-check` prints a full report with
//! corpus metrics.
//!
//! The crate deliberately has no dependencies (ADR-0027): it must build
//! anywhere `cargo` works, including offline.

pub mod cexamples;
pub mod corpus;
pub mod docs;
mod text;

use std::path::{Path, PathBuf};

/// Root of the repository: this crate lives in `tools/repo-check`.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .expect("the crate lives in tools/<name> inside the repository")
}
