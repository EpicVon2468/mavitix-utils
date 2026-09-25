#define __MAVITIX_INTERNAL__
#include <internal/__defs.h>
#include <internal/rpmalloc.h>

#include <elf.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sysexits.h>

__LINKER_SYMBOL__(__preinit_array_start);
__LINKER_SYMBOL__(__preinit_array_end);
__LINKER_SYMBOL__(__init_array_start);
__LINKER_SYMBOL__(__init_array_end);
__LINKER_SYMBOL__(__fini_array_start);
__LINKER_SYMBOL__(__fini_array_end);

extern
int __libc_start_main(
	__main_t main,
	int argc,
	char **ubp_av,
	__init_t init,
	__fini_t fini,
	__fini_t rtld_fini,
	void (*stack_end)()
);
extern
int __mavitix_libc_init(
	int argc,
	char **argv,
	char **envp,
	__fini_t fini,
	__fini_t rtld_fini
);
__PROTECTED__ static
void __libc_init(__init_t _init);
extern
void __libc_fini(int status, void *_fini);

// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---libc-start-main-.html
int __libc_start_main(
	const __main_t main,
	const int argc,
	char **ubp_av,
	const __init_t init,
	const __fini_t fini,
	const __fini_t rtld_fini,
	// Evil fucking K&R function prototype
	void (*const)()
) {
	char **argv = ubp_av;
	char **envp = argv + argc + 1;
	if (!__mavitix_libc_init(argc, argv, envp, fini, rtld_fini)) {
		_Exit(EX_SOFTWARE);
	};
	__libc_init(init);

	if __likely (main != NULL) {
		// SAFETY:
		// Problem(s):
		// - Dereferencing a null ptr is Undefined Behaviour.
		// - The `main` parameter is not guaranteed to be non-null.
		// - Dereferencing `main` if it is null would thus be UB.
		// Excuse(s):
		// - We perform a null check before dereference.
		exit((*main)(argc, argv, envp));
	} else {
		_Exit(EX_SOFTWARE);
	};
}

__NOTHROW__ extern
int __mavitix_libc__stdio_init(void);
__NOTHROW__ extern
int __mavitix_libc__stdio_fini(void);

__PROTECTED__ static
void __libc_init(const __init_t _init) {
	size_t index;

	{
		const size_t size = __preinit_array_end - __preinit_array_start;
		index = 0;
		while (index < size) {
#ifdef __MAVITIX_VERBOSE_UB_CHECKS
			void (*const fn_ptr)(void) = __preinit_array_start[index];
			if __likely (fn_ptr != NULL) {
				// SAFETY:
				// Problem(s):
				// - Dereferencing a null ptr is Undefined Behaviour.
				// - Elements within the preinit array are ptrs to functions.
				// - Dereferencing such a ptr if it is null would thus be UB.
				// Excuse(s):
				// - We perform a null check before dereference.
				(*fn_ptr)();
			};
#else
			// SAFETY:
			// Problem(s):
			// - Dereferencing a null ptr is Undefined Behaviour.
			// - Elements within the preinit array are ptrs to functions.
			// - Dereferencing such a ptr if it is null would thus be UB.
			// Excuse(s):
			// - The linker takes responsibility to guarantee the array is
			// well-formed.
			(*__preinit_array_start[index])();
#endif
			index += 1;
		};
	};

	if __likely (_init != NULL) {
		// SAFETY:
		// Problem(s):
		// - Dereferencing a null ptr is Undefined Behaviour.
		// - The `_init` parameter is not guaranteed to be non-null.
		// - Dereferencing `_init` if it is null would thus be UB.
		// Excuse(s):
		// - We perform a null check before dereference.
		(*_init)();
	};

	{
		const size_t size = __init_array_end - __init_array_start;
		index = 0;
		while (index < size) {
#ifdef __MAVITIX_VERBOSE_UB_CHECKS
			void (*const fn_ptr)(void) = __init_array_start[index];
			if __likely (fn_ptr != NULL) {
				// SAFETY:
				// Problem(s):
				// - Dereferencing a null ptr is Undefined Behaviour.
				// - Elements within the init array are ptrs to functions.
				// - Dereferencing such a ptr if it is null would thus be UB.
				// Excuse(s):
				// - We perform a null check before dereference.
				(*fn_ptr)();
			};
#else
			// SAFETY:
			// Problem(s):
			// - Dereferencing a null ptr is Undefined Behaviour.
			// - Elements within the init array are ptrs to functions.
			// - Dereferencing such a ptr if it is null would thus be UB.
			// Excuse(s):
			// - The linker takes responsibility to guarantee the array is
			// well-formed.
			(*__init_array_start[index])();
#endif
			index += 1;
		};
	};
}

void __libc_fini(const int, void *const _fini) {
	size_t index = __fini_array_end - __fini_array_start;
	while (index-- > 0) {
#ifdef __MAVITIX_VERBOSE_UB_CHECKS
		void (*const fn_ptr)(void) = __fini_array_start[index];
		if __likely (fn_ptr != NULL) {
			// SAFETY:
			// Problem(s):
			// - Dereferencing a null ptr is Undefined Behaviour.
			// - Elements within the fini array are ptrs to functions.
			// - Dereferencing such a ptr if it is null would thus be UB.
			// Excuse(s):
			// - We perform a null check before dereference.
			(*fn_ptr)();
		};
#else
		// SAFETY:
		// Problem(s):
		// - Dereferencing a null ptr is Undefined Behaviour.
		// - Elements within the fini array are ptrs to functions.
		// - Dereferencing such a ptr if it is null would thus be UB.
		// Excuse(s):
		// - The linker takes responsibility to guarantee the array is
		// well-formed.
		(*__fini_array_start[index])();
#endif
	};
#ifdef __MAVITIX_VERBOSE_UB_CHECKS
	if __likely (_fini != NULL) {
		// SAFETY:
		// Problem(s):
		// - Dereferencing a null ptr is Undefined Behaviour.
		// - The `_fini` parameter is not guaranteed to be non-null.
		// - Dereferencing `_fini` if it is null would thus be UB.
		// Excuse(s):
		// - We perform a null check before dereference.
		(*((__fini_t) _fini))();
	};
#else
	(*((__fini_t) _fini))();
#endif
}
