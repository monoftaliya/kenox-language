/* Программа на C, использующая библиотеку counter из примера 12-04 */
#include <stdio.h>
#include "counter.h"

int main(void) {
    Counter *c = counter_new("запросы");
    if (c == NULL) {
        return 1;
    }
    counter_add(c, 5);
    counter_add(c, 7);
    printf("%lld\n", (long long)counter_value(c));   /* 12 */
    counter_free(c);
    return 0;
}
