# 2. Синтаксис

> Глава спецификации ядра · Версия 0.1 · 5 ноября 2026 · Утверждена 05.11.2026; правило [syn.generic.expr] — предварительно · Общие соглашения — [README](README.md)
>
> Синтаксис определяет, как из лексем главы 1 строится программа: объявления, типы, инструкции, выражения и образцы. Глава описывает весь язык эскиза — каждый пример корпуса должен разбираться по этой грамматике. Смысл конструкций — в главах 3–10. Решения, которых не было в эскизе, — в разделе 2.9; контрольные примеры для них — в разделе 2.10.

## 2.1 Общие правила

**[syn.sep]** Разделитель элементов последовательности — `NL` или `;` (правило `Sep`). Лишние разделители в начале, в конце и между элементами допускаются. Когда перевод строки разделяет элементы, а когда пропускается, определяет раздел 1.8.

**[syn.list.trailing-comma]** В списках через запятую — аргументах, параметрах, аргументах типа, литералах коллекций, полях и вариантах — после последнего элемента может стоять запятая. Так удобнее писать список по одному элементу в строке.

**[syn.list.sep]** Поля структуры, варианты перечисления и ветки `match` разделяются переводом строки или запятой (правило `ListSep`).

**[syn.op.compose]** Операции `>=`, `>>` и `>>=` состоят из соседних лексем `>` и `=` без пробела между ними ([lex.op.gt]). `a > = b` и `a > > b` — ошибка.

```ebnf
Sep = NL | ";" ;
ListSep = Sep | "," ;
GtEq = ">" "=" ;
GtGt = ">" ">" ;
GtGtEq = ">" ">" "=" ;
```

## 2.2 Файл и импорт

**[syn.file]** Файл состоит из импортов, за которыми следуют объявления. Импорт после первого объявления — ошибка. Корень грамматики — `File`.

**[syn.import.form]** Импорт называет модуль или имя внутри модуля через точку: `import std.fs`, `import model.item.Item`. Несколько имён одного модуля перечисляются в фигурных скобках: `import std.collections.{Map, Set}`.

**[syn.import.pub]** `pub import` делает импортированные имена частью публичного API пакета (реэкспорт).

**[syn.import.limits]** В версии 0.1 нет импорта «всех имён модуля» и переименования при импорте. Слово `as` зарезервировано для переименования ([lex.keyword.reserved]).

```ebnf
File = { Import Sep } { Declaration Sep } ;
Import = [ "pub" ] "import" ModulePath [ "." ImportGroup ] ;
ModulePath = IDENT { "." IDENT } ;
ImportGroup = "{" IDENT { "," IDENT } [ "," ] "}" ;
```

```kenox
import std.fs
import std.rc.{LocalShared, LocalWeak}
pub import counting.{wordCount, stats, Stats}
```

## 2.3 Объявления

**[syn.decl.prefix]** Перед объявлением могут стоять документирующие комментарии и атрибуты, затем — видимость `pub` или `private`. Смысл видимости — глава 3.

**[syn.attr.syntax]** Атрибут — `@`, имя и, по желанию, аргументы в скобках: `@test`, `@repr(C)`, `@export("counter_new")`, `@derive(Copy, Show)`. Между `@` и именем нет пробела. Аргумент атрибута — имя или литерал. Каждый атрибут обычно пишут на отдельной строке; перевод строки после атрибута пропускается, потому что объявление ещё не закончено ([lex.nl.end]).

**[syn.doc.place]** Документирующий комментарий допустим только перед объявлением, членом типа, вариантом перечисления и объявлением в блоке `extern`. В другом месте — ошибка.

**[syn.decl.globals]** На верхнем уровне файла объявляются функции, структуры, перечисления, трейты, расширения, блоки `extern`, константы `const` и псевдонимы типов. Глобальных переменных `val` и `var` в версии 0.1 нет.

**[syn.decl.local]** Внутри функций объявляются только локальные привязки `val`, `var` и `inout` (раздел 2.5). Вложенных функций и типов в версии 0.1 нет: вместо вложенной функции — замыкание.

```ebnf
Declaration = DeclPrefix DeclBody ;
DeclPrefix = { DOC_COMMENT | Attribute } [ Visibility ] ;
Visibility = "pub" | "private" ;
DeclBody = FunDecl | StructDecl | EnumDecl | TraitDecl | ExtendDecl
         | ExternBlock | ConstDecl | TypeAlias ;
Attribute = "@" IDENT [ "(" [ AttrArg { "," AttrArg } [ "," ] ] ")" ] ;
AttrArg = IDENT | Literal ;
```

### 2.3.1 Функции

**[syn.fun.form]** Объявление функции: `fun`, имя, обобщённые параметры, параметры в круглых скобках, тип результата после `:`, условия `where` и тело.

**[syn.fun.self]** Параметр `self` (с режимом или без) может стоять только первым. Функция в теле типа без `self` — статическая.

**[syn.fun.mode]** Перед параметром может стоять режим `inout` или `consume`; без режима параметр заимствуется (ADR-0003, глава 7).

**[syn.fun.default]** Значение по умолчанию пишется после типа: `max: Int = 100`. Параметры со значениями по умолчанию стоят после параметров без них.

**[syn.fun.body]** Тело — блок или `=` и выражение. Тела нет только у функций в трейтах (тогда реализация обязательна в расширении) и в блоках `extern`. Выражение после `=` может начинаться на следующей строке: строка, оборвавшаяся на `=`, не закончена.

