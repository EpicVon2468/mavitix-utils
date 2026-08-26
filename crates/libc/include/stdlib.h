#pragma once

#define __STDC_VERSION_STDLIB_H__ 202311L
#define __MAVITIX_LIBC__ 1

#define EXIT_FAILURE 1

#define EXIT_SUCCESS 0

#include <internal/__null.h>   /* NULL */
#include <internal/__size_t.h> /* size_t */

#if __STDC_VERSION__ >= 202311L
/* C23 attribute. */
[[noreturn]]
#else
/* C11 keyword; Deprecated in C23. */
_Noreturn
#endif
#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
__attribute__((__noreturn__))
#endif
extern void _Exit(int status);
