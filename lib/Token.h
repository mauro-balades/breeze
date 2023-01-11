
#ifndef __BREEZE_TOKEN_H_
#define __BREEZE_TOKEN_H_

typedef struct Token
{
    const char* content;
    enum TokenType {
        Str, Idnt,

        Assign, Cond, Lor, Lan, Or, Xor, And, Eq, Ne, Lt, Gt, Le, Ge,
        Shl, Shr, Add, Sub, Mul, Div, Mod, Inc, Dec, Brak
    } type;
} Token;

#endif // __BREEZE_TOKEN_H_
