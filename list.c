#include "list.h"

List *new_list(size_t data_byte, size_t capacity) {
	List *list = malloc(sizeof(List));
	list->capacity = capacity;
	list->data_byte = data_byte;
	list->size = 0;
	list->data = malloc(data_byte * capacity);

	if (list == NULL || list->data == NULL) {
		puts("Error Allocate Memory for List");
		exit(-1);
	}
	return list;
}

void list_append(List *list, void *data) {
	if (list->size == list->capacity) {
		list->capacity = list->capacity > 0 ? list->capacity * 2 : 1;
		void *data = realloc(list->data, list->data_byte * list->capacity);
		if (!data) 
			goto err;
		list->data = data;
	}
	memcpy((uint8_t*)list->data + (list->size * list->data_byte), data, list->data_byte);
	list->size++;
	goto done;
err:
	puts("Error append data to list");
	exit(-1);
done:
	return;
}

void list_remove(List *list, size_t index) {
	if (list->size == 0) {
		printf("List is empty\n");
		exit(-1);
	} 
	if (index >= list->capacity) {
		printf("Maximum Capacity is %zu :: but index %zu\n", list->capacity, index);
		exit(-1);
	} 
	if (list->size == 1) {
		list->size = 0;
		goto done;
	}
	--list->size;
	uint8_t *dst = (uint8_t*)list->data + (index * list->data_byte);
	uint8_t *src = (uint8_t*)list->data + (list->size * list->data_byte);
	memcpy(dst, src, list->data_byte);
done:
	return;
} 

void *list_get_element(List *list, size_t index) {
	if (index >= list->capacity) {
		printf("Maximum Capacity is %zu :: but index %zu\n", list->capacity, index);
		exit(-1);
	}
	return (uint8_t*)list->data + (index * list->data_byte);
}

void list_free(List *list) {
	if (!list) {
		free(list->data);
		free(list);
	}
}