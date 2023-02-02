
#include <stdio.h>
#include "Base.h"

#ifndef __BREEZE_NODE_FUNCTION_CALL_H_
#define __BREEZE_NODE_FUNCTION_CALL_H_

typedef struct FnCall
{
    union {
      struct Node* base;
    } base;
    
    const char* name;
    
} FnCall;


#endif // __BREEZE_NODE_FUNCTION_CALL_H_
