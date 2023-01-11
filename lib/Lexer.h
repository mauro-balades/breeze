
#include "Token.h"

#include <stdlib.h>

#ifndef __BREEZE_LEXER_H_
#define __BREEZE_LEXER_H_

typedef struct
{
    Token* tokens;
	size_t tok_count;
	size_t tok_capacity;

    const char* current_char;
    size_t current_tok_index;

    const char* source;
} Lexer;

Lexer* lexer_init(const char* p_source);
void lexer_tokenize(Lexer* p_lexer);

#endif // __BREEZE_LEXER_H_
