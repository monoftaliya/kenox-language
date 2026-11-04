/* Пример того, что сгенерирует kenox для пакета counter 0.1.0 (ADR-0013).
   Не редактировать вручную: файл создаётся заново при каждой сборке. */
#ifndef COUNTER_H
#define COUNTER_H

#include <stdint.h>

typedef struct Counter Counter;   /* непрозрачный тип: поля недоступны из C */

/* Создаёт счётчик. Владение результатом переходит к вызывающему; освобождать — counter_free. */
Counter *counter_new(const char *name);

/* 0 — успех; -1 — передан NULL; -2 — переполнение. */
int counter_add(Counter *c, int64_t delta);

/* Текущее значение; для NULL возвращает 0. */
int64_t counter_value(const Counter *c);

/* Освобождает счётчик, созданный counter_new. NULL допустим. */
void counter_free(Counter *c);

#endif
