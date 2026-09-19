.intel_syntax noprefix

.section .text
	.global _start
	.extern main
	.type main, @function
	.extern _Exit
	.type _Exit, @function

	_start:
		endbr64
		call main
		# Move the value returned by `main` into `rdi`.
		mov rdi, rax
		# TODO: Use actual `exit()` which cleans up libc & calls exit hooks.
		call _Exit

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", %progbits
