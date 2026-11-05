/* geom.h — небольшая C-библиотека для примера 12-02 */
#ifndef GEOM_H
#define GEOM_H

typedef struct {
    double width;
    double height;
} geom_rect;

typedef enum { GEOM_OK = 0, GEOM_NEGATIVE = 1 } geom_status;

double geom_area(geom_rect r);                       /* структура по значению */
int geom_scale(geom_rect *r, double factor);         /* по указателю; возвращает geom_status */

#endif
