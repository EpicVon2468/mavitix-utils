.intel_syntax noprefix

.file "getgid.s"

.section .text
	.global getgid
	.type getgid, @function

	.global getegid
	.type getegid, @function

	getgid:
		endbr64
		mov rax, 0x68
		syscall
		ret

	getegid:
		endbr64
		mov rax, 0x6C
		syscall
		ret

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", @progbits
