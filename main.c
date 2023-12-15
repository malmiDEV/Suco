#include "suco.h"

// #include <stdbool.h>
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
// #include <string.h>

int main(int argc, char **arg) {
  const char *fil = "../test.suco";
  FILE *fp = fopen(fil, "rb");
  if (fp == NULL) {
    printf("ERROR LOAD SORUCE: %s\n", fil);
    goto done;
  }

  fseek(fp, 0, SEEK_END);
  uint64_t length = ftell(fp);
  fseek(fp, 0, SEEK_SET);
  char *buffer = calloc(1, length);
  fread(buffer, 1, length, fp);
  fclose(fp);

  compilation_unit(buffer);

  free(buffer);

done:
  return 0;
}
