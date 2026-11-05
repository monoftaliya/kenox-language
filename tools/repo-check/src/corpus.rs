//! Lint and metrics for the example corpus (`examples/**/*.kenox`).
//!
//! The lint is a heuristic, not a parser: comments are removed and string
//! literals blanked, then each line is searched for constructs of other
//! languages that the syntax sketch replaced (`fn`, `=>`, `when`, `let`, …).

use std::path::Path;

use crate::text::{count_word, is_word, read, rel, walk, word_positions};

/// Result of the corpus check.
#[derive(Debug, Default)]
pub struct CorpusReport {
    pub problems: Vec<String>,
    pub files: usize,
    /// Metric name and value, in report order.
    pub metrics: Vec<(&'static str, usize)>,
}

const WORDS: [(&str, &str); 6] = [
    ("fn", "fn (Rust) — в KENOX fun"),
    ("when", "when (Kotlin) — в KENOX match"),
    ("let", "let — в KENOX val/var"),
    ("impl", "impl (Rust) — в KENOX extend"),
    ("mut", "mut (Rust) — в KENOX var или inout"),
    ("this", "this (Kotlin) — в KENOX self"),
];

const METRIC_NAMES: [&str; 17] = [
    "строк кода",
    "строк комментариев",
    "отвергаемых случаев (✗)",
    "пометок ⚠",
    "unsafe",
    "clone()",
    "@borrows",
    "ref",
    "consume",
    "inout",
    "& при вызове",
    "try",
    "Box<dyn",
    ".share()",
    "escaping",
    "some",
    "do {",
];

/// Lints every `.kenox` file under `examples/` and sums the metrics.
pub fn check(root: &Path) -> CorpusReport {
    let mut report = CorpusReport::default();
    let mut totals = [0usize; METRIC_NAMES.len()];
    for path in walk(&root.join("examples"), "kenox") {
        report.files += 1;
        match read(&path) {
            Ok(text) => {
                let counts = lint_source(&rel(root, &path), &text, &mut report.problems);
                for (total, value) in totals.iter_mut().zip(counts) {
                    *total += value;
                }
            }
            Err(e) => report.problems.push(e),
        }
    }
    report.metrics = METRIC_NAMES.iter().copied().zip(totals).collect();
    report.metrics.push(("файлов .kenox", report.files));
    report
}

/// Lints one file; pushes problems and returns its metrics in `METRIC_NAMES` order.
fn lint_source(name: &str, text: &str, problems: &mut Vec<String>) -> [usize; METRIC_NAMES.len()] {
    let raw_lines: Vec<&str> = text.split('\n').collect();
    let first = raw_lines.first().copied().unwrap_or("");
    if !(first.starts_with("// Пример") || first.starts_with("// Модуль")) {
        problems.push(format!(
            "{name}:1: нет заголовка «// Пример …» или «// Модуль …»"
        ));
    }

    let code_lines = lint_code(name, text, 1, problems);
    let code = code_lines.join("\n");
    [
        code_lines.iter().filter(|l| !l.trim().is_empty()).count(),
        raw_lines
            .iter()
            .filter(|l| l.trim_start().starts_with("//"))
            .count(),
        raw_lines
            .iter()
            .filter(|l| l.trim().starts_with("//   ошибка"))
            .count(),
        raw_lines.iter().filter(|l| l.contains("// ⚠")).count(),
        count_word(&code, "unsafe"),
        code.matches(".clone()").count(),
        code.matches("@borrows").count(),
        count_word(&code, "ref"),
        count_word(&code, "consume"),
        count_word(&code, "inout"),
        call_site_ampersands(&code),
        count_word(&code, "try"),
        code.matches("Box<dyn").count(),
        code.matches(".share()").count(),
        count_word(&code, "escaping"),
        count_word(&code, "some"),
        keyword_then(&code.chars().collect::<Vec<_>>(), "do", '{'),
    ]
}

/// Checks KENOX source text for constructs of other languages. `first_line` is
/// the line number of the text's first line in its file (for code blocks inside
/// Markdown). Returns the code lines with comments and string bodies removed.
pub(crate) fn lint_code(
    name: &str,
    text: &str,
    first_line: usize,
    problems: &mut Vec<String>,
) -> Vec<String> {
    let (code_lines, strings) = split_code(text);
    for (no, line) in code_lines.iter().enumerate() {
        let line_no = first_line + no;
        let chars: Vec<char> = line.chars().collect();
        let stripped = line.trim();
        for message in foreign_constructs(&chars) {
            problems.push(format!("{name}:{line_no}: {message}: {stripped}"));
        }
        if rust_question_mark(&chars) && !is_fun_or_extern(stripped) {
            problems.push(format!(
                "{name}:{line_no}: возможен `?` в стиле Rust: {stripped}"
            ));
        }
        if arrow_return_type(line) {
            problems.push(format!(
                "{name}:{line_no}: тип результата функции через -> (нужно :): {stripped}"
            ));
        }
    }
    for s in &strings {
        if kotlin_interpolation(s) {
            let short: String = s.chars().take(40).collect();
            problems.push(format!(
                "{name}: интерполяция в стиле Kotlin ($): \"{short}\""
            ));
        }
    }
    code_lines
}

/// Messages for constructs of other languages found in one code line.
fn foreign_constructs(line: &[char]) -> Vec<&'static str> {
    let s: String = line.iter().collect();
    let mut found = Vec::new();
    let has_word = |w: &str| !word_positions(line, w).is_empty();
    if has_word("fn") {
        found.push(WORDS[0].1);
    }
    if s.contains("=>") {
        found.push("=> (Rust) — в ветках match используется ->");
    }
    for (word, message) in &WORDS[1..] {
        if has_word(word) {
            found.push(*message);
        }
    }
    if s.contains("::") {
        found.push(":: (Rust) — пути через точку");
    }
    if keyword_then(line, "if", '(') > 0 {
        found.push("скобки вокруг условия if");
    }
    if keyword_then(line, "while", '(') > 0 {
        found.push("скобки вокруг условия while");
    }
    if bare_range(line) {
        found.push("диапазон .. без < или =");
    }
    if new_type(line) {
        found.push("new Тип (Java/Kotlin)");
    }
    if has_word("class") {
        found.push("class зарезервирован до версии после 1.0");
    }
    found
}

