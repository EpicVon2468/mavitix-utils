.intel_syntax noprefix

.section .text
	.global _start

	.global __stack_end__stub
	.type __stack_end__stub, @function

	.extern main
	.type main, @function

	.extern _init
	.type _init, @function

	.extern _fini
	.type _fini, @function

	.extern __libc_start_main
	.type __libc_start_main, @function

	__stack_end__stub:
		endbr64
		ret

	_start:
		endbr64
		# "The content of this register is unspecified at process initialisation time, but the user code should mark the deepest stack frame by setting the frame pointer to zero"
		xor rbp, rbp
		# stack ptr alignment
		and rsp, -16
		# load main ptr
		lea rdi, main
		# argc
		mov rsi, rsp
		# "a function pointer that the application should register with atexit (BA_OS)." (rtld_fini)
		mov r9, rdx
		# upb_av (argv + envp)
		lea rdx, [rsp+0x8]
		# init
		lea rcx, _init
		# fini
		lea r8, _fini
		# begin "evil fucking thing I copied from godbolt"
		lea rax, __stack_end__stub
		mov rsp, rax
		# end
		call __libc_start_main
		ud2

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", @progbits
