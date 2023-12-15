CC = clang
CFLAGS = -std=c11 -O3 -g -Wall -Wextra -Wpedantic -Wstrict-aliasing
CFLAGS += -Wno-pointer-arith -Wno-newline-eof -Wno-unused-parameter -Wno-gnu-statement-expression
CFLAGS += -Wno-gnu-compound-literal-initializer -Wno-gnu-zero-variadic-macro-arguments

FILES = *.c

.PHONY: all clean
all: dir sucocc

dir:
	mkdir -p builds

sucocc:
	$(CC) $(FILES) $(CFLAGS) -o builds/sucocc

run:
	@cd builds && ./sucocc
	@cd ..

clean:
	rm -rf builds
