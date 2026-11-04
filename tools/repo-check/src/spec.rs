//! Consistency of the language specification (`spec/`): rule identifiers and
//! references to them, the EBNF grammar, and KENOX examples in the
//! specification and the syntax sketch.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::corpus::lint_code;
use crate::text::{read, rel, walk};

/// Prefixes of rule identifiers: `spec` for general rules, then one per chapter
/// (see the chapter table in `spec/README.md`).
pub const RULE_PREFIXES: [&str; 11] = [
    "spec", "lex", "syn", "name", "type", "expr", "pat", "own", "drop", "unsafe", "prog",
];

/// Start symbols of the lexical and the syntactic grammar; only they may be unused.
const GRAMMAR_ROOTS: [&str; 2] = ["INPUT", "File"];

/// Counts printed in the report.
#[derive(Debug, Default)]
pub struct Summary {
    pub rules: usize,
    pub grammar_rules: usize,
    pub examples: usize,
}

/// All specification problems.
pub fn check(root: &Path) -> Vec<String> {
    check_with_summary(root).0
}

/// All specification problems plus counts for the report.
pub fn check_with_summary(root: &Path) -> (Vec<String>, Summary) {
    let mut problems = Vec::new();
    let mut spec_texts: Vec<(PathBuf, String)> = Vec::new();
    for path in walk(&root.join("spec"), "md") {
        match read(&path) {
            Ok(text) => spec_texts.push((path, text)),
            Err(e) => problems.push(e),
        }
    }

    // 1. Every rule identifier is defined once.
    let mut defined: BTreeMap<String, String> = BTreeMap::new();
    for (path, text) in &spec_texts {
        for (line, id) in rule_definitions(text, &mut problems, &rel(root, path)) {
            let here = format!("{}:{line}", rel(root, path));
            match defined.get(&id) {
                Some(first) => {
                    problems.push(format!("{here}: правило [{id}] уже определено в {first}"))
                }
                None => {
                    defined.insert(id, here);
                }
            }
        }
    }

    // 2. References to rules point to existing rules — in documents and examples.
    let mut sources = walk(root, "md");
    sources.extend(walk(&root.join("examples"), "kenox"));
    for path in &sources {
        let Ok(text) = read(path) else { continue };
        for (line, id) in rule_references(&text) {
            if !defined.contains_key(&id) {
                problems.push(format!(
                    "{}:{line}: ссылка на несуществующее правило [{id}]",
                    rel(root, path)
                ));
            }
        }
    }

    // 3. Grammar: every rule defined once, every used rule defined, every rule used.
    let mut grammar: BTreeMap<String, String> = BTreeMap::new();
    let mut used: BTreeSet<String> = BTreeSet::new();
    for (path, text) in &spec_texts {
        for (start, block) in fenced_blocks(text, "ebnf") {
            for stmt in ebnf_statements(&block) {
                let here = format!("{}:{}", rel(root, path), start + stmt.line);
                match stmt.name {
                    Some(name) => match grammar.get(&name) {
                        Some(first) => problems.push(format!(
                            "{here}: правило грамматики {name} уже определено в {first}"
                        )),
                        None => {
                            grammar.insert(name, here);
                        }
                    },
                    None => problems.push(format!(
                        "{here}: в грамматике ожидалось определение вида «Имя = … ;»"
                    )),
                }
                used.extend(stmt.refs);
            }
        }
    }
    for name in used.iter().filter(|n| !grammar.contains_key(*n)) {
        problems.push(format!(
            "Правило грамматики {name} используется, но не определено"
        ));
    }
    for (name, at) in &grammar {
        if !used.contains(name) && !GRAMMAR_ROOTS.contains(&name.as_str()) {
            problems.push(format!(
                "{at}: правило грамматики {name} нигде не используется"
            ));
        }
    }

    // 4. KENOX examples in the specification and the sketch are written by the sketch.
    let mut example_texts = spec_texts;
    let sketch = root.join("docs").join("SYNTAX_SKETCH.md");
    match read(&sketch) {
        Ok(text) => example_texts.push((sketch, text)),
        Err(e) => problems.push(e),
    }
    let mut examples = 0;
    for (path, text) in &example_texts {
        for (start, block) in fenced_blocks(text, "kenox") {
            examples += 1;
            lint_code(&rel(root, path), &block, start, &mut problems);
        }
    }

    let summary = Summary {
        rules: defined.len(),
        grammar_rules: grammar.len(),
        examples,
    };
    (problems, summary)
}

