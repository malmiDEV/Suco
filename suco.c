#include "suco.h"
#include "list.h"


void compilation_unit(char *src) {
  Lexer *lex = init_lexer(src);
  lexer_get_token(lex);

  char *template = "\rname: <%s>\ttype: <%s>\n";

  for (size_t i = 0; i < lex->list->size; i++) {
    printf(template, 
      ((Token*)list_get_element(lex->list, i))->name,
      token_get_type_str(((Token*)list_get_element(lex->list, i))->type)
    );
  }
  lexer_free(lex);
}
