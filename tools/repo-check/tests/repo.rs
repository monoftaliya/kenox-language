//! `cargo test` runs these checks on the whole repository.
//! Full report with metrics: `cargo run -p kenox-repo-check`.

use kenox_repo_check::{cexamples, corpus, docs, repo_root, spec};

fn assert_clean(what: &str, problems: &[String]) {
    assert!(
        problems.is_empty(),
        "{what}: найдено проблем — {}\n\n{}\n",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn documents_are_consistent() {
    assert_clean("Документы", &docs::check(&repo_root()));
}

#[test]
fn specification_is_consistent() {
    assert_clean("Спецификация", &spec::check(&repo_root()));
}

#[test]
fn corpus_follows_the_syntax_sketch() {
    assert_clean("Корпус примеров", &corpus::check(&repo_root()).problems);
}

#[test]
fn c_examples_compile_without_warnings() {
    assert_clean("C-код примеров", &cexamples::check(&repo_root()));
}
