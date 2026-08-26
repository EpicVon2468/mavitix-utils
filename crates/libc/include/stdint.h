#pragma once

#define __STDC_VERSION_STDINT_H__ 202311L
#define __MAVITIX_LIBC__ 1

#ifdef __INT8_TYPE__
typedef __INT8_TYPE__ int8_t;
#else
typedef signed char int8_t;
#endif /* int8_t */
#define INT8_WIDTH 8

#ifdef __INT16_TYPE__
typedef __INT16_TYPE__ int16_t;
#else
typedef signed short int int16_t;
#endif /* int16_t */
#define INT16_WIDTH 16

#ifdef __INT32_TYPE__
typedef __INT32_TYPE__ int32_t;
#else
typedef signed int int32_t;
#endif /* int32_t */
#define INT32_WIDTH 32

#ifdef __INT64_TYPE__
typedef __INT64_TYPE__ int64_t;
#else
typedef signed long int int64_t;
#endif /* int64_t */
#define INT64_WIDTH 64

#define INTPTR_WIDTH UINTPTR_WIDTH
#define UINTPTR_WIDTH 16

#define INTMAX_WIDTH UINTMAX_WIDTH
#define UINTMAX_WIDTH 64

#define PTRDIFF_WIDTH 16
#define SIZE_WIDTH 16
#define WCHAR_WIDTH 8
#define WINT_WIDTH 16