**[syn.fun.result]** Тип результата пишется после `:`; без него результат — `Unit`. Запись `->` в объявлении функции — ошибка с подсказкой; `->` используется только в функциональных типах (раздел 2.4).

**[syn.fun.extern]** `extern "C"` перед `fun` объявляет функцию с соглашением о вызове C (ADR-0013). Строка после `extern` — название соглашения; в версии 0.1 допустимо только `"C"`.

**[syn.fun.variadic]** Параметр `...` (переменное число аргументов) допустим только последним и только у функции в блоке `extern` (глава 9).

```ebnf
FunDecl = [ ExternAbi ] "fun" IDENT [ GenericParams ] "(" [ ParamList ] ")"
          [ ":" Type ] [ WhereClause ] [ FunBody ] ;
ExternAbi = "extern" STRING_LITERAL ;
ParamList = Param { "," Param } [ "," ] ;
Param = SelfParam | [ ParamMode ] IDENT ":" Type [ "=" Expr ] | "..." ;
SelfParam = [ ParamMode ] "self" ;
ParamMode = "inout" | "consume" ;
FunBody = "=" Expr | Block ;
```

```kenox
fun clamp(value: Int, min: Int = 0, max: Int = 100): Int =
    if value < min { min } else if value > max { max } else { value }

fun deposit(inout account: Account, amount: Int) {
    account.balance += amount
}
// ✗ [syn.fun.result] fun square(x: Int) -> Int = x * x — тип результата пишется после двоеточия
```

### 2.3.2 Обобщённые параметры и `where`

**[syn.generic.params]** Обобщённые параметры — в угловых скобках после имени функции или типа и после `extend`. Параметр-тип может иметь ограничения через `:`, несколько трейтов соединяются `+`: `<T: Zero + Add + Copy>`. Константный параметр — `const N: Int`.

**[syn.generic.where]** Условия `where` перечисляют ограничения через запятую: `where T: Show, U: Eq`. Они могут начинаться на следующей строке: `where` — лексема продолжения ([lex.nl.continue]).

```ebnf
GenericParams = "<" GenericParam { "," GenericParam } [ "," ] ">" ;
GenericParam = IDENT [ ":" Bounds ] | "const" IDENT ":" Type ;
Bounds = TypePath { "+" TypePath } ;
WhereClause = "where" WherePredicate { "," WherePredicate } ;
WherePredicate = Type ":" Bounds ;
```

### 2.3.3 Структуры

**[syn.struct.form]** У структуры либо именованные поля и методы в фигурных скобках, либо безымянные поля в круглых скобках — структура-обёртка: `struct UserId(Int)`. Пустая структура — `struct ConsoleLog {}`.

**[syn.struct.members]** В теле структуры — поля, методы и константы в любом порядке, разделённые переводом строки или запятой. Поле — `имя: Тип`, перед ним может стоять видимость.

```ebnf
StructDecl = "struct" IDENT [ GenericParams ] ( TupleFields | [ WhereClause ] MemberBlock ) ;
TupleFields = "(" [ Type { "," Type } [ "," ] ] ")" ;
MemberBlock = "{" [ Member { ListSep Member } [ ListSep ] ] "}" ;
Member = DeclPrefix ( Field | FunDecl | ConstDecl ) ;
Field = IDENT ":" Type ;
```

```kenox
struct Size { w: Int, h: Int }

pub struct Email {
    private value: String

    pub fun domain(self): str = self.value.substringAfter("@")
}
```

### 2.3.4 Перечисления

**[syn.enum.variant]** Вариант перечисления — имя и, по желанию, именованные поля в скобках: `Circle(radius: Float64)`. Варианты разделяются переводом строки или запятой; в теле перечисления могут быть и методы.

**[syn.enum.value]** Варианту без полей можно задать целое значение: `Low = 1`. Тогда значения задаются всем вариантам, а у перечисления должен быть атрибут `@repr` с целым типом (глава 9).

**[syn.enum.no-visibility]** Перед вариантом не ставится видимость: варианты видны везде, где видно перечисление.

```ebnf
EnumDecl = "enum" IDENT [ GenericParams ] [ WhereClause ]
           "{" [ EnumItem { ListSep EnumItem } [ ListSep ] ] "}" ;
EnumItem = DeclPrefix ( Variant | FunDecl ) ;
Variant = IDENT [ "(" VariantField { "," VariantField } [ "," ] ")" ] [ "=" Expr ] ;
VariantField = IDENT ":" Type ;
```

```kenox
enum Shape {
    Circle(radius: Float64)
    Rect(width: Float64, height: Float64)
    Empty
}

@repr(Int32)
enum Level { Low = 1, High = 2 }
```

### 2.3.5 Трейты и расширения

**[syn.trait.form]** Трейт содержит функции (с телом — реализация по умолчанию, без тела — обязательные), ассоциированные типы `type Item` и константы. После имени трейта через `:` могут быть указаны трейты, которые он расширяет.

**[syn.extend.form]** `extend Тип { … }` добавляет методы к типу; `extend Тип: Трейт { … }` реализует трейт для типа. Обобщённые параметры пишутся сразу после `extend`: `extend<T: Show> Stack<T> { … }`.

**[syn.extend.by]** Делегирование `by self.поле` записывается после трейта; цель — только путь от `self` через поля: `by self.inner`, `by self.parts.log` (ADR-0004, Р-8).

**[syn.assoc.type]** В трейте ассоциированный тип объявляется как `type Имя` с необязательными ограничениями; в расширении — задаётся: `type Item = Int`.

