
#include <stdio.h>

#ifndef __BREEZE_LEXER_EXPORT_H_
#define __BREEZE_LEXER_EXPORT_H_

extern int yyparse();
extern FILE* yyin;
extern int yydebug = 0;

#endif // __BREEZE_LEXER_EXPORT_H_
