
#include <stdlib.h>

#ifndef __BREEZE_UTILS_H_
#define __BREEZE_UTILS_H_

#define BRZ_ANSI_CLR_RED     "\x1b[31m"
#define BRZ_ANSI_CLR_GREEN   "\x1b[32m"
#define BRZ_ANSI_CLR_YELLOW  "\x1b[33m"
#define BRZ_ANSI_CLR_BLUE    "\x1b[34m"
#define BRZ_ANSI_CLR_MAGENTA "\x1b[35m"
#define BRZ_ANSI_CLR_CYAN    "\x1b[36m"
#define BRZ_ANSI_CLR_INFO    "\x1b[1;36m"
#define BRZ_ANSI_CLR_WARNING "\x1b[1;33m"
#define BRZ_ANSI_CLR_ERROR   "\x1b[1;31m"
#define BRZ_ANSI_CLR_RESET   "\x1b[0m"

#define BRZ_INFO_HEADER    BRZ_ANSI_CLR_INFO    "info: "    BRZ_ANSI_CLR_RESET
#define BRZ_WARNING_HEADER BRZ_ANSI_CLR_WARNING "warning: " BRZ_ANSI_CLR_RESET
#define BRZ_ERROR_HEADER   BRZ_ANSI_CLR_ERROR   "error: "   BRZ_ANSI_CLR_RESET

#define BRZ_ERROR_BUFF_SIZE 255

#define BRZ_INTERNAL_ERROR() \
    printf(BRZ_ERROR_HEADER "Internal breeze error"); \
    exit(2);

#define BZR_NO_NULL(p, n) \
    if (p == NULL && n > 0) { \
        BRZ_INTERNAL_ERROR() \
    }

void *brz_malloc(size_t n);
void *brz_realloc(void *p, size_t n);

#endif // __BREEZE_UTILS_H_
