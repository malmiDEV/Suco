#ifndef LEXER_H
#define LEXER_H

#include "list.h"
#include "token.h"

typedef struct {
   int pos;
   int line; 
   List *list;
   char *src;
   char cur;
} Lexer;

Lexer *init_lexer(char *src);
void lexer_get_token(Lexer *lexer);
void lexer_free(Lexer *lexer);

#endif