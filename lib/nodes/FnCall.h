
#include <stdio.h>
#include "Base.h"

#ifndef __BREEZE_NODE_FUNCTION_CALL_H_
#define __BREEZE_NODE_FUNCTION_CALL_H_


struct FnCall
{
    union {
      struct NodeBase base;
    } base;
    
    const char* name;
    
};


#endif // __BREEZE_NODE_FUNCTION_CALL_H_
