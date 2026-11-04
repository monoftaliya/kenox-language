# CLAUDE.md — правила для сессий Claude

Этот файл читается в начале каждой сессии. Он не пересказывает документы, а указывает на них.

## Проект

KENOX — язык программирования на этапе проектирования. Где что лежит:

| Что | Где |
|---|---|
| Текущее состояние, что дальше | [STATUS.md](STATUS.md) |
| Архитектура (главный документ) | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) |
| Решения | [docs/ADR/](docs/ADR/README.md) |
| План и критерии этапов | [docs/ROADMAP.md](docs/ROADMAP.md) |
| Открытые вопросы | [docs/OPEN_QUESTIONS.md](docs/OPEN_QUESTIONS.md) |
| Синтаксис | [docs/SYNTAX_SKETCH.md](docs/SYNTAX_SKETCH.md), корпус — [examples/](examples/README.md) |
| Порядок работы с Git, CI и проверкой | [docs/WORKFLOW.md](docs/WORKFLOW.md) |

## В начале сессии

1. Прочитать [STATUS.md](STATUS.md) и документы, относящиеся к задаче.
2. Убедиться, что `main` зелёный: `cargo test`.

## Запреты этапа 0

- Не писать код компилятора, runtime, лексер и парсер до точки решения G0 ([ROADMAP.md](docs/ROADMAP.md), этап 0). Разрешены инструменты репозитория в `tools/` и одноразовые прототипы вне `main`.
- Не принимать спорные решения о языке самостоятельно. Вместо этого — ADR со статусом «требует утверждения» и вопрос в OPEN_QUESTIONS.md. Решения, меняющие язык, утверждает инициатор проекта.

## Как вносить изменения

Порядок — [docs/WORKFLOW.md](docs/WORKFLOW.md). Кратко:

- Одно изменение — одна ветка `claude/<тема>` и один pull request в `main` по шаблону. В `main` напрямую не коммитить.
- Перед pull request:
  ```
  cargo fmt --all
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test
  ```
- Каждый pull request обновляет [STATUS.md](STATUS.md).
- Связанное меняется вместе: изменение синтаксиса правит эскиз, примеры и CORPUS_REVIEW.md в одном pull request; новое решение — ADR, его строку в `docs/ADR/README.md` и статус в ARCHITECTURE.md.
- Нумерация без пропусков: Q-NN (OPEN_QUESTIONS.md), Р-N (CORPUS_REVIEW.md, буква кириллическая), E-N, ADR-NNNN. `cargo test` это проверяет.

## Соглашения

- Документы — на русском. Статусы ADR и шаблон — [docs/ADR/README.md](docs/ADR/README.md). Факты, оценки и цели помечаются: [факт], [оценка], [цель].
- Rust: версия закреплена в `rust-toolchain.toml`; edition 2024; внешние зависимости — только через ADR (ADR-0027). Идентификаторы и комментарии в коде — на английском; сообщения для пользователя — на русском.
- Вывод инструментов не зависит от ОС: окончания строк LF, пути печатаются относительно корня репозитория и с `/`.
- clang ищется так: переменная `KENOX_CLANG` → `clang` в `PATH` → стандартная папка LLVM на Windows.

## Окружение Claude

- Облачная сессия — Linux x86_64. Доступ к crates.io и static.rust-lang.org может быть закрыт: поэтому инструменты пишутся без внешних зависимостей. Если закреплённая версия Rust не скачивается, а установленная `stable` с ней совпадает, запускать с `RUSTUP_TOOLCHAIN=stable`.
- Пользователь работает на Windows 11: всё, что он запускает, должно работать в PowerShell.
