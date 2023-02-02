
#include "array.h"
#include "../nodes/Base.h"

extern Array* initArray(size_t initialSize) {

    Array* a = malloc(sizeof (Array));

    a->array = malloc(initialSize * sizeof(int));
    a->used = 0;
    a->size = initialSize;

    return a;
}

extern void insertArray(struct Array *a, struct Node* element) {
    // a->used is the number of used entries, because a->array[a->used++] updates a->used only *after* the array has been accessed.
    // Therefore a->used can go up to a->size 
    if (a->used == a->size) {
        a->size *= 2;
        a->array = realloc(a->array, a->size * sizeof(int));
    }
    a->array[a->used++] = *element;
}

extern void freeArray(struct Array *a) {
    free(a->array);
    a->array = NULL;
    a->used = a->size = 0;
}
