# Suco
A Suco language. My programming language that i create for writing malmiOS

Suco language generate raw 32bit binary,
it's like use pure assembly.

for example this code: 
```
defun main() -> u32 {
  return 32;
}
```

compiler generate something like this

```asm
main:
  push ebp
  mov ebp, esp
  mov eax, 32
  mov esp, ebp
  pop ebp
  ret
```

Suco language does not depend on any os like linux or mac.
It's mean language contain only few basic features like pointers, function, variables, loops, if else swtich-case statements and etc.
But it will change when my own binary format, linker in malmiOS is completed.
