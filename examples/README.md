# Корпус примеров KENOX

> Этап 0, шаг 0.2 (эксперимент E-1) · 5 ноября 2026

44 небольших примера по двенадцати темам. Их задача — проверить удобство и непротиворечивость языка до того, как появится компилятор. Синтаксис — по [эскизу](../docs/SYNTAX_SKETCH.md); он не утверждён. Результаты анализа — в [CORPUS_REVIEW.md](../docs/CORPUS_REVIEW.md).

## Обозначения в примерах

| Пометка | Значение |
|---|---|
| `// ✗ …` | Код, который компилятор **обязан отвергнуть**. Следующая строка объясняет почему. Без таких строк файл должен компилироваться |
| `// ⚠ Р-N: …` | Неудобство или противоречие, найденное при написании примера. Разбор и решение — в CORPUS_REVIEW.md под номером Р-N |
| Остальные комментарии | Пояснения: что происходит и почему |

## Содержание

| № | Файл | Что проверяет |
|---|---|---|
| 01-01 | [01-basics/01_bindings.kenox](01-basics/01_bindings.kenox) | `val`/`var`, вывод типов, константы, блок-выражение `do` |
| 01-02 | [01-basics/02_numbers.kenox](01-basics/02_numbers.kenox) | Ширина целых, отсутствие неявных преобразований, переполнение |
| 01-03 | [01-basics/03_text.kenox](01-basics/03_text.kenox) | `String` и `str`, интерполяция, сырые строки, байты и символы |
| 02-01 | [02-functions/01_basics.kenox](02-functions/01_basics.kenox) | Функции-выражения, значения по умолчанию, именованные аргументы, `Never` |
| 02-02 | [02-functions/02_parameter_modes.kenox](02-functions/02_parameter_modes.kenox) | Заимствование, `inout`, `consume`; что видно на месте вызова |
| 02-03 | [02-functions/03_closures.kenox](02-functions/03_closures.kenox) | Trailing-лямбды, `it`, невыходящие и выходящие замыкания |
| 03-01 | [03-structs-enums/01_structs.kenox](03-structs-enums/01_structs.kenox) | Методы, статические функции, приватные поля, проверяющий конструктор |
| 03-02 | [03-structs-enums/02_enums.kenox](03-structs-enums/02_enums.kenox) | Перечисления с данными вместо иерархии классов |
| 03-03 | [03-structs-enums/03_newtypes_tuples_derive.kenox](03-structs-enums/03_newtypes_tuples_derive.kenox) | Типы-обёртки, кортежи, `@derive`, явный `clone` |
| 04-01 | [04-traits/01_traits.kenox](04-traits/01_traits.kenox) | Трейты, методы по умолчанию, статические функции трейта |
| 04-02 | [04-traits/02_composition_delegation.kenox](04-traits/02_composition_delegation.kenox) | Композиция и делегирование `by` вместо наследования |
| 04-03 | [04-traits/03_dynamic_dispatch.kenox](04-traits/03_dynamic_dispatch.kenox) | `Box<dyn Trait>`, совместимость трейта с `dyn` |
| 04-04 | [04-traits/04_extensions.kenox](04-traits/04_extensions.kenox) | `extend` для чужих типов, `some`, приоритет методов |
| 05-01 | [05-generics/01_generic_functions.kenox](05-generics/01_generic_functions.kenox) | Ограничения, `where`, результат-заимствование `ref T` |
| 05-02 | [05-generics/02_generic_types.kenox](05-generics/02_generic_types.kenox) | Обобщённые структуры и перечисления, условные методы |
| 05-03 | [05-generics/03_iterators.kenox](05-generics/03_iterators.kenox) | Свой итератор, ассоциированные типы, `some Iterator<T>` |
| 05-04 | [05-generics/04_fixed_size_simd.kenox](05-generics/04_fixed_size_simd.kenox) | `Array<T, N>`, константные параметры, SIMD |
| 06-01 | [06-errors/01_result_try.kenox](06-errors/01_result_try.kenox) | `Result`, `try`, преобразование ошибок через `From` |
| 06-02 | [06-errors/02_any_error_context.kenox](06-errors/02_any_error_context.kenox) | `AnyError`, контекст ошибок, `main` с результатом |
| 06-03 | [06-errors/03_panics.kenox](06-errors/03_panics.kenox) | Граница между паникой и ошибкой |
| 07-01 | [07-collections/01_list.kenox](07-collections/01_list.kenox) | `List`, срезы, изменяемый обход, явный `ref` для элемента |
| 07-02 | [07-collections/02_map_set.kenox](07-collections/02_map_set.kenox) | `Map`, `Set`, литералы, `in` |
| 07-03 | [07-collections/03_iterator_chains.kenox](07-collections/03_iterator_chains.kenox) | Ленивые цепочки с заимствующими замыканиями |
| 08-01 | [08-patterns/01_match.kenox](08-patterns/01_match.kenox) | Полнота, условия, альтернативы, диапазоны |
| 08-02 | [08-patterns/02_destructuring.kenox](08-patterns/02_destructuring.kenox) | Разбор структур и кортежей, режимы привязок |
| 08-03 | [08-patterns/03_is_and_if_val.kenox](08-patterns/03_is_and_if_val.kenox) | `is`, `if val`, `while val` |
| 08-04 | [08-patterns/04_tokenizer.kenox](08-patterns/04_tokenizer.kenox) | Лексер: `match` по символам, свой `Iterator` |
| 09-01 | [09-memory/01_move_clone.kenox](09-memory/01_move_clone.kenox) | Перемещение, `Copy`, `clone`, `ref`, частичное перемещение |
| 09-02 | [09-memory/02_borrow_rules.kenox](09-memory/02_borrow_rules.kenox) | Правила заимствований, двухфазные заимствования |
| 09-03 | [09-memory/03_returned_borrows.kenox](09-memory/03_returned_borrows.kenox) | Возврат заимствований, `@borrows` |
| 09-04 | [09-memory/04_raii.kenox](09-memory/04_raii.kenox) | `Drop`, порядок уничтожения, явное `close` |
| 09-05 | [09-memory/05_shared_weak.kenox](09-memory/05_shared_weak.kenox) | `LocalShared`, `LocalWeak`, `Cell`, наблюдатель |
| 09-06 | [09-memory/06_arena.kenox](09-memory/06_arena.kenox) | Арена и дескрипторы `Id<T>` |
| 09-07 | [09-memory/07_unsafe_buffer.kenox](09-memory/07_unsafe_buffer.kenox) | `unsafe` под безопасным интерфейсом |
| 09-08 | [09-memory/08_threads.kenox](09-memory/08_threads.kenox) | Потоки, `Shared<Mutex<T>>`, `Send`/`Sync`, каналы |
| 10-01 | [10-files/01_read_write.kenox](10-files/01_read_write.kenox) | Чтение и запись файлов, пути |
| 10-02 | [10-files/02_lines_buffered.kenox](10-files/02_lines_buffered.kenox) | Построчная обработка, буферизация, явное закрытие |
| 10-03 | [10-files/03_directories.kenox](10-files/03_directories.kenox) | Обход каталогов, метаданные |
| 11-01 | [11-modules/inventory/](11-modules/inventory/src/main.kenox) | Приложение из модулей, видимость, зависимость |
| 11-02 | [11-modules/textstats/](11-modules/textstats/src/lib.kenox) | Библиотека, публичный API, реэкспорт, тесты |
| 12-01 | [12-ffi/01_call_libc.kenox](12-ffi/01_call_libc.kenox) | Вызов функций C, строки C |
| 12-02 | [12-ffi/02_repr_c/](12-ffi/02_repr_c/src/main.kenox) | `@repr(C)`, C-перечисления, структура по значению и указателю |
| 12-03 | [12-ffi/03_callbacks.kenox](12-ffi/03_callbacks.kenox) | Обратные вызовы из C (`qsort`) |
| 12-04 | [12-ffi/04_export_library/](12-ffi/04_export_library/src/lib.kenox) | Библиотека для C: экспорт, владение через границу, заголовок |

## Как проверялся корпус

Компилятора ещё нет, поэтому корпус проверен двумя способами:
1. Инструмент `tools/repo-check` (запускается командой `cargo test`) ищет конструкции других языков (`fn`, `=>`, `when`, `$` в строках, `..` без `<` или `=`, `?` для передачи ошибки и другие), проверяет заголовки примеров и считает пометки. Отчёт — в CORPUS_REVIEW.md.
2. Каждый пример прочитан на соответствие эскизу и модели памяти. Найденные расхождения исправлены в примерах или вынесены в Р-N.

C-файлы из раздела 12 компилируются clang без предупреждений — это тоже часть `cargo test`.