```ebnf
TraitDecl = "trait" IDENT [ GenericParams ] [ ":" Bounds ] [ WhereClause ]
            "{" [ AssocItem { Sep AssocItem } [ Sep ] ] "}" ;
ExtendDecl = "extend" [ GenericParams ] Type [ ":" TypePath [ Delegation ] ] [ WhereClause ]
             "{" [ AssocItem { Sep AssocItem } [ Sep ] ] "}" ;
Delegation = "by" "self" "." IDENT { "." IDENT } ;
AssocItem = DeclPrefix ( FunDecl | AssocType | ConstDecl ) ;
AssocType = "type" IDENT [ ":" Bounds ] [ "=" Type ] ;
```

```kenox
extend Fibonacci: Iterator {
    type Item = Int
    fun next(inout self): Int? { … }
}

extend<L: Log> Numbered<L>: Log by self.inner {
    fun write(inout self, line: str) { … }
}
```

### 2.3.6 Константы, псевдонимы типов, блоки `extern`

**[syn.const.form]** Константа — `const ИМЯ: Тип = выражение`; тип можно опустить, если он выводится из выражения.

**[syn.type-alias.form]** Псевдоним типа — `type Имя = Тип`, может иметь обобщённые параметры.

**[syn.extern.block]** Блок `extern "C" { … }` содержит объявления C-функций без тела. Объявление может занимать несколько строк: внутри круглых скобок переводы строк не значат ничего ([lex.nl.brackets]).

```ebnf
ConstDecl = "const" IDENT [ ":" Type ] "=" Expr ;
TypeAlias = "type" IDENT [ GenericParams ] "=" Type ;
ExternBlock = ExternAbi "{" [ ExternItem { Sep ExternItem } [ Sep ] ] "}" ;
ExternItem = { DOC_COMMENT | Attribute } FunDecl ;
```

```kenox
const MAX_RETRIES: Int = 3
type UserMap = Map<UserId, User>

extern "C" {
    fun strlen(s: RawPtr<CChar>): CSize
    fun qsort(
        base: RawMutPtr<CVoid>,
        count: CSize,
        size: CSize,
        compare: extern "C" fun(RawPtr<CVoid>, RawPtr<CVoid>) -> CInt,
    )
}
```

## 2.4 Типы

**[syn.type.path]** Имя типа — путь через точку с аргументами типа в угловых скобках: `Int`, `fs.IoError`, `Map<String, List<Int>>`, `Self`, `Self.Item`. Аргумент типа — тип или целый литерал: `Array<Float32, 4>`.

**[syn.type.optional]** `?` после типа делает его необязательным: `String?`, `List<String?>`, `(Int, Int)?`. Знаки можно повторять: `Int??` (ADR-0009).

**[syn.type.ref]** `ref T` и `inout T` — результат-заимствование (Р-12, глава 7). `?` относится ко всей записи: `ref T?` — необязательное заимствование, то есть `(ref T)?`.

**[syn.type.paren]** В круглых скобках: `()` — тип `Unit`; `(T)` — то же, что `T`; `(A, B)` и `(T,)` — кортежи.

**[syn.type.fn]** Функциональный тип — список параметров в круглых скобках, `->` и тип результата: `(Int) -> Int`, `() -> Unit`. Перед параметром может стоять режим: `(inout T) -> Unit`. Тип результата — всё, что справа от `->`: `() -> Int?` возвращает `Int?`, а необязательная функция пишется как `(() -> Int)?`.

**[syn.type.escaping]** `escaping` ставится только перед функциональным типом: `escaping (str) -> String` (Р-7).

**[syn.type.extern-fn]** Указатель на C-функцию — `extern "C" fun(…) -> Тип`.

**[syn.type.some-dyn]** `some Трейт` — непрозрачный тип (Р-11); `dyn Трейт` — тип для динамической диспетчеризации, например `Box<dyn Command>`. После них можно перечислить несколько трейтов через `+`.

```ebnf
Type = PrefixedType { "?" } ;
PrefixedType = [ "ref" | "inout" ] BaseType
             | "escaping" ParenType
             | ( "some" | "dyn" ) Bounds ;
BaseType = TypePath | ParenType | ExternFnType ;
TypePath = ( IDENT | "Self" ) [ GenericArgs ] { "." IDENT [ GenericArgs ] } ;
GenericArgs = "<" GenericArg { "," GenericArg } [ "," ] ">" ;
GenericArg = Type | [ "-" ] INT_LITERAL ;
ParenType = "(" [ FnParam { "," FnParam } [ "," ] ] ")" [ "->" Type ] ;
FnParam = [ ParamMode ] Type ;
ExternFnType = ExternAbi "fun" "(" [ FnParam { "," FnParam } [ "," ] ] ")" [ "->" Type ] ;
```

Режим перед элементом допустим только в функциональном типе — в кортеже `(inout Int, Int)` это ошибка. `escaping` без `->` — тоже ошибка.

```kenox
fun largest<T: Order>(items: Slice<T>): ref T? { … }
fun makeCounter(start: Int): escaping () -> Int { … }
var handlers: List<escaping () -> Unit> = []
fun all(self): some Iterator<ref Item> = self.items.values()
```

## 2.5 Инструкции и блоки

**[syn.block]** Блок — инструкции в фигурных скобках, разделённые `Sep`. Значение блока-выражения — значение последней инструкции, если это выражение (глава 5).

**[syn.stmt.local]** `val` объявляет неизменяемую привязку, `var` — изменяемую. После слова — образец (обычно имя), затем необязательные тип и начальное значение: `val (lo, hi) = bounds`, `var best: ref T? = null`.

