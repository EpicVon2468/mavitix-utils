#pragma once
#define __MAVITIX_LIBC__ 1

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define __THROW__ __attribute__((nothrow))
#define __USED__ __attribute__((used))
#endif

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L /* C23 */

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define __NORETURN__ [[noreturn]] __attribute__((noreturn))
#else
#define __NORETURN__ [[noreturn]]
#endif

#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L /* C11 */

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define __NORETURN__ _Noreturn __attribute__((noreturn))
#else
#define __NORETURN__ _Noreturn
#endif

#elif defined(__clang__) || defined(__llvm__) || defined(__GNUC__)

#define __NORETURN__ __attribute__((noreturn))

#endif /* __NORETURN__ */
