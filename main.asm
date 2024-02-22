extra:
	push ebp
	mov ebp, esp
	sub esp, 32
	mov dword [ebp-4], 23
	mov dword [ebp-8], 23
	mov dword [ebp-12], 23
	mov dword [ebp-16], 23
	mov dword [ebp-20], 23
	mov al, 12
	mov esp, ebp
	pop ebp
	ret
extraFunctionThatReturnSomeValueAndCreateVariable:
	push ebp
	mov ebp, esp
	sub esp, 16
	mov byte [ebp-1], 122
	mov word [ebp-4], 124
	mov dword [ebp-8], 12
	mov byte [ebp-9], 12
	mov dword [ebp-16], 12
	mov ax, 6974
	mov esp, ebp
	pop ebp
	ret
main:
	push ebp
	mov ebp, esp
	sub esp, 16
	mov dword [ebp-4], 12
	mov eax, 122
	mov esp, ebp
	pop ebp
	ret

