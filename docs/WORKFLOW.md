# KENOX — как устроена работа

> Версия 0.1 · 5 ноября 2026 · Решение — [ADR-0027](ADR/0027-ai-assisted-development.md)
>
> Единственная инструкция по Git, GitHub, CI и проверке изменений. README.md, CLAUDE.md и шаблон pull request ссылаются сюда и её не пересказывают. Если что-то здесь расходится с практикой, исправляется этот файл.

## 1. Коротко

```
Claude:   ветка claude/<тема> → изменения → pull request
                                              │
GitHub:   проверки CI на Windows, Linux и macOS
                                              │
Вы:       описание и три зелёные галочки → (по желанию) cargo test у себя → Squash and merge
                                              │
                                            main
```

Три правила:
1. В `main` попадает только pull request с зелёными проверками. Напрямую в `main` не пишет никто — кроме самого первого коммита.
2. Один pull request — одна цель. Описание — по шаблону: цель, что изменилось, как проверить, что не сделано.
3. Решения о языке утверждаете вы. Pull request с таким решением ссылается на ADR и вопрос из OPEN_QUESTIONS.md.

## 2. Установка на Windows 11 (один раз)

Все команды — в PowerShell: меню «Пуск» → «Терминал».

1. **Git:**
   ```powershell
   winget install --id Git.Git -e
   ```
