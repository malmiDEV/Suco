#include "lexer.h"

#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>

static const struct {
   char *name; 
   int type;
} Keyword[] = {
   {"def", TOKEN_DEF},
   {"while", TOKEN_WHILE},
   {"for", TOKEN_FOR},
   {"let", TOKEN_LET},
   {"return", TOKEN_RETURN},
   {"if", TOKEN_IF},
   {"elif", TOKEN_ELIF},
   {"else", TOKEN_ELSE},
   {"match", TOKEN_MATCH}
};

static const struct {
   char *name; 
   int type;
} Types[] = {
   {"0U", TOKEN_UINT0},
   {"64I", TOKEN_UINT64},
   {"64U", TOKEN_INT64},
   {"32I", TOKEN_INT32},
   {"32U", TOKEN_UINT32},
   {"16I", TOKEN_INT16},
   {"16U", TOKEN_UINT16},
   {"8I", TOKEN_INT8},
   {"8U", TOKEN_UINT8},
   {"64F", TOKEN_FLOAT64},
};

static const struct {
   char *name; 
   int type;
} Symbols[] = {
   {"(", TOKEN_LPAREN},
   {")", TOKEN_RPAREN},
   {"{", TOKEN_LBRACE},
   {"}", TOKEN_RBRACE},
   {"<", TOKEN_LARROW},
   {">", TOKEN_RARROW},
   {"->", TOKEN_ARROW},
   {"=>", TOKEN_EQUAL_ARROW},
   {":", TOKEN_COLON},
   {"=", TOKEN_EQUAL},
   {"==", TOKEN_EQUAL_TO},
   {"!=", TOKEN_NOT_EQUAL},
   {";", TOKEN_SEMI},
   {"+", TOKEN_PLUS},
   {"++", TOKEN_INC},
   {"+=", TOKEN_INC},
   {"-", TOKEN_MINUS},
   {"--", TOKEN_DEC},
   {"-=", TOKEN_DEC},
};

Lexer *init_lexer(char *src) {
   Lexer *lexer = malloc(sizeof(Lexer));
   if (!lexer) {
      puts("Error Init Lexer");
      exit(-1);
   }
   lexer->list = new_list(sizeof(Lexer),0);
   lexer->line = 0;
   lexer->pos = 0;
   lexer->src = src;
   lexer->src[lexer->pos] = lexer->src[lexer->pos];
   return lexer;
}

static char lexer_src_offestting(Lexer *lex, size_t offset) {
   return lex->src[lex->pos + offset];
}

static void lexer_whitespace(Lexer *lexer) {
   while (
      lexer->src[lexer->pos] == 0x0A || 
      lexer->src[lexer->pos] == 0x0D || 
      lexer->src[lexer->pos] == ' '  || 
      lexer->src[lexer->pos] == '\t'
   ) {
      lexer->pos++;
   }
}


static void lexer_process_token(Lexer *lexer, Token tok) {
   Token token = {0};
   token.name = tok.name;
   token.type = tok.type;
   lexer->pos++;
   list_append(lexer->list, &token);
}

static void lexer_process_token_advance(Lexer *lexer, Token tok) {
   Token token = {0};
   token.name = tok.name;
   token.type = tok.type;
   lexer->pos+=2;
   list_append(lexer->list, &token);
}

static Token lexer_get_id(Lexer *lexer) {
   char *buffer = malloc(sizeof(char));
   while (isalpha(lexer->src[lexer->pos])) {
      buffer = realloc(buffer, (strlen(buffer) + 2) * sizeof(char));
      strcat(buffer, (char[]){lexer->src[lexer->pos],0});
      lexer->pos++;
   }

   Token tok = {0};
   for (int i = 0; i < 7; i++) {
      if (strcmp(buffer, Keyword[i].name) == 0) {
         tok.name = Keyword[i].name;
         tok.type = Keyword[i].type;
         goto end;
      }
   }

   tok.name = buffer;
   tok.type = TOKEN_ID;
end:
   return tok;
}

