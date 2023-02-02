#include <stdlib.h>
#include "../nodes/Base.h"

#ifndef __BREEZE_NODE_ARRAY_H_
#define __BREEZE_NODE_ARRAY_H_

typedef struct Array {
  struct Node* array;
  size_t used;
  size_t size;
} Array;

extern Array* initArray(size_t initialSize);
extern void insertArray(struct Array *a, struct Node* element);
extern void freeArray(struct Array *a);

#endif // __BREEZE_NODE_ARRAY_H_