**[syn.stmt.inout]** `inout имя = выражение` привязывает изменяемую проекцию места (раздел 10 эскиза, глава 7): `inout counter = unsafe { c.asMut() } ?: return -1`.

**[syn.stmt.assign]** Присваивание — инструкция, а не выражение: `x = 1`, `self.count += 1`. Оно не возвращает значения, его нельзя использовать внутри выражения и нельзя писать цепочкой `a = b = c`. Слева — место (глава 5). Утверждено 05.11.2026.

**[syn.stmt.expr]** Выражение может стоять как инструкция. Компилятор предупреждает, если значение такой инструкции не используется и не имеет тип `Unit` или `Never`.

```ebnf
Block = "{" [ Statement { Sep Statement } [ Sep ] ] "}" ;
Statement = LocalDecl | InoutBinding | Assignment | Expr ;
LocalDecl = ( "val" | "var" ) Pattern [ ":" Type ] [ "=" Expr ] ;
InoutBinding = "inout" IDENT "=" Expr ;
Assignment = Expr AssignOp Expr ;
AssignOp = "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | GtGtEq ;
```

```kenox
fun sample() {
    var attempts = 0
    attempts += 1
    val (lo, hi) = bounds(attempts)
    val Point(x, y) = Point(x = lo, y = hi)
}
// ✗ [syn.stmt.assign] if (count = 0) { … } — присваивание не является выражением; сравнение пишется как ==
```

## 2.6 Выражения

### 2.6.1 Приоритет операций

**[syn.expr.prec]** Приоритет операций — от самого высокого к самому низкому:

| Уровень | Операции | Порядок |
|---|---|---|
| 1 | Постфиксные: вызов `f(…)` и `f { … }`, индекс `a[i]`, член `.x`, безопасный вызов `?.x`, поле кортежа `.0` | слева направо |
| 2 | Префиксные: `-` `!` `~` `try` `ref` `consume` | справа налево |
| 3 | `*` `/` `%` | слева направо |
| 4 | `+` `-` | слева направо |
| 5 | `<<` `>>` | слева направо |
| 6 | `&` | слева направо |
| 7 | `^` | слева направо |
| 8 | `\|` | слева направо |
| 9 | `?:` | справа налево |
| 10 | `..<` `..=` | не цепляются |
| 11 | `==` `!=` `<` `<=` `>` `>=` `in` `is` | не цепляются |
| 12 | `&&` | слева направо |
| 13 | `\|\|` | слева направо |

`return`, `break` и `continue` — ниже всех: `return` забирает всё выражение справа.

**[syn.expr.bitwise]** Битовые операции связывают сильнее сравнений, как в Rust и Swift (Q-43): `x & MASK == 0` означает `(x & MASK) == 0`, а не ловушку C `x & (MASK == 0)`.

**[syn.expr.nonassoc]** Сравнения и диапазоны не цепляются: `a < b < c` и `a == b == c` — ошибка с подсказкой. Диапазон проверяется через `in`: `x in 0..<10`.

**[syn.expr.prefix]** Префиксные операции применяются ко всей постфиксной цепочке справа: `!c.isNull()` — это `!(c.isNull())`, `try fs.readText(path).context("…")` — `try` от результата `context`, `ref tasks[0]` — заимствование элемента.

**[syn.expr.elvis]** `?:` правоассоциативна, справа от неё может стоять `return`, `break` или `continue`: `val user = find(id) ?: return Err(.NotFound(id))`. `?:` связывает сильнее диапазонов и сравнений: `start ?: 0..<end` — это `(start ?: 0)..<end`, а `a ?: 0 > 5` — `(a ?: 0) > 5`. Утверждено 05.11.2026.

**[syn.expr.unary-plus]** Унарного `+` нет: строка, которая начинается с `+`, продолжает предыдущую ([lex.nl.continue]).

```ebnf
Expr = JumpExpr | OrExpr ;
JumpExpr = "return" [ Expr ] | "break" | "continue" ;
OrExpr = AndExpr { "||" AndExpr } ;
AndExpr = CmpExpr { "&&" CmpExpr } ;
CmpExpr = RangeExpr [ CmpOp RangeExpr | "in" RangeExpr | "is" IsPattern ] ;
CmpOp = "==" | "!=" | "<" | "<=" | ">" | GtEq ;
RangeExpr = ElvisExpr [ ( "..<" | "..=" ) ElvisExpr ] ;
ElvisExpr = BitOrExpr [ "?:" ( ElvisExpr | JumpExpr ) ] ;
BitOrExpr = BitXorExpr { "|" BitXorExpr } ;
BitXorExpr = BitAndExpr { "^" BitAndExpr } ;
BitAndExpr = ShiftExpr { "&" ShiftExpr } ;
ShiftExpr = AddExpr { ( "<<" | GtGt ) AddExpr } ;
AddExpr = MulExpr { ( "+" | "-" ) MulExpr } ;
MulExpr = PrefixExpr { ( "*" | "/" | "%" ) PrefixExpr } ;
PrefixExpr = { "-" | "!" | "~" | "try" | "ref" | "consume" } PostfixExpr ;
```

```kenox
fun examples(flags: Int, n: Int, port: Int?): Bool {
    val isClear = flags & 0x4 == 0          // (flags & 0x4) == 0
    val last = 0..<n - 1                    // 0..<(n - 1)
    val p = port ?: 8080
    return isClear && p in 1..=65535
}
// ✗ [syn.expr.nonassoc] if 0 < x < 10 { … } — пишется как x in 1..<10 или 0 < x && x < 10
```

### 2.6.2 Постфиксные операции и вызовы

