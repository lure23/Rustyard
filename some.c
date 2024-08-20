/*
* This
*/
#include "some.h"

extern void surface(void *x);

void tunnel(Context *p) {
    p->count++;
    surface(p->vp);
}
