
#include "Token.h"

#include <stdlib.h>

#ifndef __BREEZE_LEXER_H_
#define __BREEZE_LEXER_H_

typedef struct
{
    Token* tokens;
	size_t tok_count;
	size_t tok_capacity;

    char current_char;

    const char* source;
    const char* old_source;

    size_t line;
    size_t col;
} Lexer;

Lexer* lexer_init(const char* p_source);
void lexer_tokenize(Lexer* p_lexer);

#endif // __BREEZE_LEXER_H_