**[syn.call.args]** Аргументы вызова — позиционные, затем именованные: `connect("db.local", port = 5432)`. Позиционный аргумент после именованного — ошибка.

**[syn.call.inout]** `&` в начале аргумента отмечает передачу в параметр `inout`: `deposit(&acc, 50)`. Кроме аргументов вызова, `&` в этом смысле стоит только после `in` в цикле `for inout` (раздел 2.6.4). В остальных местах `&` — двоичная операция «побитовое и».

**[syn.call.trailing]** Последний аргумент-замыкание можно вынести за скобки: `list.sortBy { it.priority }`, `List.generate(1000) { it }`. Если других аргументов нет, скобки не пишутся. Открывающая `{` должна стоять на той же строке: перевод строки перед ней заканчивает вызов.

**[syn.generic.expr]** В выражении `<` после имени начинает аргументы типа, если дальше разбирается список типов, закрытый `>`, и сразу за ним идёт `(` или `.`: `Arena<Expr>()`, `channel<String>()`, `Simd<Float32, 8>.splat(0.0)`, `a.cast<Int32>()`. Иначе `<` — сравнение. Поэтому `f(a < b, c > (d))` разбирается как вызов обобщённой функции `a<b, c>(d)`; если нужны два сравнения, их берут в скобки: `f((a < b), (c > d))`. Утверждено предварительно 05.11.2026: разборщик обязательно покрывается тестами неоднозначностей из раздела 2.10; если на практике пробный разбор окажется слишком сложным или неудобным, решение пересматривается (эксперимент E-9).

**[syn.expr.member-number]** После `.` целое число — номер поля кортежа или структуры-обёртки: `pair.0`, `id.0` ([lex.number.after-dot]).

**[syn.expr.postfix-any]** Постфиксные операции применимы к любому первичному выражению, включая `unsafe { … }` и `do { … }`: `unsafe { CStr.fromPtr(raw) }.toStringLossy()`.

```ebnf
PostfixExpr = Primary { Postfix } ;
Postfix = CallSuffix | "[" Expr "]" | ( "." | "?." ) IDENT [ GenericArgs ] | "." INT_LITERAL ;
CallSuffix = "(" [ ArgList ] ")" [ Closure ] | Closure ;
ArgList = Arg { "," Arg } [ "," ] ;
Arg = [ IDENT "=" ] [ "&" ] Expr ;
```

```kenox
fun calls(counts: Map<String, Int>, w: String) {
    counts.update(w, default = 0) { it += 1 }
    val va = Simd<Float32, 8>.splat(0.0)
    val size = sizeOf<Int32>()
}
// ✗ [syn.call.args] connect(port = 5432, "db.local") — позиционный аргумент после именованного
```

### 2.6.3 Первичные выражения, коллекции и замыкания

**[syn.implicit-member]** `.Имя` — член типа, известного из контекста: `.Circle(radius = 1.0)`, `.Pending`, `e.kind() == .NotFound`. Какой тип ожидается, определяет глава 5.

**[syn.paren]** В круглых скобках: `()` — значение `Unit`, `(a)` — то же, что `a`, `(a, b)` и `(a,)` — кортежи.

**[syn.collection]** `[a, b, c]` — литерал последовательности, `[k: v, …]` — литерал словаря, `[:]` — пустой словарь. Какая коллекция получится (`List`, `Set`, `Array`), определяет ожидаемый тип (глава 5).

**[syn.closure.brace]** Фигурная скобка в позиции выражения начинает замыкание (Р-2). Блоком она будет после `if`, `else`, `while`, `for`, `loop`, `do`, `unsafe`, в теле функции и в теле ветки `match` после `->`.

**[syn.closure.params]** Параметры замыкания перечисляются в начале через запятую и отделяются `->`: `{ acc, item -> acc + item.price }`. Типы параметров можно указать: `{ x: Int -> x * 2 }`. Без списка параметров единственный параметр называется `it`.

**[syn.closure.condition]** В условии `if` и `while`, в выражении после `match` и после `in` в `for` замыкание не выносится за скобки на верхнем уровне: `{` там начинает тело конструкции. Замыкание в таком месте пишут в скобках: `if xs.any({ it > 0 }) { … }`. Утверждено 05.11.2026.

**[syn.block.expr]** Отдельный блок-выражение пишется как `do { … }` (Р-2).

```ebnf
Primary = Literal | IDENT [ GenericArgs ] | "self" | "Self" | ImplicitMember
        | ParenExpr | CollectionLiteral | Closure | IfExpr | MatchExpr
        | WhileExpr | ForExpr | LoopExpr | DoExpr | UnsafeExpr ;
Literal = INT_LITERAL | FLOAT_LITERAL | CHAR_LITERAL | STRING_LITERAL
        | "true" | "false" | "null" ;
ImplicitMember = "." IDENT ;
ParenExpr = "(" [ Expr { "," Expr } [ "," ] ] ")" ;
CollectionLiteral = "[" [ Expr { "," Expr } [ "," ] ] "]"
                  | "[" MapEntry { "," MapEntry } [ "," ] "]"
                  | "[" ":" "]" ;
MapEntry = Expr ":" Expr ;
Closure = "{" [ ClosureParams "->" ] [ Statement { Sep Statement } [ Sep ] ] "}" ;
ClosureParams = ClosureParam { "," ClosureParam } ;
ClosureParam = ( IDENT | "_" ) [ ":" Type ] ;
DoExpr = "do" Block ;
UnsafeExpr = "unsafe" Block ;
```

