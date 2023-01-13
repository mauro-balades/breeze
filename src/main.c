#include <stdio.h>
#include "lexer.h"

int main(int argc, char* argv[]) {
    yyin = fopen(argv[1], "r");
    if(!yyin) {
        printf("couldn't open file for reading\n");
        return 1;
    }

    yydebug = 1;
    return yyparse();

}
