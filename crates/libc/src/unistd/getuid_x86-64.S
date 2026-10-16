.intel_syntax noprefix

.file "getuid.s"

.section .text
	.global getuid
	.type getuid, @function

	.global geteuid
	.type geteuid, @function

	getuid:
		endbr64
		mov rax, 0x66
		syscall
		ret

	geteuid:
		endbr64
		mov rax, 0x6B
		syscall
		ret

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", @progbits
