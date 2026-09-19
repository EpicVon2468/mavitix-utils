#pragma once
#include <internal/__checks.h>

#define __MAVITIX_LIBC__ 1

#define _POSIX_SOURCE 1
#define _POSIX_C_SOURCE 202405L
#define _XOPEN_SOURCE 800

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define __CLANG_LIKE__ 1
#else
#define __CLANG_LIKE__ 0
#endif

#if __CLANG_LIKE__
#define __NOTHROW__ __attribute__((nothrow))
#define __USED__ __attribute__((used))
#define __CONST__ __attribute__((const))
#define __NOINLINE__ __attribute__((noinline))
#define __PURE__ __attribute__((pure))
#else
#define __NOTHROW__
#define __USED__
#define __CONST__
#define __NOINLINE__
#define __PURE__
#endif

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L

#if __CLANG_LIKE__
#define __NORETURN__ [[noreturn]] __attribute__((noreturn))
#else
#define __NORETURN__ [[noreturn]]
#endif

#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L

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
