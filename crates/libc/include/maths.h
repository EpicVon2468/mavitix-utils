#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_MATH_H__ 202311L
#define __STDC_VERSION_MATHS_H__ 202311L

#ifdef __cplusplus
extern "C" {
#endif

#if __has_builtin(__builtin_inff)
#define INFINITY __builtin_inff()
#else
#define INFINITY (1.0f / 0.0f)
#endif /* INFINITY */

#if __has_builtin(__builtin_nanf)
#define NAN __builtin_nanf("")
#else
#define NAN (0.0f / 0.0f)
#endif /* NAN */

#if __has_builtin(__builtin_huge_val)
#define HUGE_VAL __builtin_huge_val()
#else
#define HUGE_VAL (1.0 / 0.0)
#endif /* HUGE_VAL */

#if __has_builtin(__builtin_huge_valf)
#define HUGE_VALF __builtin_huge_valf()
#else
#define HUGE_VALF INFINITY
#endif /* HUGE_VALF */

#if __has_builtin(__builtin_huge_vall)
#define HUGE_VALL __builtin_huge_vall()
#else
#define HUGE_VALL (1.0L / 0.0L)
#endif /* HUGE_VALL */

#define MATH_ERRNO 1
#define MATH_ERREXCEPT 2

__NOTHROW__ extern
double acos(double value);
__NOTHROW__ extern
float acosf(float value);
__NOTHROW__ extern
long double acosl(long double value);

__NOTHROW__ extern
double asin(double value);
__NOTHROW__ extern
float asinf(float value);
__NOTHROW__ extern
long double asinl(long double value);

__NOTHROW__ extern
double atan(double value);
__NOTHROW__ extern
float atanf(float value);
__NOTHROW__ extern
long double atanl(long double value);

__NOTHROW__ extern
double atan2(double y, double x);
__NOTHROW__ extern
float atan2f(float y, float x);
__NOTHROW__ extern
long double atan2l(long double y, long double x);

__NOTHROW__ extern
double cos(double value);
__NOTHROW__ extern
float cosf(float value);
__NOTHROW__ extern
long double cosl(long double value);

__NOTHROW__ extern
double sin(double value);
__NOTHROW__ extern
float sinf(float value);
__NOTHROW__ extern
long double sinl(long double value);

__NOTHROW__ extern
double tan(double value);
__NOTHROW__ extern
float tanf(float value);
__NOTHROW__ extern
long double tanl(long double value);

#ifdef __cplusplus
}
#endif
