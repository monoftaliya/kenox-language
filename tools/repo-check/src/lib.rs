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
///
/// `cargo test` and `cargo run` set `CARGO_MANIFEST_DIR` at run time; it is
/// preferred over the compile-time value so that a moved checkout is still
/// found correctly without a rebuild.
pub fn repo_root() -> PathBuf {
    let manifest = std::env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    manifest
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .expect("the crate lives in tools/<name> inside the repository")
}
