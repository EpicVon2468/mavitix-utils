.section .text
	.global __mavitix_unreachable_impl

	__mavitix_unreachable_impl:
		#if defined(__x86_64__) || defined(__amd64__)
		ud2
		#elif defined(__aarch64__)
		udf #0
		#else
		#error Unsupported target architecture for stub implementation of unreachable().
		#endif
