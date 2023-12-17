#include "token.h"

Token get_token(char *name, int type) {
   Token tok = {0};
   tok.name = name;
   tok.type = type;
   return tok;
}

char *token_get_type_str(int type) {
   switch (type) {
      case TOKEN_NONE:         return "TOKEN_NONE";   
      case TOKEN_WHITESPACE:   return "TOKEN_WHITESPACE";
      case TOKEN_DEF:          return "TOKEN_DEF";
      case TOKEN_LET:          return "TOKEN_LET";   
      case TOKEN_WHILE:        return "TOKEN_WHILE";   
      case TOKEN_FOR:          return "TOKEN_FOR";
      case TOKEN_IF:           return "TOKEN_IF";
      case TOKEN_ELIF:         return "TOKEN_ELIF";  
      case TOKEN_ELSE:         return "TOKEN_ELSE";   
      case TOKEN_MATCH:        return "TOKEN_MATCH";
      case TOKEN_RETURN:       return "TOKEN_RETURN";      
      case TOKEN_ID:           return "TOKEN_ID";
      case TOKEN_LPAREN:       return "TOKEN_LPAREN";      
      case TOKEN_RPAREN:       return "TOKEN_RPAREN";      
      case TOKEN_LBRACE:       return "TOKEN_LBRACE";      
      case TOKEN_RBRACE:       return "TOKEN_RBRACE";      
      case TOKEN_LARROW:       return "TOKEN_LARROW";      
      case TOKEN_RARROW:       return "TOKEN_RARROW";   
      case TOKEN_LARROW_EQ:    return "TOKEN_LARROW_EQ";
      case TOKEN_RARROW_EQ:    return "TOKEN_RARROW_EQ";   
      case TOKEN_ARROW:        return "TOKEN_ARROW";   
      case TOKEN_EQUAL_ARROW:  return "TOKEN_EQUAL_ARROW";
      case TOKEN_COLON:        return "TOKEN_COLON";   
      case TOKEN_EQUAL:        return "TOKEN_EQUAL";  
      case TOKEN_EQUAL_TO:     return "TOKEN_EQUAL_TO";
      case TOKEN_NOT_EQUAL:    return "TOKEN_NOT_EQUAL";
      case TOKEN_SEMI:         return "TOKEN_SEMI";   
      case TOKEN_COMMA:        return "TOKEN_COMMA";
      case TOKEN_PLUS:         return "TOKEN_PLUS";   
      case TOKEN_INC:          return "TOKEN_INC";      
      case TOKEN_MINUS:        return "TOKEN_MINUS";   
      case TOKEN_DEC:          return "TOKEN_DEC";      
      case TOKEN_MULT:         return "TOKEN_MULT";
      case TOKEN_BITAND:       return "TOKEN_BITAND";
      case TOKEN_NOT:          return "TOKEN_NOT";
      case TOKEN_UINT0:        return "TOKEN_UINT0";
      case TOKEN_UINT64:       return "TOKEN_UINT64";
      case TOKEN_INT64:        return "TOKEN_INT64";
      case TOKEN_INT32:        return "TOKEN_INT32";
      case TOKEN_UINT32:       return "TOKEN_UINT32";
      case TOKEN_INT16:        return "TOKEN_INT16";
      case TOKEN_UINT16:       return "TOKEN_UINT16";
      case TOKEN_INT8:         return "TOKEN_INT8";
      case TOKEN_UINT8:        return "TOKEN_UINT8";
      case TOKEN_FLOAT64:      return "TOKEN_FLOAT64";
      case TOKEN_INTCONST:     return "TOKEN_INTCONST";      
      case TOKEN_FLOATCONST:   return "TOKEN_FLOATCONST";      
      default:                 return "TOKEN_NONE";
   }
}