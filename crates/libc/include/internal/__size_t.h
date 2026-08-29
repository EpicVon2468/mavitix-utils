#pragma once

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L
typedef typeof_unqual(sizeof(int)) size_t;
#elif defined(__SIZE_TYPE__)
typedef __SIZE_TYPE__ size_t;
#elif defined(__x86_64__) || defined(__amd64__) || defined(__aarch64__)
/* `size_t` is `unsigned long int` for x86-64 & aarch64 on Linux. */
typedef unsigned long int size_t;
#endif /* size_t */