2. **Rust.** Скачайте `rustup-init.exe` с сайта [rustup.rs](https://rustup.rs) и запустите. Если установщик сообщит, что не хватает компонентов Visual Studio, выберите пункт 1 — `Quick install via the Visual Studio Community installer`: он поставит компилятор и библиотеки Microsoft, без которых Rust на Windows не собирает программы. Visual Studio Community бесплатна для частных лиц и проектов с открытым кодом ([документация rustup](https://rust-lang.github.io/rustup/installation/windows-msvc.html)). Затем согласитесь на установку по умолчанию.
3. **LLVM (clang):**
   ```powershell
   winget install --id LLVM.LLVM -e
   ```
   Установщик обычно не добавляет clang в `PATH`. Это не мешает: проверки сами находят clang в папке `C:\Program Files\LLVM\bin`. Другое расположение можно указать в переменной окружения `KENOX_CLANG`.
4. **Проверка установки.** Закройте и снова откройте терминал, затем:
   ```powershell
   git --version
   cargo --version
   & "C:\Program Files\LLVM\bin\clang.exe" --version
   ```
   Каждая команда должна напечатать номер версии.

Python, bash и другие инструменты не нужны.

## 3. Получить репозиторий и проверить

```powershell
cd $HOME
git clone https://github.com/<ваш-логин>/kenox-language.git
cd kenox-language
cargo test
```

Первый `cargo test` идёт дольше обычного: rustup скачивает версию Rust, закреплённую в `rust-toolchain.toml`. Это происходит один раз.

**Ожидаемый результат** — в конце несколько строк `test result: ok. … 0 failed`. Если есть `FAILED`, скопируйте вывод целиком и пришлите Claude.

Что проверяет `cargo test`:

| Проверка | Что именно |
|---|---|
| Документы | Ссылки между файлами; нумерация вопросов Q, экспериментов E, решений ADR и пунктов Р без пропусков и висячих ссылок; обязательные разделы ADR; номера примеров |
| Корпус примеров | Нет конструкций других языков (`fn`, `let`, `=>`, `when`, `$` в строках…); у каждого файла есть заголовок |
| C-код примеров | Компилируется clang без единого предупреждения |
| Сам инструмент проверки | Собственные тесты: проверки действительно находят намеренно внесённые ошибки |

Подробный отчёт с метриками корпуса: `cargo run -p kenox-repo-check`.

## 4. Как принять pull request

1. На странице репозитория — вкладка **Pull requests** → нужный запрос.
2. Внизу — блок проверок. Нужны три зелёные галочки: `ubuntu-latest`, `windows-latest`, `macos-latest`. Жёлтый кружок — проверки ещё идут (обычно несколько минут). Красный крестик — см. раздел 6.
3. Прочитайте описание: цель, что изменилось, как проверить, что не сделано.
4. По желанию проверьте у себя. Имя ветки указано вверху страницы запроса, например `claude/spec-lexer`:
   ```powershell
   cd $HOME\kenox-language
   git fetch origin
   git switch claude/spec-lexer
   cargo test
   git switch main
   ```
5. Если всё устраивает — **Squash and merge** → **Confirm squash and merge**, затем **Delete branch**. Все коммиты запроса станут одним коммитом в `main`.
6. Если нужны правки — напишите комментарий в pull request или скажите Claude в чате. Исправления придут в ту же ветку, проверки запустятся заново.

## 5. Обновить свою копию

После слияния:

```powershell
cd $HOME\kenox-language
git switch main
git pull
```

## 6. Если проверка не прошла

- Не сливайте pull request.
- Нажмите на красную проверку — откроется журнал с шагом, где произошла ошибка.
- Пришлите Claude ссылку на pull request. Исправление придёт в ту же ветку.

## 7. Что делает CI

Файл — [.github/workflows/ci.yml](../.github/workflows/ci.yml). Каждый pull request и каждое изменение `main` проверяются на Windows, Linux и macOS:

| Шаг | Команда | Смысл |
|---|---|---|
| Rust | `rustup toolchain install` | Та же версия, что у вас и у Claude (`rust-toolchain.toml`) |
| Оформление | `cargo fmt --all --check` | Код на Rust оформлен единообразно |
| Clippy | `cargo clippy --workspace --all-targets --locked -- -D warnings` | Анализатор Rust не нашёл замечаний |
| Проверки | `cargo test --workspace --locked` | То же, что `cargo test` у вас |
| Отчёт | `cargo run -p kenox-repo-check` | Метрики и подробности; на результат не влияет |

Вам достаточно `cargo test`. Оформление и Clippy касаются только кода на Rust — за них отвечает Claude.

## 8. Правила для изменений

- Ветки Claude называются `claude/<тема>`.
- Каждый pull request обновляет [STATUS.md](../STATUS.md).
- Связанные документы меняются вместе: синтаксис — эскиз, примеры и CORPUS_REVIEW.md; новое решение — ADR, оглавление ADR и ARCHITECTURE.md. Подробнее — [CLAUDE.md](../CLAUDE.md).
- Окончания строк — LF. Это обеспечивает `.gitattributes`; настройку Git `core.autocrlf` менять не нужно.
- Версия Rust меняется только отдельным pull request.

## 9. Первая загрузка на GitHub

**Вариант А — выкладывает Claude (рекомендуется).**
1. Создайте на GitHub пустой публичный репозиторий `kenox-language` — без README, лицензии и `.gitignore`.
2. Подключите GitHub к Claude в настройках claude.ai (раздел Connectors) и разрешите доступ к этому репозиторию.
3. Пришлите Claude ссылку на репозиторий.

Claude создаст первый коммит в `main` (README и лицензии), а всё остальное откроет первым pull request — его вы примете по разделу 4.

**Вариант Б — вручную.** Распакуйте архив в `$HOME\kenox-language` и выполните:

```powershell
cd $HOME\kenox-language
git config --global user.name "Ваше имя"
git config --global user.email "ваша@почта"
git init -b main
git add .
git commit -m "Шаг 0.1: репозиторий, документы, корпус примеров, CI"
git remote add origin https://github.com/<ваш-логин>/kenox-language.git
git push -u origin main
```

При первом `git push` откроется окно входа в GitHub. В варианте Б Claude не сможет открывать pull request сам — изменения будут приходить архивами, поэтому вариант А удобнее.

## 10. Настройка репозитория (один раз, по желанию)

После первого прогона CI:

- **Только squash.** Settings → General → раздел Pull Requests: оставьте включённым только Allow squash merging. Тогда кнопка слияния всегда будет Squash and merge ([документация GitHub](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/configuring-commit-squashing-for-pull-requests)).
- **Защита `main`.** Settings → Rulesets → New ruleset → New branch ruleset. Статус — Active; цель — ветка по умолчанию; включите Require status checks before merging и добавьте проверки `ubuntu-latest`, `windows-latest`, `macos-latest`; затем Create ([документация GitHub](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository)). Тогда GitHub сам не даст слить запрос с красными проверками.

## 11. Частые проблемы

| Симптом | Что делать |
|---|---|
| `cargo` или `git` не найдены | Закройте и снова откройте терминал после установки |
| `link.exe not found` при `cargo test` | Не установлены компоненты Visual Studio — запустите `rustup-init.exe` ещё раз (раздел 2, шаг 2) |
| `clang не найден` | Установите LLVM (раздел 2, шаг 3) или укажите путь в `KENOX_CLANG` |
| Первый `cargo test` долго скачивает | Это загрузка закреплённой версии Rust; повторно не происходит |
| В `git status` изменены все файлы | Проверьте, что вы в папке репозитория; окончания строк выравнивает `.gitattributes` |
