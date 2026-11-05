//! The C sources of the FFI examples must compile without warnings
//! (`clang -std=c11 -Wall -Wextra -Werror -fsyntax-only`).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::text::{rel, walk};

/// The clang that was found and its version line.
#[derive(Debug)]
pub struct Clang {
    pub program: PathBuf,
    pub version: String,
}

/// Finds clang: `KENOX_CLANG` if set; otherwise `clang` (or `clang-NN`) in `PATH`;
/// on Windows also the default LLVM folder, because the LLVM installer does
/// not always add it to `PATH`.
pub fn find_clang() -> Result<Clang, String> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(path) = std::env::var_os("KENOX_CLANG") {
        candidates.push(PathBuf::from(path));
    } else {
        for name in ["clang", "clang-20", "clang-19", "clang-18"] {
            candidates.push(PathBuf::from(name));
        }
        if cfg!(windows) {
            candidates.push(PathBuf::from(r"C:\Program Files\LLVM\bin\clang.exe"));
        }
    }
    for program in &candidates {
        let Ok(out) = Command::new(program).arg("--version").output() else {
            continue;
        };
        if out.status.success() {
            let version = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            return Ok(Clang {
                program: program.clone(),
                version,
            });
        }
    }
    let tried: Vec<String> = candidates.iter().map(|p| p.display().to_string()).collect();
    Err(format!(
        "clang не найден (проверено: {}). Установите LLVM по инструкции docs/WORKFLOW.md \
         или укажите путь к clang в переменной окружения KENOX_CLANG",
        tried.join(", ")
    ))
}

/// Compiles every `.c` file under `examples/` in syntax-only mode.
pub fn check(root: &Path) -> Vec<String> {
    let files = walk(&root.join("examples"), "c");
    if files.is_empty() {
        return Vec::new();
    }
    let clang = match find_clang() {
        Ok(clang) => clang,
        Err(e) => return vec![e],
    };
    let mut problems = Vec::new();
    for file in &files {
        let mut cmd = Command::new(&clang.program);
        cmd.args(["-std=c11", "-Wall", "-Wextra", "-Werror", "-fsyntax-only"]);
        if cfg!(windows) {
            cmd.arg("-D_CRT_SECURE_NO_WARNINGS");
        }
        // Run from the repository root with relative paths, so that messages
        // look the same on every machine.
        cmd.current_dir(root);
        let dir = file.parent().unwrap_or(root);
        cmd.arg("-I").arg(rel(root, dir));
        // Package layout of the examples: <package>/c/*.c and <package>/include/*.h.
        if dir.file_name().is_some_and(|n| n == "c")
            && let Some(include) = dir
                .parent()
                .map(|p| p.join("include"))
                .filter(|p| p.is_dir())
        {
            cmd.arg("-I").arg(rel(root, &include));
        }
        cmd.arg(rel(root, file));
        match cmd.output() {
            Ok(out) if out.status.success() => {}
            Ok(out) => problems.push(format!(
                "{}: clang сообщает об ошибках или предупреждениях:\n{}",
                rel(root, file),
                String::from_utf8_lossy(&out.stderr).trim_end()
            )),
            Err(e) => problems.push(format!(
                "{}: не удалось запустить clang: {e}",
                rel(root, file)
            )),
        }
    }
    problems
}
