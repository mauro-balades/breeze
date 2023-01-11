#include <stdio.h>
#include "Lexer.h"

int main() {
    const char* source = "hey";

    Lexer* lexer = lexer_init(source);
    lexer_tokenize(lexer);

}
