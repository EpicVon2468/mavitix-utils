#pragma once
#include <internal/__defs.h>

#if defined(__SIZE_TYPE__)
typedef __SIZE_TYPE__ size_t;
#elif __C23__
typedef typeof_unqual(sizeof(int)) size_t;
#elif __X86_64__ || __AARCH64__ || __RISCV64__
/* `size_t` is `unsigned long int` for x86-64 & aarch64 & riscv64 on Linux. */
typedef unsigned long int size_t;
#endif /* size_t */