```kenox
fun collections() {
    val ages = ["Анна": 31, "Борис": 27]
    val empty: Map<String, Int> = [:]
    val pair = ("a", 1)
    val total = [120, 45, 300].fold(0) { acc, price -> acc + price }
    val area = do {
        val w = 3
        w * 4
    }
}
// ✗ [syn.closure.condition] if xs.any { it > 0 } { … } — замыкание в условии берётся в скобки
```

### 2.6.4 Управляющие конструкции

**[syn.if.form]** `if` пишется без скобок вокруг условия и всегда с фигурными скобками у тела: `if n > 0 { … } else if n == 0 { … } else { … }`. `else` может начинаться на новой строке ([lex.nl.continue]).

**[syn.cond.val]** Условие `val образец = выражение` выполняет тело, если значение выражения — не `null`, и связывает образец с содержимым: `if val name = n { … }`, `if val (lo, hi) = minMax(xs) { … }`. `inout` вместо `val` даёт изменяемую проекцию. В версии 0.1 такое условие не соединяется с другими через `&&`; для проверки образца с условием — `is`: `if s is .Circle(r) && r > 1.0 { … }`.

**[syn.match.form]** `match выражение { ветки }`. Ветка — образец, необязательное условие `if`, `->` и тело. Ветки разделяются переводом строки или запятой.

**[syn.match.body]** Тело ветки — блок, присваивание или выражение, в том числе цикл и `return`: `.Leaf -> self = .Node(…)`, `.Ok(entries) -> for e in entries { … }`, `.Err(e) -> return Err(e)`. Фигурная скобка после `->` начинает блок, а не замыкание: `_ -> {}` — пустой блок.

**[syn.for.form]** `for образец in выражение { … }`. `for inout x in &xs` изменяет элементы на месте; `for x in consume xs` забирает элементы.

**[syn.loop.form]** `loop { … }` — бесконечный цикл. `break` и `continue` в версии 0.1 — без меток и без значения.

```ebnf
IfExpr = "if" Condition Block [ "else" ( IfExpr | Block ) ] ;
Condition = ( "val" | "inout" ) Pattern "=" Expr | Expr ;
MatchExpr = "match" Expr "{" [ MatchArm { ListSep MatchArm } [ ListSep ] ] "}" ;
MatchArm = Pattern [ "if" Expr ] "->" ArmBody ;
ArmBody = Block | Assignment | Expr ;
WhileExpr = "while" Condition Block ;
ForExpr = "for" [ "inout" ] Pattern "in" [ "&" ] Expr Block ;
LoopExpr = "loop" Block ;
```

```kenox
fun control(tasks: List<Task>, queue: List<Int>) {
    var pending = tasks.clone()
    for inout t in &pending {
        if t.priority >= 3 { t.done = true }
    }
    var jobs = queue.clone()
    while val job = jobs.popFirst() {
        if job < 0 { continue }
        println("задача {job}")
    }
    val label = match pending.len() { 0 -> "пусто", 1 -> "одна", _ -> "много" }
}
```

## 2.7 Образцы

**[syn.pat.binding]** Одиночное имя в образце — всегда новая привязка. Константы и варианты сравниваются по полному имени через точку (`Int.MIN`, `GeomStatus.Ok`) или по имени варианта с точкой (`.Quit`); константа верхнего уровня — через условие ветки: `n if n == LIMIT -> …`. Утверждено 05.11.2026.

**[syn.pat.binding-warning]** Если имя привязки в образце совпадает с видимой константой, компилятор обязан выдать предупреждение и подсказать запись сравнения: скорее всего, имелось в виду сравнение, а не новая переменная. Утверждено 05.11.2026.

**[syn.pat.constructor]** Имя со скобками — образец структуры или структуры-обёртки: `Point(x, 0)`, `UserId(n)`. Поля сопоставляются по порядку или по имени: `Point(x = px, y = 0)`.

**[syn.pat.variant]** `.Вариант(…)` — образец варианта перечисления, тип которого известен из контекста; `Тип.Вариант(…)` — с явным типом.

**[syn.pat.range]** Диапазон в образце — `..<` или `..=` между литералами, константами или символами: `1..=9`, `'a'..='z'`, `Int.MIN..=-1`.

**[syn.pat.values]** Литералы в образцах — целые (с необязательным `-`), символы, строки, `true`, `false`, `null`. Вещественных литералов в образцах нет: точное сравнение вещественных чисел обычно ошибочно.

**[syn.pat.alt]** Альтернативы через `|` допустимы в ветках `match`, в `val` и в `for`. После `is` стоит один образец; альтернативы там берутся в скобки: `x is (.A | .B)`.

**[syn.pat.group]** `(образец)` — тот же образец; `(a, b)` и `(a,)` — образцы кортежей.

```ebnf
Pattern = PrimaryPattern { "|" PrimaryPattern } ;
IsPattern = PrimaryPattern ;
PrimaryPattern = "_" | TuplePattern | VariantPattern
               | ValuePattern [ RangeTail ]
               | QualifiedName [ PatternArgs | RangeTail ]
               | IDENT [ PatternArgs | RangeTail ] ;
ValuePattern = [ "-" ] INT_LITERAL | CHAR_LITERAL | STRING_LITERAL
             | "true" | "false" | "null" ;
RangeTail = ( "..<" | "..=" ) PatternBound ;
PatternBound = ValuePattern | QualifiedName | IDENT ;
QualifiedName = IDENT "." IDENT { "." IDENT } ;
VariantPattern = "." IDENT [ PatternArgs ] ;
PatternArgs = "(" [ PatternArg { "," PatternArg } [ "," ] ] ")" ;
PatternArg = [ IDENT "=" ] Pattern ;
TuplePattern = "(" [ Pattern { "," Pattern } [ "," ] ] ")" ;
```

