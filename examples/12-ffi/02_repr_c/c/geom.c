#include "geom.h"

double geom_area(geom_rect r) {
    return r.width * r.height;
}

int geom_scale(geom_rect *r, double factor) {
    if (factor < 0) {
        return GEOM_NEGATIVE;
    }
    r->width *= factor;
    r->height *= factor;
    return GEOM_OK;
}