/// Number of places where `word` is followed (after optional whitespace) by `next`.
fn keyword_then(chars: &[char], word: &str, next: char) -> usize {
    word_positions(chars, word)
        .into_iter()
        .filter(|&p| {
            let mut j = p + word.chars().count();
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            chars.get(j) == Some(&next)
        })
        .count()
}

/// `..` that is neither `..<`, `..=` nor part of `...`.
fn bare_range(line: &[char]) -> bool {
    (0..line.len().saturating_sub(1)).any(|i| {
        line[i] == '.'
            && line[i + 1] == '.'
            && (i == 0 || line[i - 1] != '.')
            && !matches!(line.get(i + 2), Some('<' | '=' | '.'))
    })
}

/// `new Type` (Java/Kotlin object creation).
fn new_type(line: &[char]) -> bool {
    word_positions(line, "new").into_iter().any(|p| {
        let mut j = p + 3;
        let start = j;
        while j < line.len() && line[j].is_whitespace() {
            j += 1;
        }
        j > start && line.get(j).is_some_and(char::is_ascii_uppercase)
    })
}

/// `)?` followed by whitespace, `)`, `,` or the end of the line.
fn rust_question_mark(line: &[char]) -> bool {
    (0..line.len().saturating_sub(1)).any(|i| {
        line[i] == ')'
            && line[i + 1] == '?'
            && line
                .get(i + 2)
                .is_none_or(|&c| c.is_whitespace() || c == ')' || c == ',')
    })
}

fn starts_with_word(s: &str, word: &str) -> bool {
    s.strip_prefix(word)
        .is_some_and(|r| !r.starts_with(is_word))
}

fn is_fun_or_extern(stripped: &str) -> bool {
    let s = stripped
        .strip_prefix("pub ")
        .or_else(|| stripped.strip_prefix("private "))
        .unwrap_or(stripped);
    starts_with_word(s, "fun") || starts_with_word(s, "extern")
}

