#ifndef LEXER_H
#define LEXER_H

#include "list.h"

typedef struct {
   char *name;
   int type;
} Token;

typedef struct {
   int pos;
   int line; 
   List *list;
   char *src;
} Lexer;

Lexer *init_lexer(char *src);
void lexer_get_token(Lexer *lexer);
char *lexer_get_type_str(int type);
void lexer_free(Lexer *lexer);

#endif