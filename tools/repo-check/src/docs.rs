//! Consistency of the design documents: relative links, numbering of questions
//! (Q-NN), experiments (E-N), decisions (ADR-NNNN) and corpus findings (Р-N),
//! section references (§N.N), required ADR sections and example ids (NN-NN).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::text::{digits_at, is_word, read, rel, walk};

const REQUIRED_ADR_SECTIONS: [&str; 7] = [
    "## Проблема",
    "## Возможные варианты",
    "## Рекомендуемое решение",
    "## Причины выбора",
    "## Последствия",
    "## Риски",
    "## Статус",
];

/// Counts printed in the report.
#[derive(Debug, Default)]
pub struct Summary {
    pub files: usize,
    pub questions: usize,
    pub findings: usize,
    pub examples: usize,
}

/// All document problems.
pub fn check(root: &Path) -> Vec<String> {
    check_with_summary(root).0
}

/// All document problems plus counts for the report.
pub fn check_with_summary(root: &Path) -> (Vec<String>, Summary) {
    let mut problems = Vec::new();
    let docs = root.join("docs");
    let adr_dir = docs.join("ADR");

    let mut texts: Vec<(PathBuf, String)> = Vec::new();
    for path in walk(root, "md") {
        match read(&path) {
            Ok(t) => texts.push((path, t)),
            Err(e) => problems.push(e),
        }
    }
    let mut special = |name: &Path| -> String {
        read(name).unwrap_or_else(|e| {
            problems.push(e);
            String::new()
        })
    };
    let questions = special(&docs.join("OPEN_QUESTIONS.md"));
    let adr_index = special(&adr_dir.join("README.md"));
    let review = special(&docs.join("CORPUS_REVIEW.md"));
    let architecture = special(&docs.join("ARCHITECTURE.md"));
    let audit = special(&docs.join("ARCHITECTURE_REVIEW.md"));
    let examples_index = special(&root.join("examples").join("README.md"));

    // 1. Relative links resolve.
    for (path, text) in &texts {
        let base = path.parent().unwrap_or(root);
        for target in markdown_links(text) {
            if target.starts_with("http://")
                || target.starts_with("https://")
                || target.starts_with("mailto:")
                || target.starts_with('#')
            {
                continue;
            }
            let file_part = target.split('#').next().unwrap_or("");
            if !file_part.is_empty() && !base.join(file_part).exists() {
                problems.push(format!("{}: битая ссылка → {target}", rel(root, path)));
            }
        }
    }

    // 2. Questions Q-NN: every reference defined, no gaps, no duplicates.
    let mut defined_q: BTreeMap<u32, usize> = BTreeMap::new();
    for line in questions.lines() {
        let id = if let Some(rest) = line.strip_prefix("### Q-") {
            numbered(rest, 2, ".")
        } else if let Some(rest) = line.strip_prefix("| Q-") {
            numbered(rest, 2, " |")
        } else {
            None
        };
        if let Some(n) = id {
            *defined_q.entry(n).or_default() += 1;
        }
    }
    let mut referenced_q = BTreeSet::new();
    for (_, text) in &texts {
        referenced_q.extend(question_refs(text));
    }
    for q in referenced_q.iter().filter(|q| !defined_q.contains_key(q)) {
        problems.push(format!(
            "Вопрос Q-{q:02} упоминается, но не описан в OPEN_QUESTIONS.md"
        ));
    }
    if let Some(&max) = defined_q.keys().max() {
        for q in (1..=max).filter(|q| !defined_q.contains_key(q)) {
            problems.push(format!("Пропуск в нумерации вопросов: Q-{q:02}"));
        }
    }
    for (q, _) in defined_q.iter().filter(|(_, n)| **n > 1) {
        problems.push(format!("Вопрос Q-{q:02} описан в OPEN_QUESTIONS.md дважды"));
    }

    // 3. Experiments E-N.
    let defined_e: BTreeSet<u32> = questions
        .lines()
        .filter_map(|l| l.strip_prefix("| E-").and_then(|r| numbered(r, 1, " |")))
        .collect();
    let mut referenced_e = BTreeSet::new();
    for (_, text) in &texts {
        referenced_e.extend(experiment_refs(text));
    }
    for e in referenced_e.difference(&defined_e) {
        problems.push(format!(
            "Эксперимент E-{e} упоминается, но не описан в OPEN_QUESTIONS.md"
        ));
    }

    // 4. ADR-NNNN: referenced ones are in the index; drafted ones have files.
    let mut listed = BTreeSet::new();
    let mut drafted = BTreeSet::new();
    for line in adr_index.lines() {
        if let Some(rest) = line.strip_prefix("| [")
            && let Some(n) = numbered(rest, 4, "](")
        {
            listed.insert(n);
            drafted.insert(n);
        } else if let Some(rest) = line.strip_prefix("| ")
            && let Some(n) = numbered(rest, 4, " |")
        {
            listed.insert(n);
        }
    }
    let mut referenced_adr = BTreeSet::new();
    for (_, text) in &texts {
        referenced_adr.extend(adr_refs(text));
    }
    for n in referenced_adr.difference(&listed) {
        problems.push(format!(
            "ADR-{n:04} упоминается, но его нет в docs/ADR/README.md"
        ));
    }
    let adr_files: Vec<PathBuf> = walk(&adr_dir, "md")
        .into_iter()
        .filter(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with('0'))
        })
        .collect();
    let file_numbers: BTreeSet<u32> = adr_files
        .iter()
        .filter_map(|p| p.file_name()?.to_string_lossy().get(..4)?.parse().ok())
        .collect();
    for n in drafted.difference(&file_numbers) {
        problems.push(format!(
            "ADR-{n:04} отмечен как подготовленный, но файла нет"
        ));
    }
    for n in file_numbers.difference(&drafted) {
        problems.push(format!(
            "Файл ADR-{n:04} есть, но в оглавлении он не отмечен как подготовленный"
        ));
    }

    // 5. Section references point to headings of the two architecture documents.
    let mut headings = BTreeSet::new();
    for text in [&architecture, &audit] {
        headings.extend(numbered_headings(text));
    }
    let mut section_refs = BTreeSet::new();
    for (_, text) in &texts {
        section_refs.extend(section_refs_in(text));
    }
    for s in section_refs.difference(&headings) {
        problems.push(format!(
            "Ссылка §{s}: такого раздела нет ни в ARCHITECTURE.md, ни в аудите"
        ));
    }

    // 6. Every ADR has the required sections.
    for path in &adr_files {
        let text = read(path).unwrap_or_default();
        for heading in REQUIRED_ADR_SECTIONS {
            if !text.contains(heading) {
                problems.push(format!("{}: нет раздела «{heading}»", rel(root, path)));
            }
        }
    }

    // 7. Corpus findings Р-N: referenced in documents or examples, defined in the review.
    let defined_r: BTreeSet<u32> = review
        .lines()
        .filter_map(|l| l.strip_prefix("| Р-").and_then(|r| numbered(r, 0, " |")))
        .collect();
    let mut referenced_r = BTreeSet::new();
    for (_, text) in &texts {
        referenced_r.extend(finding_refs(text));
    }
    for path in walk(&root.join("examples"), "kenox") {
        referenced_r.extend(finding_refs(&read(&path).unwrap_or_default()));
    }
    for r in referenced_r.difference(&defined_r) {
        problems.push(format!(
            "Пункт Р-{r} упоминается, но не описан в CORPUS_REVIEW.md"
        ));
    }
    if let Some(&max) = defined_r.iter().max() {
        for r in (1..=max).filter(|r| !defined_r.contains(r)) {
            problems.push(format!("Пропуск в нумерации пунктов: Р-{r}"));
        }
    }

    // 8. Example ids mentioned in docs/ exist in the examples index.
    let indexed: BTreeSet<String> = examples_index
        .lines()
        .filter_map(|l| l.strip_prefix("| "))
        .filter_map(|r| {
            let id: String = r.chars().take(5).collect();
            (is_example_id(&id) && r[id.len()..].starts_with(" |")).then_some(id)
        })
        .collect();
    let mut mentioned = BTreeSet::new();
    let spec_dir = root.join("spec");
    for (path, text) in &texts {
        let parent = path.parent();
        if parent == Some(docs.as_path())
            || parent == Some(adr_dir.as_path())
            || parent == Some(spec_dir.as_path())
        {
            mentioned.extend(example_ids_in(text));
        }
    }
    for id in mentioned.difference(&indexed) {
        problems.push(format!(
            "Пример {id} упоминается в документах, но его нет в examples/README.md"
        ));
    }

    problems.dedup();
    let summary = Summary {
        files: texts.len(),
        questions: defined_q.len(),
        findings: defined_r.len(),
        examples: indexed.len(),
    };
    (problems, summary)
}