static Token lexer_get_number(Lexer *lexer) {
   char *buffer = malloc(sizeof(char));
   while (isdigit(lexer->src[lexer->pos])) {
      buffer = realloc(buffer, (strlen(buffer) + 2) * sizeof(char));
      strcat(buffer, (char[]){lexer->src[lexer->pos],0});
      lexer->pos++;
   }

   Token tok = {0};
   if (isalpha(lexer->src[lexer->pos])) {
      char *dst = strcat(buffer, (char[]){lexer->src[lexer->pos],0});
      for (int i = 0; i < 9; i++) {
         if (strcmp(dst, Types[i].name) == 0) {
            tok.name = Types[i].name;
            tok.type = Types[i].type;
            lexer->pos++;
            goto end;
         }
      }
   }
   tok.name = buffer;
   tok.type = TOKEN_INTCONST;
end:
   return tok;
}

void lexer_get_token(Lexer *lexer) {
   Token tok = {0};
   while (lexer->src[lexer->pos] != '\0') {
      printf("%c\n", lexer->src[lexer->pos]);
      lexer_whitespace(lexer);
      if (isalpha(lexer->src[lexer->pos])) {
         tok = lexer_get_id(lexer);
         list_append(lexer->list, &tok);
      }
      if (isdigit(lexer->src[lexer->pos])) {
         tok = lexer_get_number(lexer);
         list_append(lexer->list, &tok);
      }
         
      switch(lexer->src[lexer->pos]) {
         case '(': lexer_process_token(lexer, (Token){"(", TOKEN_LPAREN}); break;
         case ')': lexer_process_token(lexer, (Token){")", TOKEN_RPAREN}); break;
         case '{': lexer_process_token(lexer, (Token){"{", TOKEN_LBRACE}); break;
         case '}': lexer_process_token(lexer, (Token){"}", TOKEN_RBRACE}); break;
         case ';': lexer_process_token(lexer, (Token){";", TOKEN_SEMI});   break;
         case ':': lexer_process_token(lexer, (Token){":", TOKEN_COLON});  break;
         case ',': lexer_process_token(lexer, (Token){",", TOKEN_COMMA});  break;
         case '=': 
            if (lexer_src_offestting(lexer,1)=='=') { lexer_process_token_advance(lexer, (Token){"==", TOKEN_EQUAL_TO}); break; }
            lexer_process_token(lexer, (Token){"=",TOKEN_EQUAL}); break;
         case '<': 
            if (lexer_src_offestting(lexer,1)=='=') { lexer_process_token_advance(lexer, (Token){"<=", TOKEN_LARROW_EQ}); break; }
            lexer_process_token(lexer, (Token){"<", TOKEN_LARROW}); break;
         case '>': 
            if (lexer_src_offestting(lexer,1)=='=') { lexer_process_token_advance(lexer, (Token){">=", TOKEN_RARROW_EQ}); break; }
            lexer_process_token(lexer, (Token){">", TOKEN_RARROW}); break;
         case '!':
            if (lexer_src_offestting(lexer,1)=='=') { lexer_process_token_advance(lexer, (Token){"!=", TOKEN_NOT_EQUAL}); break; }
            lexer_process_token(lexer, (Token){"!", TOKEN_NOT});   break;
         case '+':
            if (lexer_src_offestting(lexer,1)=='=') { lexer_process_token_advance(lexer, (Token){"+=", TOKEN_INC}); break; }
            if (lexer_src_offestting(lexer,1)=='+') { lexer_process_token_advance(lexer, (Token){"++", TOKEN_INC}); break; }
            lexer_process_token(lexer, (Token){"+", TOKEN_PLUS});  break;
         case '-':
            if (lexer_src_offestting(lexer,1)=='>') { lexer_process_token_advance(lexer, (Token){"->", TOKEN_ARROW}); break; }
            if (lexer_src_offestting(lexer,1)=='-') { lexer_process_token_advance(lexer, (Token){"--", TOKEN_DEC}); break; }
            if (lexer_src_offestting(lexer,1)=='=') { lexer_process_token_advance(lexer, (Token){"-=", TOKEN_DEC}); break; }
            lexer_process_token(lexer, (Token){"-", TOKEN_MINUS}); break;
         case '*': lexer_process_token(lexer, (Token){"*", TOKEN_MULT}); break;
         case '&': lexer_process_token(lexer, (Token){"&", TOKEN_BITAND}); break;
      }
   }
   printf("Unexpected Token: (%c) AT: (%d)\n ", lexer->src[lexer->pos], lexer->pos);
}

void lexer_free(Lexer *lexer) {
   if (!lexer) {
      list_free(lexer->list);
      free(lexer);
   }
}

