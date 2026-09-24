#define __MAVITIX_INTERNAL__
#include <internal/__defs.h>
#include <internal/auxv.h>
#include <internal/rpmalloc.h>

#include <elf.h>
#include <stdbool.h>
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
__PROTECTED__ static
bool __mavitix_libc_init(
	int argc,
	char **argv,
	char **envp,
	__fini_t fini,
	__fini_t rtld_fini
);
__PROTECTED__ static
void __libc_init(__init_t _init);
__PROTECTED__ static
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
bool __mavitix_libc__stdio_init(void);
__NOTHROW__ extern
bool __mavitix_libc__stdio_fini(void);

extern char **environ;

__PROTECTED__ static
bool __mavitix_libc_init(
	const int argc,
	char **const argv,
	char **const envp,
	const __fini_t _fini,
	const __fini_t rtld_fini
) {
	environ = envp;
	size_t size = 0;
	while (envp[size] != NULL) {
		size += 1;
	};
	const auxv_t *const raw_auxv = (auxv_t *) (envp + size + 1);
	__libc_auxv auxv = {
		.exec_fn = NULL,
		.page_size = 0,
		.secure = 0,
	};
	size = 0;
	while (true) {
		const auxv_t entry = raw_auxv[size];
		switch (entry.a_type) {
			case AT_EXECFN:
				auxv.exec_fn = entry.a_un.a_ptr;
				break;
			case AT_PAGESZ:
				auxv.page_size = entry.a_un.a_val;
				break;
			case AT_SECURE:
				auxv.secure = entry.a_un.a_val;
				break;
			case AT_NULL:
				goto escape;
			default:
				break;
		};
		size += 1;
	};
escape:
	rpmalloc_config_t config = {
		.page_size = auxv.page_size,
		.page_name = "mavitix-libc-rpmalloc-page",
		.huge_page_name = "mavitix-libc-rpmalloc-huge-page",
		.unmap_on_finalize = 0,
	};
	rpmalloc_initialize_config(NULL, &config);
	if (!__mavitix_libc__stdio_init()) {
		return false;
	};
	const char *program_name;
	if __likely (argc > 0) {
		// SANITY(unusual):
		// From C23 (N3220) 5.1.2.3.2.2:
		// """
		// If the value of argc is greater than zero, the string pointed to by
		// argv[0] represents the program name; argv[0][0] shall be the null
		// character if the program name is not available from the host
		// environment.  If the value of argc is greater than one, the strings
		// pointed to by argv[1] through argv[argc-1] represent the program
		// parameters.
		// """
		const char *const ptr = argv[0];
		if __likely (ptr[0] != 0) {
			program_name = strdup(ptr);
		} else {
			program_name = auxv.exec_fn;
		};
	} else {
		program_name = auxv.exec_fn;
	};

	if (program_name == NULL) {
		// Failed to allocate enough memory.
		return false;
	};

	if __likely (rtld_fini != NULL) {
		// "a function pointer that the application should register with atexit"
		atexit(rtld_fini);
	};
	if __likely (_fini != NULL) {
		on_exit(__libc_fini, (void *) _fini);
	};

	return true;
}

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

__PROTECTED__ static
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