/// Targets of `[text](target)` links; targets with spaces are ignored.
fn markdown_links(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(pos) = rest.find("](") {
        let after = &rest[pos + 2..];
        if let Some(end) = after.find(')') {
            let target = &after[..end];
            if !target.is_empty() && !target.chars().any(char::is_whitespace) {
                out.push(target.to_string());
            }
        }
        rest = after;
    }
    out
}

/// Parses `digits` + `suffix` at the start of `s`. `width` = 0 means any number of digits.
fn numbered(s: &str, width: usize, suffix: &str) -> Option<u32> {
    let digits: String = s.chars().take_while(char::is_ascii_digit).collect();
    let width_ok = if width == 0 {
        !digits.is_empty()
    } else {
        digits.len() == width
    };
    (width_ok && s[digits.len()..].starts_with(suffix)).then(|| digits.parse().ok())?
}

/// `Q-NN` references (exactly two digits).
fn question_refs(text: &str) -> Vec<u32> {
    let c: Vec<char> = text.chars().collect();
    (0..c.len().saturating_sub(3))
        .filter(|&i| {
            c[i] == 'Q' && c[i + 1] == '-' && c[i + 2].is_ascii_digit() && c[i + 3].is_ascii_digit()
        })
        .filter_map(|i| c[i + 2..i + 4].iter().collect::<String>().parse().ok())
        .collect()
}

