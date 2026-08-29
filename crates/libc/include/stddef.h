#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_STDDEF_H__ 202311L

#ifdef __PTRDIFF_TYPE__
typedef __PTRDIFF_TYPE__ ptrdiff_t;
#elif defined(__x86_64__) || defined(__amd64__) || defined(__aarch64__)
/*
 * `ptrdiff_t` is `signed long int` for x86-64 & aarch64 on Linux.
 * I've checked these definitions against what clang says they are.
 * See:
 * - llvm-project/clang/lib/Basic/Targets/X86.h
 * - llvm-project/clang/lib/Basic/Targets/X86.cpp
 * - llvm-project/clang/lib/Basic/Targets/AArch64.h
 * - llvm-project/clang/lib/Basic/Targets/AArch64.cpp
 * (Note that AArch64 doesn't set `PtrDiffType`, defaulting to `SignedLong`).
 */
typedef signed long int ptrdiff_t;
#endif /* ptrdiff_t */

#include <internal/__size_t.h> /* size_t */

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
typedef long double max_align_t;
#endif /* max_align_t */

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L
typedef typeof_unqual(nullptr) nullptr_t;
#endif /* nullptr_t */

#include <internal/__null.h> /* NULL */

/*
 * Mavitix libc extension: `unreachable()` (or at least a stub) is provided for
 * versions as far back as C11.
 */
#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
/*
 * Clang & GCC both provide `__builtin_unreachable()`, no matter what C
 * Standard is specified.
 */
#define unreachable() __builtin_unreachable()
#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L

#warning Using mavitix-libc function as a fallback for `unreachable()`!
#warning This build may be non-portable against other libc implementations!

/* We're >= C11 here, so `inline` is a valid keyword (available since C99). */
__NORETURN__ extern inline void __mavitix_unreachable_impl(void);
#define unreachable() __mavitix_unreachable_impl()

#endif /* unreachable(void) */

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define offsetof(type, member_designator) __builtin_offsetof(type, member_designator)
#else
/*
 * Stub implementation; Pretends `0` is an instance of `type`, then takes the
 * address of `member_designator` as an offset from 0.
 * This never involves dereferencing a value, as all operations are performed on
 * the offsetted addresses.
 * See:
 * https://stackoverflow.com/questions/713963/why-does-this-implementation-of-offsetof-work
 */
#define offsetof(type, member_designator) ((size_t) ( (char *)&((type *)(0))->member_designator - (char *)0 ))
#endif /* offsetof(type, member_designator) */
