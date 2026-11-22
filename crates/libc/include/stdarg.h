#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#define __STDC_VERSION_STDARG_H__ 202311L

/*
 * FIXME:
 * I can't get a straight answer out of anyone as to whether these are 'correct'
 * definitions or not.
 */
#if __has_keyword(__builtin_va_list)
/* GCC doesn't provide `__is_identifier`, so this never passes for GCC. */
typedef __builtin_va_list va_list;
#elif __X86_64__
/* System V x86-64 ABI */
typedef struct {
	unsigned int gp_offset;
	unsigned int fp_offset;
	void *overflow_arg_area;
	void *reg_save_area;
} va_list[1];
#elif __AARCH64__
/*
 * AArch64 Procedure Call Standard
 * https://developer.arm.com/Additional%20Resources/ABI-Procedure%20Call%20Standard%20for%20the%20Arm%2064-bit%20Architecture
 * https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst#1015definition-of-va_list
 */
typedef struct __va_list {
	void *__stack;
	void *__gr_top;
	void *__vr_top;
	int __gr_offs;
	int __vr_offs;
} va_list;
#elif __RISCV64__
typedef struct {
	void *ptr;
} va_list;
#endif /* va_list */

#if __C23__
#if __has_builtin(__builtin_c23_va_start)
#define va_start(...) __builtin_c23_va_start(__VA_ARGS__)
#else
// #define va_start(...)
#endif
#else
#if __has_builtin(__builtin_va_start)
#define va_start(ap, param) __builtin_va_start(ap, param)
#else
// #define va_start(ap, param)
#endif
#endif /* va_start */

#if __has_builtin(__builtin_va_arg)
#define va_arg(ap, type) __builtin_va_arg(ap, type)
#else
// #define va_arg(ap, type)
#endif /* va_arg */

#if __has_builtin(__builtin_va_end)
#define va_end(ap) __builtin_va_end(ap)
#else
/*
 * None of the architectures supported specify to do anything here.
 * Still leave a scope in case someone uses this after an un-bracketed `if`.
 */
#define va_end(ap) {}
#endif /* va_end */

__EXT_CXX_END__
