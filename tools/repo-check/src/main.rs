//! `cargo run -p kenox-repo-check` — full report: problems of every check plus
//! corpus metrics. The same checks run in `cargo test`.

use std::process::ExitCode;

use kenox_repo_check::{cexamples, corpus, docs, repo_root};

fn main() -> ExitCode {
    let root = repo_root();
    let mut total = 0;

    println!("== Документы ==");
    let (problems, summary) = docs::check_with_summary(&root);
    println!(
        "файлов .md: {}; вопросов Q: {}; пунктов Р: {}; примеров в оглавлении: {}",
        summary.files, summary.questions, summary.findings, summary.examples
    );
    total += print_problems(&problems);

    println!("\n== Корпус примеров ==");
    let report = corpus::check(&root);
    for (name, value) in &report.metrics {
        println!("{name:<28} {value}");
    }
    total += print_problems(&report.problems);

    println!("\n== C-код примеров ==");
    match cexamples::find_clang() {
        Ok(clang) => println!("{} ({})", clang.version, clang.program.display()),
        Err(e) => println!("{e}"),
    }
    total += print_problems(&cexamples::check(&root));

    if total == 0 {
        println!("\nПроблем не найдено.");
        ExitCode::SUCCESS
    } else {
        println!("\nНайдено проблем: {total}");
        ExitCode::FAILURE
    }
}

fn print_problems(problems: &[String]) -> usize {
    if problems.is_empty() {
        println!("проблем нет");
    }
    for p in problems {
        println!("ПРОБЛЕМА: {p}");
    }
    problems.len()
}
