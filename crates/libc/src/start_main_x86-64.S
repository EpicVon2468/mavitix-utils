.intel_syntax noprefix

.section .text
	.global _start
	.extern main
	.extern _Exit

	_start:
		endbr64
		call main
		# Move the value returned by `main` into `rdi`.
		mov rdi, rax
		# TODO: Use actual `exit()` which cleans up libc & calls exit hooks.
		call _Exit
