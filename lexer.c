#include "lexer.h"

#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>

enum {
   TOKEN_NONE,
   TOKEN_WHITESPACE,
   TOKEN_FN,
   TOKEN_LET,
   TOKEN_WHILE,
   TOKEN_IF,
   TOKEN_ELIF,
   TOKEN_ELSE,
   TOKEN_RETURN,
   TOKEN_ID,
   TOKEN_LPAREN,
   TOKEN_RPAREN,
   TOKEN_LBRACE,
   TOKEN_RBRACE,
   TOKEN_LARROW,
   TOKEN_RARROW,
   TOKEN_ARROW,
   TOKEN_COLON,
   TOKEN_EQUAL,
   TOKEN_SEMI,
   TOKEN_PLUS,
   TOKEN_PLUSINC,
   TOKEN_MINUS,
   TOKEN_MINUSINC,
   TOKEN_INT32,
   TOKEN_UINT32,
   TOKEN_FLOAT32,
   TOKEN_INTCONST,
   TOKEN_FLOATCONST,
} TokenKind;

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
   return lexer;
}

char *lexer_get_type_str(int type) {
   switch (type) {
      case TOKEN_NONE:        return "TOKEN_NONE";   
      case TOKEN_WHITESPACE:  return "TOKEN_WHITESPACE";
      case TOKEN_FN:          return "TOKEN_FN";
      case TOKEN_LET:         return "TOKEN_LET";   
      case TOKEN_WHILE:       return "TOKEN_WHILE";   
      case TOKEN_IF:          return "TOKEN_IF";
      case TOKEN_ELIF:        return "TOKEN_ELIF";  
      case TOKEN_ELSE:        return "TOKEN_ELSE";   
      case TOKEN_RETURN:      return "TOKEN_RETURN";      
      case TOKEN_ID:          return "TOKEN_ID";
      case TOKEN_LPAREN:      return "TOKEN_LPAREN";      
      case TOKEN_RPAREN:      return "TOKEN_RPAREN";      
      case TOKEN_LBRACE:      return "TOKEN_LBRACE";      
      case TOKEN_RBRACE:      return "TOKEN_RBRACE";      
      case TOKEN_LARROW:      return "TOKEN_LARROW";      
      case TOKEN_RARROW:      return "TOKEN_RARROW";      
      case TOKEN_ARROW:       return "TOKEN_ARROW";   
      case TOKEN_COLON:       return "TOKEN_COLON";   
      case TOKEN_EQUAL:       return "TOKEN_EQUAL";  
      case TOKEN_SEMI:        return "TOKEN_SEMI";   
      case TOKEN_PLUS:        return "TOKEN_PLUS";   
      case TOKEN_PLUSINC:     return "TOKEN_PLUSINC";      
      case TOKEN_MINUS:       return "TOKEN_MINUS";   
      case TOKEN_MINUSINC:    return "TOKEN_MINUSINC";      
      case TOKEN_INT32:       return "TOKEN_INT32";   
      case TOKEN_UINT32:      return "TOKEN_UINT32";    
      case TOKEN_FLOAT32:     return "TOKEN_FLOAT32";    
      case TOKEN_INTCONST:    return "TOKEN_INTCONST";      
      case TOKEN_FLOATCONST:  return "TOKEN_FLOATCONST";      
      default:                return "TOKEN_NONE";
   }
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

static Token lexer_get_id(Lexer *lexer) {
   static const struct {
      char *name; int type;
   } Keyword[] = {
      {"fn", TOKEN_FN},
      {"while", TOKEN_WHILE},
      {"let", TOKEN_LET},
      {"return", TOKEN_RETURN},
      {"if", TOKEN_IF},
      {"elif", TOKEN_ELIF},
      {"else", TOKEN_ELSE},
   };

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
   static const struct {
      char *name; int type;
   } Types[] = {
      {"32I", TOKEN_INT32},
      {"32U", TOKEN_UINT32},
      {"32F", TOKEN_FLOAT32}
   };

   char *buffer = malloc(sizeof(char));
   while (isdigit(lexer->src[lexer->pos])) {
      buffer = realloc(buffer, (strlen(buffer) + 2) * sizeof(char));
      strcat(buffer, (char[]){lexer->src[lexer->pos],0});
      lexer->pos++;
   }

   Token tok = {0};
   if (isalpha(lexer->src[lexer->pos])) {
      char *dst = strcat(buffer, (char[]){lexer->src[lexer->pos],0});
      for (int i = 0; i < 3; i++) {
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
         case '(':
            tok.name = "(";
            tok.type = TOKEN_LPAREN;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case ')':
            tok.name = ")";
            tok.type = TOKEN_RPAREN;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case '{':
            tok.name = "{";
            tok.type = TOKEN_LBRACE;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case '}':
            tok.name = "}";
            tok.type = TOKEN_RBRACE;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case '<':
            tok.name = "<";
            tok.type = TOKEN_LARROW;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case ';':
            tok.name = ";";
            tok.type = TOKEN_SEMI;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case ':':
            tok.name = ":";
            tok.type = TOKEN_COLON;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case '=':
            tok.name = "=";
            tok.type = TOKEN_EQUAL;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case '+':
            if (lexer_src_offestting(lexer, 1) == '+') {
               tok.name = "++";
               tok.type = TOKEN_PLUSINC;
               lexer->pos += 2;
               list_append(lexer->list, &tok);
               break;
            }
            tok.name = "+";
            tok.type = TOKEN_PLUS;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
         case '-':
            if (lexer_src_offestting(lexer, 1) == '>') {
               tok.name = "->";
               tok.type = TOKEN_ARROW;
               lexer->pos += 2;
               list_append(lexer->list, &tok);
               break;
            }
            if (lexer_src_offestting(lexer, 1) == '-') {
               tok.name = "--";
               tok.type = TOKEN_MINUSINC;
               lexer->pos += 2;
               list_append(lexer->list, &tok);
               break;
            }
            tok.name = "-";
            tok.type = TOKEN_MINUS;
            lexer->pos++;
            list_append(lexer->list, &tok);
            break;
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

