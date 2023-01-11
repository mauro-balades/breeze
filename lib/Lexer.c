
#include "Lexer.h"
#include "Token.h"
#include "utils.h"

#include <stdio.h>
#include <string.h>
#include <stdarg.h>

// private functions

static void lexer_error(Lexer* p_lexer, char *fmt, ...) {
    char* string = malloc(255);
    va_list arg_ptr;

    va_start(arg_ptr, fmt);
    vsprintf(string, fmt, arg_ptr);
    va_end(arg_ptr);

    printf(BRZ_ERROR_HEADER "%s [%zu:%zu]\n", string, p_lexer->line, p_lexer->col);
    exit(1);
}

static Token* lexer_create_result(enum TokenType ty, const char* content) {
    Token* tok = malloc(sizeof (Token));
    tok->type = ty;
    tok->content = content;

    return tok;
}

static Token* lexer_parse_char(Lexer* p_lexer) {

    switch (p_lexer->current_char) {
        case 0:
            BRZ_INTERNAL_ERROR()
            break;

        default:
            lexer_error(p_lexer, "Unexpected token found! ('%c')", p_lexer->current_char);
            break;
    }

    BRZ_INTERNAL_ERROR();
}

// lexer utility functions

static char lexer_next(Lexer* p_lexer) {
    p_lexer->current_char = *p_lexer->source++;
    return p_lexer->current_char;
}

static void lexer_add_token(Lexer* p_lexer, Token* token) {
	const size_t size = p_lexer->tok_count;
	const size_t capacity = p_lexer->tok_capacity;

	if (size == capacity) {
		const size_t new_capacity = (capacity * 3) / 2 + 1;
		p_lexer->tok_capacity = new_capacity;
		p_lexer->tokens = brz_realloc(p_lexer->tokens, new_capacity * sizeof(Token));
	}

	p_lexer->tokens[p_lexer->tok_count++] = *token;
}

// public functions

Lexer* lexer_init(const char* p_source) {
    Lexer* lexer = brz_malloc(sizeof (Lexer));

    lexer->source = p_source;
    lexer->old_source = p_source;

    lexer->current_char = *lexer->source;

    lexer->line = 0;
    lexer->col = 0;

    return lexer;
}

void lexer_tokenize(Lexer* p_lexer) {

    while (p_lexer->current_char != 0) {
        lexer_parse_char(p_lexer);
    }
}

