%{
#include <stdio.h>
#include "nodes/all.h"

#include "utils/array.h"

Array* ast = initArray(0); /* the top level root node of our final AST */

int yylex();
int yyerror(char *s);

%}

%token	IDENTIFIER I_CONSTANT F_CONSTANT STRING_LITERAL
%token	PTR_OP INC_OP DEC_OP LEFT_OP RIGHT_OP LE_OP GE_OP EQ_OP NE_OP
%token	AND_OP OR_OP MUL_ASSIGN DIV_ASSIGN MOD_ASSIGN ADD_ASSIGN
%token	SUB_ASSIGN LEFT_ASSIGN RIGHT_ASSIGN AND_ASSIGN
%token	XOR_ASSIGN OR_ASSIGN

%start prog
%%

prog:
    | prog stmt
    ;

stmt
    : /*TODo*/
    | expr
    ;

expr
    : function_call
    | IDENTIFIER
    | I_CONSTANT
    | F_CONSTANT
    | STRING_LITERAL
    | array_expr
    ;

// expressions

array_expr
    : '[' array_body ']'
    ;

array_body
    : /* empty */
    | array_inner_exprs
    ;

array_inner_exprs
    : expr
    | function_exprs ',' expr
    ;


// statements

function_call
    : IDENTIFIER '(' function_arguments ')' {
        $$ = 
    }
    ;

function_arguments
    : /* empty */
    | function_exprs

function_exprs
    : function_exprs_dec
    | function_exprs ',' function_exprs_dec

function_exprs_dec
    : IDENTIFIER '=' expr
    | expr
    ;

%%

int yyerror(char *s)
{
	printf("Syntax Error on line %s\n", s);
	return 0;
}