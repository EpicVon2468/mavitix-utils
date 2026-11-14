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

// Apparently `callback` is only allowed once.
// Reasoning:
// """
// 1. The main reason I believe is that it’s modelled after destructors, and it
// doesn’t usually make sense to have more than one destructor for something
// 2. This is a GCC attribute, and GCC only supports specifying it once (I
// believe it ignores any additional ones; Clang used to do that too, but we and
// GCC disagreed on which ones were ignored, so I updated our implementation to
// be what GCC does in addition to adding that error)
// """
__CALLBACK__(main, argc, ubp_av, __) extern
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
static
void __libc_init(__init_t init);
extern
void __libc_fini(int status, void *fini);

// https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---libc-start-main-.html
extern
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
	char **const envp = ubp_av + argc + 1;
	if (!__mavitix_libc_init(argc, ubp_av, envp, fini, rtld_fini)) {
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
		exit((*main)(argc, ubp_av, envp));
	} else {
		_Exit(EX_SOFTWARE);
	};
}

static
void __libc_init(const __init_t init) {
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

	if __likely (init != NULL) {
		// SAFETY:
		// Problem(s):
		// - Dereferencing a null ptr is Undefined Behaviour.
		// - The `init` parameter is not guaranteed to be non-null.
		// - Dereferencing `init` if it is null would thus be UB.
		// Excuse(s):
		// - We perform a null check before dereference.
		(*init)();
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

extern
void __libc_fini(const int, void *const fini_param) {
	// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=83584
	__gcc_bad__ const __fini_t fini = (__fini_t) fini_param;
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
	if __likely (fini != NULL) {
		// SAFETY:
		// Problem(s):
		// - Dereferencing a null ptr is Undefined Behaviour.
		// - The `_fini` parameter is not guaranteed to be non-null.
		// - Dereferencing `_fini` if it is null would thus be UB.
		// Excuse(s):
		// - We perform a null check before dereference.
		(*fini)();
	};
#else
	(*fini)();
#endif
}
