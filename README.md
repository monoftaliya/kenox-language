# KENOX

*KENOX is a programming language in the design stage. The documentation is in Russian.*

KENOX — компилируемый язык программирования, который соединяет безопасность памяти и предсказуемую производительность Rust с удобством Kotlin. Ниша версии 1.0: инструменты разработчика, серверы, системные утилиты, библиотеки и CLI, вычисления на CPU, интеграция с другими языками через C ABI.

> **Статус: этап 0 — проектирование.** Компилятора пока нет. В репозитории — архитектура, архитектурные решения (ADR), эскиз синтаксиса и корпус из 44 примеров, на которых проверяется язык. Текущее состояние — [STATUS.md](STATUS.md).

## Как выглядит KENOX

```kenox
import std.fs
import std.error.AnyError

enum PaymentMethod {
    Cash
    Card(number: String, holder: String)
    Transfer(iban: String)

    fun fee(self, amount: Int): Int = match self {
        .Cash -> 0
        .Card(_, _) -> amount * 2 / 100
        .Transfer(_) -> 30
    }
}

fun countLines(path: str): Result<Int, AnyError> {
    val text = try fs.readText(path).context("не удалось прочитать {path}")
    return Ok(text.lines().count())
}
```

Больше — в [корпусе примеров](examples/README.md) и [эскизе синтаксиса](docs/SYNTAX_SKETCH.md).

## Документы

| Документ | Что в нём |
|---|---|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Главный документ: устройство языка и компилятора |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Этапы, вехи и критерии готовности |
| [docs/ADR/](docs/ADR/README.md) | Архитектурные решения |
| [docs/OPEN_QUESTIONS.md](docs/OPEN_QUESTIONS.md) | Открытые вопросы |
| [docs/SYNTAX_SKETCH.md](docs/SYNTAX_SKETCH.md) | Эскиз синтаксиса |
| [examples/](examples/README.md) | Корпус примеров |
| [docs/CORPUS_REVIEW.md](docs/CORPUS_REVIEW.md) | Анализ корпуса: найденные проблемы и решения |
| [docs/ARCHITECTURE_REVIEW.md](docs/ARCHITECTURE_REVIEW.md) | Исходный аудит архитектуры |
| [docs/WORKFLOW.md](docs/WORKFLOW.md) | Как устроена работа: установка, проверка, pull request |

## Проверка

```
cargo test
```

Команда проверяет документы (ссылки, нумерацию вопросов и решений), корпус примеров и C-код примеров — одинаково на Windows, Linux и macOS. Установка инструментов и порядок работы — в [docs/WORKFLOW.md](docs/WORKFLOW.md).

## Лицензия

На ваш выбор — одна из двух лицензий:

- Apache License 2.0 — [LICENSE-APACHE](LICENSE-APACHE);
- MIT — [LICENSE-MIT](LICENSE-MIT).

Если вы явно не указали иное, любой вклад, намеренно переданный вами для включения в проект, как это определено в лицензии Apache-2.0, распространяется на тех же двух условиях, без каких-либо дополнительных условий.