/// `E-N` references as whole words (one digit).
fn experiment_refs(text: &str) -> Vec<u32> {
    let c: Vec<char> = text.chars().collect();
    (0..c.len().saturating_sub(2))
        .filter(|&i| {
            c[i] == 'E'
                && c[i + 1] == '-'
                && c[i + 2].is_ascii_digit()
                && (i == 0 || !is_word(c[i - 1]))
                && c.get(i + 3).is_none_or(|&n| !is_word(n))
        })
        .filter_map(|i| c[i + 2].to_digit(10))
        .collect()
}

/// `ADR-NNNN` references.
fn adr_refs(text: &str) -> Vec<u32> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(pos) = rest.find("ADR-") {
        rest = &rest[pos + 4..];
        let digits: String = rest
            .chars()
            .take(4)
            .take_while(char::is_ascii_digit)
            .collect();
        if digits.len() == 4 {
            out.extend(digits.parse::<u32>().ok());
        }
    }
    out
}

/// `Р-N` references (Cyrillic «Р»).
fn finding_refs(text: &str) -> Vec<u32> {
    let c: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    for i in 0..c.len().saturating_sub(2) {
        if c[i] == 'Р' && c[i + 1] == '-' {
            let (digits, _) = digits_at(&c, i + 2);
            out.extend(digits.parse::<u32>().ok());
        }
    }
    out
}

/// Numbers of headings like `## 3. Title` or `### 3.1 Title`.
fn numbered_headings(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let hashes = line.chars().take_while(|&c| c == '#').count();
            if !(2..=4).contains(&hashes) {
                return None;
            }
            let rest: Vec<char> = line[hashes..].strip_prefix(' ')?.chars().collect();
            let (number, end) = section_number(&rest, 0)?;
            matches!(rest.get(end), Some(' ' | '.')).then_some(number)
        })
        .collect()
}

/// `§3` or `§3.4` references.
fn section_refs_in(text: &str) -> Vec<String> {
    let c: Vec<char> = text.chars().collect();
    (0..c.len())
        .filter(|&i| c[i] == '§')
        .filter_map(|i| section_number(&c, i + 1).map(|(n, _)| n))
        .collect()
}

/// Reads `N` or `N.M` at `i`; a trailing dot without digits is not part of the number.
fn section_number(c: &[char], i: usize) -> Option<(String, usize)> {
    let (major, end) = digits_at(c, i);
    if major.is_empty() {
        return None;
    }
    if c.get(end) == Some(&'.') && c.get(end + 1).is_some_and(char::is_ascii_digit) {
        let (minor, end2) = digits_at(c, end + 1);
        return Some((format!("{major}.{minor}"), end2));
    }
    Some((major, end))
}

fn is_example_id(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    c.len() == 5 && c[2] == '-' && [0, 1, 3, 4].iter().all(|&i| c[i].is_ascii_digit())
}

/// Example ids `NN-NN` that are not part of dates, versions or longer numbers.
fn example_ids_in(text: &str) -> Vec<String> {
    let c: Vec<char> = text.chars().collect();
    (0..c.len().saturating_sub(4))
        .filter(|&i| {
            let id: String = c[i..i + 5].iter().collect();
            is_example_id(&id)
                && (i == 0 || !(is_word(c[i - 1]) || c[i - 1] == '-' || c[i - 1] == '.'))
                && c.get(i + 5)
                    .is_none_or(|&n| !(n.is_ascii_digit() || n == '-'))
        })
        .map(|i| c[i..i + 5].iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let t =
            "см. [ADR](ADR/0001-x.md#a), [сайт](https://x.org), [пусто](с пробелом), [якорь](#b)";
        assert_eq!(
            markdown_links(t),
            vec!["ADR/0001-x.md#a", "https://x.org", "#b"]
        );
    }

    #[test]
    fn numbering() {
        assert_eq!(numbered("07. Текст", 2, "."), Some(7));
        assert_eq!(numbered("7. Текст", 2, "."), None);
        assert_eq!(numbered("12 | x", 0, " |"), Some(12));
        assert_eq!(question_refs("Q-05, Q-40 и Q-7"), vec![5, 40]);
        assert_eq!(experiment_refs("E-1, CODE-2, E-3x, (E-8)"), vec![1, 8]);
        assert_eq!(adr_refs("ADR-0003 и ADR-12"), vec![3]);
        assert_eq!(finding_refs("Р-3, Р-10 и P-4 (латиница)"), vec![3, 10]);
    }

    #[test]
    fn sections() {
        let doc =
            "## 3. Границы\n### 3.1 Опоры\n#### 4.2. Деталь\n# 9. Слишком крупный\n## Без номера";
        assert_eq!(numbered_headings(doc), vec!["3", "3.1", "4.2"]);
        assert_eq!(
            section_refs_in("аудит §3.4, §5.2 (M2) и §4."),
            vec!["3.4", "5.2", "4"]
        );
    }

    #[test]
    fn example_ids() {
        let t = "примеры 09-05 и 12-02; дата 2026-11-05; x86-64; версия 1.09-01";
        assert_eq!(example_ids_in(t), vec!["09-05", "12-02"]);
    }
}
