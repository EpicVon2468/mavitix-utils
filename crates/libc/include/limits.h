#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_LIMITS_H__ 202311L

#ifdef __BOOL_WIDTH__
#define BOOL_WIDTH __BOOL_WIDTH__
#else
#define BOOL_WIDTH 1
#endif /* BOOL_WIDTH */

#ifdef __CHAR_BIT__
#define CHAR_BIT __CHAR_BIT__
#else
#define CHAR_BIT 8
#endif /* CHAR_BIT */
#define CHAR_WIDTH CHAR_BIT
#define SCHAR_WIDTH CHAR_BIT
#define UCHAR_WIDTH CHAR_BIT

#ifdef __SHRT_WIDTH__
#define SHRT_WIDTH __SHRT_WIDTH__
#else
#define SHRT_WIDTH 16
#endif /* SHRT_WIDTH */
#define USHRT_WIDTH SHRT_WIDTH

#ifdef __INT_WIDTH__
#define INT_WIDTH __INT_WIDTH__
#else
#define INT_WIDTH 32
#endif /* INT_WIDTH */
#define UINT_WIDTH INT_WIDTH

#ifdef __LONG_WIDTH__
#define LONG_WIDTH __LONG_WIDTH__
#else
#define LONG_WIDTH 64
#endif /* LONG_WIDTH */
#define ULONG_WIDTH LONG_WIDTH

#ifdef __LLONG_WIDTH__
#define LLONG_WIDTH __LLONG_WIDTH__
#else
#define LLONG_WIDTH 64
#endif /* LLONG_WIDTH */
#define ULLONG_WIDTH LLONG_WIDTH