/// `true` for identifiers like `lex.ident.syntax`: a known prefix and at least
/// one more part; parts consist of lowercase ASCII letters, digits and `-`.
fn is_rule_id(s: &str) -> bool {
    let mut parts = s.split('.');
    let prefix_ok = parts.next().is_some_and(|p| RULE_PREFIXES.contains(&p));
    let rest: Vec<&str> = parts.collect();
    prefix_ok
        && !rest.is_empty()
        && rest.iter().all(|p| {
            !p.is_empty()
                && p.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        })
}

/// Rule definitions `**[id]**` with their line numbers. Bracketed bold text that
/// looks like an identifier but is malformed is reported.
fn rule_definitions(text: &str, problems: &mut Vec<String>, file: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (no, line) in text.lines().enumerate() {
        let mut rest = line;
        while let Some(pos) = rest.find("**[") {
            let after = &rest[pos + 3..];
            let Some(end) = after.find("]**") else { break };
            let id = &after[..end];
            if is_rule_id(id) {
                out.push((no + 1, id.to_string()));
            } else if id.contains('.') && !id.contains(' ') {
                problems.push(format!(
                    "{file}:{}: некорректный идентификатор правила [{id}]",
                    no + 1
                ));
            }
            rest = &after[end + 3..];
        }
    }
    out
}

/// References `[id]` to rules (definitions `**[id]**` count as well).
fn rule_references(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (no, line) in text.lines().enumerate() {
        let mut rest = line;
        while let Some(pos) = rest.find('[') {
            let after = &rest[pos + 1..];
            if let Some(end) = after.find(']') {
                let id = &after[..end];
                if is_rule_id(id) {
                    out.push((no + 1, id.to_string()));
                }
            }
            rest = after;
        }
    }
    out
}

/// Contents of fenced code blocks of the given language, with the line number
/// (1-based) of their first line in the file.
fn fenced_blocks(text: &str, lang: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut current: Option<(usize, Vec<&str>)> = None;
    let fence = format!("```{lang}");
    for (no, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        match current.as_mut() {
            None if trimmed == fence => current = Some((no + 2, Vec::new())),
            None => {}
            Some(_) if trimmed == "```" => {
                let (start, lines) = current.take().unwrap_or_default();
                out.push((start, lines.join("\n")));
            }
            Some((_, lines)) => lines.push(line),
        }
    }
    out
}

/// One `Name = … ;` statement of the grammar.
#[derive(Debug, PartialEq)]
struct Statement {
    name: Option<String>,
    refs: Vec<String>,
    /// Line of the statement start, counted from 0 inside the block.
    line: usize,
}

