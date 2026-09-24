.intel_syntax noprefix

.section .text
	.global __mavitix_unreachable_impl
	.type __mavitix_unreachable_impl, @function

	__mavitix_unreachable_impl:
		#if defined(__x86_64__) || defined(__amd64__)
		ud2
		#elif defined(__aarch64__)
		udf #0
		#else
		#error Unsupported target architecture for stub implementation of unreachable().
		#endif

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", @progbits
