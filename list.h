#ifndef LIST_H
#define LIST_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
	size_t size;
	size_t capacity;
	size_t data_byte;
	void *data;
} List;

List *new_list(size_t data_byte, size_t capacity);
void list_append(List *list, void *data); 
void *list_get_element(List *list, size_t index);
void list_free(List *list);

#endif