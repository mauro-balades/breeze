
typedef struct {
  int *array;
  size_t used;
  size_t size;
} Array;

Array* initArray(size_t initialSize);
void insertArray(Array *a, int element);
void freeArray(Array *a);
