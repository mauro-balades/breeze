
#include <stdlib.h>
#include <stdio.h>

#include "utils.h"

void *brz_malloc(size_t n)
{
	void *p = malloc(n);
    BZR_NO_NULL(p, n)

	return p;
}

void *brz_realloc(void *p, size_t n)
{
	void *new_p = realloc(p, n);
    BZR_NO_NULL(new_p, n)

	return new_p;
}
