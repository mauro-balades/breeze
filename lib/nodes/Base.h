
#include <stdio.h>

#ifndef __BREEZE_NODE_BASE_H_
#define __BREEZE_NODE_BASE_H_

typedef enum {
    Unknown = -1
} NodeType;

typedef struct Node {
    NodeType type;
} Node;

#endif // __BREEZE_NODE_BASE_H_
