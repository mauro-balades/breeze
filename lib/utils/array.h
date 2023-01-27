
#ifndef __BREEZE_NODE_ARRAY_H_
#define __BREEZE_NODE_ARRAY_H_

typedef struct {
  void* array;
  size_t used;
  size_t size;
} Array;

Array* initArray(size_t initialSize);
void insertArray(Array *a, void* element);
void freeArray(Array *a);

#endif // __BREEZE_NODE_ARRAY_H_