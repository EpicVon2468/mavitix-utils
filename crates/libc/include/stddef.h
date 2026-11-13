#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_STDDEF_H__ 202311L

#ifdef __cplusplus
extern "C" {
#endif

#ifdef __PTRDIFF_TYPE__
typedef __PTRDIFF_TYPE__ ptrdiff_t;
#elif __X86_64__ || __AARCH64__ || __RISCV64__
/*
 * `ptrdiff_t` is `signed long int` for x86-64 & aarch64 & riscv64 on Linux.
 * I've checked these definitions against what clang says they are.
 * See:
 * - llvm-project/clang/lib/Basic/Targets/X86.h
 * - llvm-project/clang/lib/Basic/Targets/X86.cpp
 * - llvm-project/clang/lib/Basic/Targets/AArch64.h
 * - llvm-project/clang/lib/Basic/Targets/AArch64.cpp
 * - llvm-project/clang/lib/Basic/Targets/RISCV.h
 * - llvm-project/clang/lib/Basic/Targets/RISCV.cpp
 * (AArch64 & RISC-V don't explicitly set `PtrDiffType`, uses default instead).
 */
typedef signed long int ptrdiff_t;
#endif /* ptrdiff_t */

#include <internal/__size_t.h> /* size_t */

#if __C11__
typedef long double max_align_t;
#endif /* max_align_t */

#if __C23__
typedef typeof_unqual(nullptr) nullptr_t;
#endif /* nullptr_t */

#include <internal/__null.h> /* NULL */

/*
 * Mavitix libc extension: `unreachable()` (or at least a stub) is provided for
 * versions as far back as possible.
 */
#if __has_builtin(__builtin_unreachable)
#define unreachable() __builtin_unreachable()
#elif __has_builtin(__builtin_trap)
#define unreachable() __builtin_trap()
#elif __has_builtin(__builtin_assume)
#define unreachable() __builtin_assume(0)
#else
__NORETURN__ extern
void __mavitix_immediate_abort(int);
/* Abort with `EX_SOFTWARE` */
#define unreachable() __mavitix_immediate_abort(70)
#endif /* unreachable */

#if __has_builtin(__builtin_offsetof)
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
#endif /* offsetof */

#ifdef __cplusplus
}
#endif
