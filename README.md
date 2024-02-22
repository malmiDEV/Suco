# Suco
A Suco language. My programming language that i create for writing malmiOS

Suco compiler generate x86 assembly.

for example this code: 
```
defun main() -> i32 {
  let value: u32 = 6974;
  return 32;
}
```

compiler generate something like this

```asm
main:
	push ebp
	mov ebp, esp
	sub esp, 16
	mov dword [ebp-4], 6974
	mov eax, 32
	mov esp, ebp
	pop ebp
	ret
```

Suco language does not depend on any os like linux or mac.

It's mean language contain only few basic features like pointers, function, variables, loops, if else swtich-case statements and etc.

But it will change when i implement my own binary format and linker in malmiOS.