/// A function declaration whose result type is written with `->` instead of `:`.
fn arrow_return_type(line: &str) -> bool {
    let mut rest = line.trim_start();
    for visibility in ["pub", "private"] {
        if let Some(r) = rest.strip_prefix(visibility)
            && r.starts_with(char::is_whitespace)
        {
            rest = r.trim_start();
            break;
        }
    }
    if let Some(r) = rest.strip_prefix("extern \"C\"")
        && r.starts_with(char::is_whitespace)
    {
        rest = r.trim_start();
    }
    let Some(r) = rest.strip_prefix("fun") else {
        return false;
    };
    if !r.starts_with(char::is_whitespace) {
        return false;
    }
    let r = r.trim_start();
    let Some(open) = r.find('(') else {
        return false;
    };
    if !r[..open].chars().all(|c| is_word(c) || "<>:, ".contains(c)) {
        return false;
    }
    let mut depth = 0;
    for (k, c) in r[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return r[open + k + 1..].trim_start().starts_with("->");
                }
            }
            _ => {}
        }
    }
    false
}

/// `${…}` or `$name` inside a string literal.
fn kotlin_interpolation(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    (0..c.len().saturating_sub(1)).any(|i| {
        c[i] == '$' && (c[i + 1] == '{' || c[i + 1].is_ascii_alphabetic() || c[i + 1] == '_')
    })
}

/// `&name` right after `(`, `,` or the word `in` — an argument passed as `inout`.
fn call_site_ampersands(code: &str) -> usize {
    let c: Vec<char> = code.chars().collect();
    (0..c.len().saturating_sub(1))
        .filter(|&k| c[k] == '&' && (c[k + 1].is_ascii_alphabetic() || c[k + 1] == '_'))
        .filter(|&k| {
            let mut p = k;
            while p > 0 && c[p - 1].is_whitespace() {
                p -= 1;
            }
            if p == 0 {
                return false;
            }
            let prev = c[p - 1];
            prev == '('
                || prev == ','
                || (p >= 2 && c[p - 2] == 'i' && prev == 'n' && (p == 2 || !is_word(c[p - 3])))
        })
        .count()
}

/// Splits source into code lines (comments removed, string bodies blanked) and string contents.
fn split_code(text: &str) -> (Vec<String>, Vec<String>) {
    #[derive(PartialEq)]
    enum State {
        Code,
        Block,
        Str,
        Raw,
        Triple,
    }
    let t: Vec<char> = text.chars().collect();
    let n = t.len();
    let at = |i: usize, pat: &str| {
        pat.chars()
            .enumerate()
            .all(|(k, p)| t.get(i + k) == Some(&p))
    };
    let mut buf = String::new();
    let mut strings = Vec::new();
    let mut cur = String::new();
    let mut state = State::Code;
    let mut depth = 0;
    let mut i = 0;
    while i < n {
        let c = t[i];
        let next = t.get(i + 1).copied();
        match state {
            State::Code => {
                if c == '/' && next == Some('/') {
                    while i < n && t[i] != '\n' {
                        i += 1;
                    }
                } else if c == '/' && next == Some('*') {
                    state = State::Block;
                    depth = 1;
                    i += 2;
                } else if at(i, "r\"\"\"") || at(i, "\"\"\"") {
                    let raw = c == 'r';
                    state = State::Triple;
                    cur.clear();
                    buf.push_str(if raw {
                        "r\"\"\" \"\"\""
                    } else {
                        "\"\"\" \"\"\""
                    });
                    i += if raw { 4 } else { 3 };
                } else if c == 'r' && next == Some('"') && (i == 0 || !is_word(t[i - 1])) {
                    state = State::Raw;
                    cur.clear();
                    buf.push_str("r\" \"");
                    i += 2;
                } else if c == '"' {
                    state = State::Str;
                    cur.clear();
                    buf.push_str("\" \"");
                    i += 1;
                } else if let Some(end) = char_literal_end(&t, i) {
                    buf.push_str("'c'");
                    i = end + 1;
                } else {
                    buf.push(c);
                    i += 1;
                }
            }
            State::Block => {
                if c == '/' && next == Some('*') {
                    depth += 1;
                    i += 2;
                } else if c == '*' && next == Some('/') {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        state = State::Code;
                    }
                } else {
                    if c == '\n' {
                        buf.push('\n');
                    }
                    i += 1;
                }
            }
            State::Str | State::Raw => {
                if state == State::Str && c == '\\' {
                    cur.push(c);
                    cur.extend(next);
                    i += 2;
                } else if c == '"' {
                    strings.push(std::mem::take(&mut cur));
                    state = State::Code;
                    i += 1;
                } else {
                    cur.push(c);
                    i += 1;
                }
            }
            State::Triple => {
                if at(i, "\"\"\"") {
                    strings.push(std::mem::take(&mut cur));
                    state = State::Code;
                    i += 3;
                } else {
                    if c == '\n' {
                        buf.push('\n');
                    }
                    cur.push(c);
                    i += 1;
                }
            }
        }
    }
    (buf.split('\n').map(str::to_string).collect(), strings)
}

