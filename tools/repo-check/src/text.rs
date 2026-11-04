//! Small text and file helpers shared by the checks (std only, no regex).

use std::fs;
use std::path::{Path, PathBuf};

/// Reads a UTF-8 file and normalizes line endings to LF.
pub fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map(|t| t.replace("\r\n", "\n"))
        .map_err(|e| format!("{}: не удалось прочитать файл: {e}", path.display()))
}

/// All files with the given extension under `dir`, sorted. Hidden directories
/// and `target` are skipped.
pub fn walk(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk_into(dir, ext, &mut out);
    out.sort();
    out
}

fn walk_into(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with('.') && name != "target" {
                walk_into(&path, ext, out);
            }
        } else if path.extension().is_some_and(|e| e == ext) {
            out.push(path);
        }
    }
}

/// Path relative to `root`, with `/` separators on every OS.
pub fn rel(root: &Path, path: &Path) -> String {
    let p = path.strip_prefix(root).unwrap_or(path);
    p.components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// Letters, digits and `_` (Unicode-aware, like `\w` in regular expressions).
pub fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Positions where `word` occurs in `chars` as a whole word.
pub fn word_positions(chars: &[char], word: &str) -> Vec<usize> {
    let w: Vec<char> = word.chars().collect();
    if w.is_empty() || chars.len() < w.len() {
        return Vec::new();
    }
    (0..=chars.len() - w.len())
        .filter(|&i| {
            chars[i..i + w.len()] == w[..]
                && (i == 0 || !is_word(chars[i - 1]))
                && chars.get(i + w.len()).is_none_or(|&c| !is_word(c))
        })
        .collect()
}

/// Number of whole-word occurrences of `word` in `text`.
pub fn count_word(text: &str, word: &str) -> usize {
    let chars: Vec<char> = text.chars().collect();
    word_positions(&chars, word).len()
}

/// Reads ASCII digits starting at `i`; returns them and the position after them.
pub fn digits_at(chars: &[char], i: usize) -> (String, usize) {
    let mut j = i;
    while j < chars.len() && chars[j].is_ascii_digit() {
        j += 1;
    }
    (chars[i.min(j)..j].iter().collect(), j)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn whole_words_only() {
        assert_eq!(
            word_positions(&chars("fn fun fn_x xfn fn"), "fn"),
            vec![0, 16]
        );
        assert_eq!(count_word("ref refs (ref) preref", "ref"), 2);
        assert_eq!(count_word("пример ref пример", "ref"), 1);
        assert_eq!(count_word("приref", "ref"), 0);
    }

    #[test]
    fn digits() {
        assert_eq!(digits_at(&chars("Q-07."), 2), ("07".to_string(), 4));
        assert_eq!(digits_at(&chars("Q-x"), 2), (String::new(), 2));
    }

    #[test]
    fn relative_paths_use_slashes() {
        let root = Path::new("repo");
        assert_eq!(rel(root, &root.join("docs").join("ADR")), "docs/ADR");
    }
}
