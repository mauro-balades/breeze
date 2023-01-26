
#include <stdio.h>

#ifndef __BREEZE_NODE_BASE_H_
#define __BREEZE_NODE_BASE_H_

enum NodeType {
    Unknown = -1
};

typedef struct {
    NodeType type;
} NodeBase;

#endif // __BREEZE_NODE_BASE_H_
