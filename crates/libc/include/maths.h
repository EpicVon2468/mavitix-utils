#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_MATH_H__ 202311L

#ifdef __cplusplus
extern "C" {
#endif

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define INFINITY __builtin_inff()
#define NAN __builtin_nanf("")
#define HUGE_VAL __builtin_huge_val()
#define HUGE_VALF __builtin_huge_valf()
#define HUGE_VALL __builtin_huge_vall()
#else
#define NAN (0.0f / 0.0f)
#define INFINITY (1.0f / 0.0f)
#define HUGE_VAL ((double) INFINITY)
#define HUGE_VALF INFINITY
#define HUGE_VALL ((long double) INFINITY)
#endif

#define MATH_ERRNO 1
#define MATH_ERREXCEPT 2

extern double acos(double value);
extern float acosf(float value);
extern long double acosl(long double value);

extern double asin(double value);
extern float asinf(float value);
extern long double asinl(long double value);

extern double atan(double value);
extern float atanf(float value);
extern long double atanl(long double value);

extern double atan2(double y, double x);
extern float atan2f(float y, float x);
extern long double atan2l(long double y, long double x);

extern double cos(double value);
extern float cosf(float value);
extern long double cosl(long double value);

extern double sin(double value);
extern float sinf(float value);
extern long double sinl(long double value);

extern double tan(double value);
extern float tanf(float value);
extern long double tanl(long double value);

#ifdef __cplusplus
}
#endif