```kenox
fun handle(e: Event): String = match e {
    .Key(27, _) -> "Escape"
    .Key(code, true) if code in 65..=90 -> "заглавная буква"
    .Resize(0, _) | .Resize(_, 0) -> "окно свёрнуто"
    .Click(x, y) -> "щелчок в ({x}, {y})"
    _ -> "прочее"
}

fun classify(n: Int): String = match n {
    Int.MIN..=-1 -> "отрицательное"
    0 -> "ноль"
    _ -> "положительное"
}
```

## 2.8 Покрытие корпуса

Грамматика сверена с корпусом чтением: ниже — конструкции корпуса и правила, которые их описывают. Машинной проверки разбора пока нет — для неё нужен разборщик, а он появится на этапе 1.

| Конструкция | Примеры | Правила |
|---|---|---|
| Импорт групп, реэкспорт | 09-05, 09-07, 11-02 | [syn.import.form], [syn.import.pub] |
| Несколько атрибутов подряд, `@borrows` | 12-02, 12-04, 09-03 | [syn.attr.syntax] |
| Функции-выражения на нескольких строках, значения по умолчанию | 02-01, 03-01, 09-03 | [syn.fun.body], [syn.fun.default] |
| Режимы параметров и `&` при вызове | 02-02, 04-03, 09-02, 12-02 | [syn.fun.mode], [syn.call.inout] |
| Ограничения через `+`, `where`, константные параметры | 04-01, 05-01, 05-02, 05-04 | [syn.generic.params], [syn.generic.where] |
| Аргументы типа в выражениях | 05-04, 09-06, 09-07, 09-08, 12-03 | [syn.generic.expr] |
| Структура-обёртка, пустая структура, поля через запятую | 03-03, 04-02, 04-01 | [syn.struct.form], [syn.struct.members] |
| Варианты с полями и со значениями | 03-02, 12-02 | [syn.enum.variant], [syn.enum.value] |
| Ассоциированные типы, делегирование | 05-03, 08-04, 04-02 | [syn.assoc.type], [syn.extend.by] |
| Блок `extern "C"`, указатель на C-функцию | 12-01, 12-03 | [syn.extern.block], [syn.type.extern-fn] |
| `ref T?`, `some`, `dyn`, `escaping` | 05-01, 09-03, 04-03, 04-04, 02-03 | [syn.type.ref], [syn.type.some-dyn], [syn.type.escaping] |
| Разбор в `val` и `for`, `inout`-привязка | 07-03, 08-02, 12-04 | [syn.stmt.local], [syn.stmt.inout] |
| Присваивание, цикл и `return` в ветке `match` | 05-02, 06-01, 10-01, 10-03 | [syn.match.body] |
| Условия в ветках, альтернативы, диапазоны в образцах | 08-01, 08-04 | [syn.pat.alt], [syn.pat.range] |
| `?:` с `return` и `continue` | 03-01, 06-01, 06-02, 12-04 | [syn.expr.elvis] |
| Замыкания за скобками и вложенные замыкания | 05-01, 07-02, 09-05, 09-08 | [syn.call.trailing], [syn.closure.params] |
| Цепочки с переносом строк, многострочные аргументы | 07-03, 06-01, 11-02 | [lex.nl.continue], [syn.list.trailing-comma] |
| Вызов метода у `unsafe { … }` | 12-01, 12-04 | [syn.expr.postfix-any] |

Две правки корпуса, которых требуют утверждённые правила главы 1:

- **08-04:** значение блока в начале строки — `.Number(value)` — продолжило бы предыдущую строку ([lex.nl.match-arm], Q-45). Теперь — `Token.Number(value)` и `Token.Ident(name)`.
- **12-04:** `RawMutPtr.null()` — `null` теперь жёсткое ключевое слово ([lex.keyword.hard], Q-42). Метод переименован в `nullPtr()`.

## 2.9 Решения главы

Решения, которых не было в эскизе. Большинство следует из эскиза и утверждённых правил; пять заметных отмечены — у них есть альтернативы. Все пять утверждены 05.11.2026, пробный разбор `<` — предварительно, с обязательными тестами и экспериментом E-9.

| Решение | Правило | Почему |
|---|---|---|
| Импорты — в начале файла; без импорта всех имён и без переименования | [syn.file], [syn.import.limits] | Видно, откуда пришло каждое имя |
| Поля, варианты и ветки разделяются переводом строки или запятой | [syn.list.sep] | Одна строка — `{ w: Int, h: Int }`, много строк — без запятых |
| Унарного `+` нет | [syn.expr.unary-plus] | Строка с `+` продолжает предыдущую |
| Сравнения и диапазоны не цепляются | [syn.expr.nonassoc] | `0 < x < 10` в C означает не то, что кажется |
| В образцах нет вещественных литералов | [syn.pat.values] | Точное сравнение вещественных чисел обычно ошибочно |
| `break` и `continue` без меток и значений; `if val` не соединяется через `&&` | [syn.loop.form], [syn.cond.val] | Не нужно корпусу; можно добавить позже без поломки кода |
| **Присваивание — инструкция, а не выражение** | [syn.stmt.assign] | Как в Kotlin, Swift и Python: `if x = 0` не скомпилируется |
| **`?:` связывает сильнее диапазонов и сравнений и допускает справа `return`** | [syn.expr.elvis] | Как в Kotlin; `x ?: return …` — частая идиома корпуса |
| **Аргументы типа в выражении — по пробному разбору** | [syn.generic.expr] | Как в Kotlin и C#; корпус пишет `Arena<Expr>()` и `channel<String>()`. Цена — редкий случай `f(a < b, c > (d))` |
| **Замыкание не выносится за скобки в условиях** | [syn.closure.condition] | Как в Swift: иначе `{` после условия неоднозначна |
| **Одиночное имя в образце — всегда привязка; константы — по полному имени** | [syn.pat.binding] | Разбор не зависит от того, какие имена видны; ловушку Rust закрывает предупреждение |
| Глобальных `val`/`var`, вложенных функций и типов в 0.1 нет | [syn.decl.globals], [syn.decl.local] | Корпусу не нужны; глобальное состояние требует правил инициализации (ADR-0013) |