/// End index of a character literal `'x'` or `'\n'` starting at `i`.
fn char_literal_end(t: &[char], i: usize) -> Option<usize> {
    if t[i] != '\'' || i + 2 >= t.len() {
        return None;
    }
    let escaped = t[i + 1] == '\\';
    if !(escaped || t[i + 2] == '\'') {
        return None;
    }
    let from = if escaped { i + 3 } else { i + 2 };
    (from..t.len()).find(|&j| t[j] == '\'')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problems_in(body: &str) -> Vec<String> {
        let mut problems = Vec::new();
        lint_source("t.kenox", &format!("// Пример\n{body}\n"), &mut problems);
        problems
    }

    #[test]
    fn foreign_constructs_are_reported() {
        let cases = [
            "fn main() {}",
            "match x { A => 1 }",
            "when (x) {}",
            "let a = 1",
            "impl Foo {}",
            "var mut x = 1",
            "this.x = 1",
            "val f = std::fs",
            "if (a) {}",
            "while(a) {}",
            "for i in 0..n {}",
            "val p = new Point()",
            "class A {}",
            "val x = parse(s)?",
            "fun f(a: Int) -> Int = a",
            "val s = \"${x}\"",
            "val s = \"$name\"",
        ];
        for body in cases {
            assert!(!problems_in(body).is_empty(), "не найдено: {body}");
        }
    }

    #[test]
    fn kenox_constructs_pass() {
        let ok = [
            "fun main() {",
            "    val r = 0..<10",
            "    val q = 0..=9",
            "    val s = \"{x} — r\\\"кавычка\\\" fn let => ::\"",
            "    val t = r\"C:\\путь\\fn\"",
            "    val u = \"\"\"",
            "        when let impl => ::",
            "    \"\"\"",
            "    if a { b() } // fn и let в комментарии",
            "    /* when (x) => /* вложенный */ impl */",
            "    val v = x?.y ?: z",
            "    match v { .A -> 1 }",
            "    val c = '\"'",
            "    deposit(&acc, 50)",
            "}",
            "fun parse(s: str): Int? = s.toInt()",
        ]
        .join("\n");
        assert_eq!(problems_in(&ok), Vec::<String>::new());
    }

    #[test]
    fn header_is_required() {
        let mut problems = Vec::new();
        lint_source("t.kenox", "fun main() {}\n", &mut problems);
        assert_eq!(problems.len(), 1);
    }

    #[test]
    fn metrics_count_code_only() {
        let src = "// Пример\n// ✗ x\n//   ошибка: y\nfun f(inout a: A) { g(&a, &b) } // inout\n// ⚠ Р-1: z\nfor inout x in &ys { try h() }\n";
        let m = lint_source("t.kenox", src, &mut Vec::new());
        let get = |name: &str| m[METRIC_NAMES.iter().position(|n| *n == name).unwrap()];
        assert_eq!(get("строк кода"), 2);
        assert_eq!(get("отвергаемых случаев (✗)"), 1);
        assert_eq!(get("пометок ⚠"), 1);
        assert_eq!(get("inout"), 2);
        assert_eq!(get("& при вызове"), 3);
        assert_eq!(get("try"), 1);
    }
}