/// Splits an EBNF block into statements. Quoted terminals, `? prose ?`,
/// `(* comments *)` and code points `U+XXXX` are skipped.
fn ebnf_statements(block: &str) -> Vec<Statement> {
    let c: Vec<char> = block.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    let mut line = 0;
    let mut start: Option<usize> = None;
    let mut names: Vec<String> = Vec::new();
    let mut name: Option<String> = None;
    let mut other_before_eq = false;
    let mut seen_eq = false;
    let skip_to = |i: &mut usize, line: &mut usize, end: &str| {
        let end: Vec<char> = end.chars().collect();
        while *i < c.len() && c[*i..].iter().take(end.len()).ne(end.iter()) {
            if c[*i] == '\n' {
                *line += 1;
            }
            *i += 1;
        }
        *i += end.len();
    };
    while i < c.len() {
        let ch = c[i];
        if ch == '\n' {
            line += 1;
            i += 1;
            continue;
        }
        if ch.is_whitespace() {
            i += 1;
            continue;
        }
        if ch == '(' && c.get(i + 1) == Some(&'*') {
            // Comments do not start a statement.
            i += 2;
            skip_to(&mut i, &mut line, "*)");
            continue;
        }
        start.get_or_insert(line);
        match ch {
            '"' | '\'' | '?' => {
                i += 1;
                skip_to(&mut i, &mut line, &ch.to_string());
                other_before_eq |= !seen_eq;
            }
            'U' if c.get(i + 1) == Some(&'+') => {
                i += 2;
                while i < c.len() && c[i].is_ascii_hexdigit() {
                    i += 1;
                }
                other_before_eq |= !seen_eq;
            }
            '=' if !seen_eq => {
                seen_eq = true;
                if names.len() == 1 && !other_before_eq {
                    name = names.pop();
                }
                i += 1;
            }
            ';' => {
                out.push(Statement {
                    name: if seen_eq { name.take() } else { None },
                    refs: std::mem::take(&mut names),
                    line: start.take().unwrap_or(line),
                });
                name = None;
                seen_eq = false;
                other_before_eq = false;
                i += 1;
            }
            c0 if c0.is_ascii_alphabetic() || c0 == '_' => {
                let begin = i;
                while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == '_') {
                    i += 1;
                }
                names.push(c[begin..i].iter().collect());
            }
            _ => {
                other_before_eq |= !seen_eq;
                i += 1;
            }
        }
    }
    if let Some(line) = start {
        out.push(Statement {
            name: None,
            refs: names,
            line,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_ids() {
        assert!(is_rule_id("lex.ident.syntax"));
        assert!(is_rule_id("lex.nl.match-arm"));
        assert!(!is_rule_id("lex"));
        assert!(!is_rule_id("foo.bar"));
        assert!(!is_rule_id("lex.Ident"));
        assert!(!is_rule_id("self.index"));
    }

    #[test]
    fn definitions_and_references() {
        let text = "**[lex.a.b]** Правило.\nСм. [lex.a.b] и [syn.c].\n**[lex.Bad.x]** плохо";
        let mut problems = Vec::new();
        let defs = rule_definitions(text, &mut problems, "f.md");
        assert_eq!(defs, vec![(1, "lex.a.b".to_string())]);
        assert_eq!(problems.len(), 1);
        let refs: Vec<String> = rule_references(text)
            .into_iter()
            .map(|(_, id)| id)
            .collect();
        assert_eq!(refs, vec!["lex.a.b", "lex.a.b", "syn.c"]);
    }

    #[test]
    fn code_blocks() {
        let text = "текст\n```ebnf\nA = B ;\n```\n```kenox\nval x = 1\n```\n";
        assert_eq!(
            fenced_blocks(text, "ebnf"),
            vec![(3, "A = B ;".to_string())]
        );
        assert_eq!(
            fenced_blocks(text, "kenox"),
            vec![(6, "val x = 1".to_string())]
        );
    }

    #[test]
    fn grammar_statements() {
        let block = "A = \"x\" | B (* комментарий C *) ;\nB = 'a' .. 'z' | ? XID_Start ? | U+000A\n  | C ;\nbroken ;";
        let stmts = ebnf_statements(block);
        assert_eq!(stmts.len(), 3);
        assert_eq!(stmts[0].name.as_deref(), Some("A"));
        assert_eq!(stmts[0].refs, vec!["B"]);
        assert_eq!(stmts[1].name.as_deref(), Some("B"));
        assert_eq!(stmts[1].refs, vec!["C"]);
        assert_eq!(stmts[1].line, 1);
        assert_eq!(stmts[2].name, None);
    }

    #[test]
    fn trailing_comment_is_not_a_statement() {
        let stmts = ebnf_statements("NL = LINE_BREAK ;   (* пояснение *)\n");
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0].name.as_deref(), Some("NL"));
    }

    #[test]
    fn missing_semicolon_is_reported() {
        let stmts = ebnf_statements("A = B");
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0].name, None);
    }
}
