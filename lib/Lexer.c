
#include "Lexer.h"
#include "Token.h"

#include <stdio.h>

// private functions

struct LexResult {
    enum TokenType ty;
    const char* content;
};

struct LexResult lexer_create_result(enum TokenType ty, const char* content) {
    struct LexResult res = { .ty = ty, .content = content };
    return res;
}

// struct LexResult lexer_parse_char(Lexer* p_lexer) {

// }

// lexer utility functions

const char* lexer_next(Lexer* p_lexer) {
    p_lexer->current_tok_index++;
    p_lexer->current_char = &p_lexer->source[p_lexer->current_tok_index];

    return p_lexer->current_char;
}

// public functions

Lexer* lexer_init(const char* p_source) {
    Lexer* lexer = malloc(sizeof (Lexer));

    lexer->source = p_source;
    lexer->current_tok_index = -1;

    lexer_next(lexer);

    return lexer;
}

void lexer_tokenize(Lexer* p_lexer) {
    printf("s: %s\n", p_lexer->source);
}