## 2.10 Контрольные примеры

Примеры этого раздела — обязательная часть тестов разборщика: на шаге 0.5 они войдут в тестовый план, на вехе 1a должны проходить ([ROADMAP.md](../docs/ROADMAP.md)). В таблицах скобками показано, как разбирается запись.

### 2.10.1 Аргументы типа и сравнения — [syn.generic.expr]

| № | Запись | Как разбирается |
|---|---|---|
| 1 | `Arena<Expr>()` | вызов обобщённого `Arena` с аргументом типа `Expr` |
| 2 | `channel<String>()` | вызов обобщённой функции |
| 3 | `List<List<Int>>()` | вложенные аргументы типа; `>>` — две лексемы `>` |
| 4 | `Simd<Float32, 8>.splat(0.0)` | обобщённый тип, затем вызов статического метода |
| 5 | `Matrix<2, 2>(rows = r)` | аргументы типа — целые литералы |
| 6 | `a.cast<Int32>()` | вызов обобщённого метода |
| 7 | `Layout.array<Int>(n)` | вызов обобщённого метода с аргументом |
| 8 | `List.generate<Int>(3) { it }` | обобщённый вызов, затем замыкание за скобками |
| 9 | `a < b` | сравнение |
| 10 | `a < b && c > d` | `(a < b) && (c > d)` |
| 11 | `f(a < b, c > d)` | два аргумента, `(a < b)` и `(c > d)`: за `>` идёт имя, а не `(` или `.` |
| 12 | `f(a < b, c > (d))` | один аргумент — вызов `a<b, c>(d)` |
| 13 | `f((a < b), (c > d))` | два аргумента-сравнения: скобки снимают неоднозначность |
| 14 | `a.len() < b.len()` | сравнение: `b.len()` — не тип |
| 15 | `a < b >> c` | `a < (b >> c)`: сдвиг связывает сильнее сравнения |
| 16 | `if count < limit { … }` | сравнение; `{` начинает тело `if` |
| 17 | `x < y > z` | ошибка [syn.expr.nonassoc]: сравнения не цепляются |

### 2.10.2 Присваивание — [syn.stmt.assign]

```kenox
fun reset(inout count: Int, inout total: Int) {
    count = 0                         // ✓ присваивание — инструкция
    total += count
}
// ✗ [syn.stmt.assign] if count = 0 { … } — в условии нужно сравнение: count == 0
// ✗ [syn.stmt.assign] val y = (count = 1) — у присваивания нет значения
// ✗ [syn.stmt.assign] a = b = 0 — присваивания не соединяются в цепочку
```

### 2.10.3 Оператор `?:` — [syn.expr.elvis]

| № | Запись | Как разбирается |
|---|---|---|
| 1 | `a ?: b ?: c` | `a ?: (b ?: c)` |
| 2 | `a ?: b + 1` | `a ?: (b + 1)` |
| 3 | `a ?: 0 > 5` | `(a ?: 0) > 5` |
| 4 | `start ?: 0..<end` | `(start ?: 0)..<end` |
| 5 | `ok && a ?: b` | `ok && (a ?: b)` |
| 6 | `find(id) ?: return Err(e)` | если `find(id)` — `null`, функция возвращает `Err(e)` |

### 2.10.4 Замыкания в условиях — [syn.closure.condition]

```kenox
fun conditions(xs: List<Int>, id: Int) {
    if xs.any({ it > 0 }) { println("есть положительные") }      // ✓
    for x in xs.filter({ it > 0 }) { println(x) }                // ✓
    match xs.find({ it == id }) {                                // ✓
        null -> println("нет")
        _ -> println("есть")
    }
    while xs.all({ it < id }) { break }                          // ✓
}
// ✗ [syn.closure.condition] if xs.any { it > 0 } { … } — `{` после `any` начинает тело if
// ✗ [syn.closure.condition] for x in xs.filter { it > 0 } { … } — то же в цикле
```

### 2.10.5 Одиночное имя в образце — [syn.pat.binding], [syn.pat.binding-warning]

```kenox
const LIMIT: Int = 10

fun check(n: Int): String = match n {
    LIMIT -> "предел"       // ⚠ [syn.pat.binding-warning] новая привязка LIMIT скрывает константу
    _ -> "другое"
}

fun checkFixed(n: Int): String = match n {
    x if x == LIMIT -> "предел"      // ✓ сравнение с константой верхнего уровня
    Int.MAX -> "максимум"            // ✓ константа по полному имени
    _ -> "другое"
}
```

В `check` первая ветка совпадает с любым числом: `LIMIT` здесь — новая переменная. Поэтому компилятор выдаёт предупреждение [syn.pat.binding-warning] и подсказывает запись из `checkFixed`.
