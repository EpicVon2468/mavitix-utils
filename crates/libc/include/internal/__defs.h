#pragma once
#include <internal/__checks.h>

#define __MAVITIX_LIBC__ 1

#define _POSIX_SOURCE 1
#define _POSIX_C_SOURCE 202405L
#define _XOPEN_SOURCE 800

#if !defined(__has_c_attribute)
#define __has__has_c_attribute(x) 0
#endif

#if !defined(__has_builtin)
#define __has_builtin(x) 0
#endif

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define __CLANG_LIKE__ 1
#else
#define __CLANG_LIKE__ 0
#endif

#define __C89__ 1
#define __C90__ 1
#define __C95__ 0
#define __C99__ 0
#define __C11__ 0
#define __C17__ 0
#define __C23__ 0

#ifdef __STDC_VERSION__

/*
 * __STDC_VERSION__ was first defined in N325 (C95).
 * Value table:
 * C89: Undefined
 * C90: Undefined
 * C95: 199409L
 * C99: 199901L
 * C11: 201112L
 * C17: 201710L
 * C23: 202311L
 */

#if __STDC_VERSION__ >= 199409L
#undef __C95__
#define __C95__ 1
#endif /* C95 */

#if __STDC_VERSION__ >= 199901L
#undef __C99__
#define __C99__ 1
#endif /* C99 */

#if __STDC_VERSION__ >= 201112L
#undef __C11__
#define __C11__ 1
#endif /* C11 */

#if __STDC_VERSION__ >= 201710L
#undef __C17__
#define __C17__ 1
#endif /* C17 */

#if __STDC_VERSION__ >= 202311L
#undef __C23__
#define __C23__ 1
#endif /* C23 */

#endif /* __STDC_VERSION__ */

#if __CLANG_LIKE__
#define __NOTHROW__ __attribute__((nothrow))
#define __USED__ __attribute__((used))
#define __CONST__ __attribute__((const))
#define __NOINLINE__ __attribute__((noinline))
#define __PURE__ __attribute__((pure))
#define __PROTECTED__ __attribute__((visibility("protected")))
#define __HIDDEN__ __attribute__((visibility("hidden")))
#define __WEAK__ __attribute__((weak))
#else
#define __NOTHROW__
#define __USED__
#define __CONST__
#define __NOINLINE__
#define __PURE__
#define __PROTECTED__
#define __HIDDEN__
#define __WEAK__
#endif

#if __C23__ && __has_c_attribute(noreturn)

#if __CLANG_LIKE__
#define __NORETURN__ [[noreturn]] __attribute__((noreturn))
#else
#define __NORETURN__ [[noreturn]]
#endif

#elif __C11__

#if __CLANG_LIKE__
#define __NORETURN__ _Noreturn __attribute__((noreturn))
#else
#define __NORETURN__ _Noreturn
#endif

#elif __CLANG_LIKE__

#define __NORETURN__ __attribute__((noreturn))

#else

#define __NORETURN__

#endif /* __NORETURN__ */

#ifdef __MAVITIX_INTERNAL__

#define __LINKER_SYMBOL__(x) \
	__HIDDEN__ __WEAK__ extern \
	void (*const x[])(void)

typedef void (*__init_t)(void);
typedef void (*__fini_t)(void);
typedef int (*__main_t)(int argc, char **argv, char **envp);

#if __C23__

#if __has_c_attribute(clang::likely)
#define _ClangLikely(x) (x) [[clang::likely]]
#else
#define _ClangLikely(x) (x)
#endif

#if __has_c_attribute(clang::unlikely)
#define _ClangUnlikely(x) (x) [[clang::unlikely]]
#else
#define _ClangUnlikely(x) (x)
#endif

#else
#define _ClangLikely(x) (x)
#define _ClangUnlikely(x) (x)
#endif /* _ClangLikely & _ClangUnlikely */

#if __has_builtin(__builtin_expect)
#define __likely(x) _ClangLikely(__builtin_expect((x), 1))
#define __unlikely(x) _ClangUnlikely(__builtin_expect((x), 0))
#else
#define __likely(x) _ClangLikely(x)
#define __unlikely(x) _ClangUnlikely(x)
#endif /* __likely & __unlikely */

#endif
