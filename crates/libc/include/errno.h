#pragma once
#include <internal/__defs.h>

#ifdef __cplusplus
extern "C" {
#endif

__CONST__ __NOTHROW__ extern
int *__errno_location(void);

#define errno (*__errno_location())

#ifdef __cplusplus
}
#endif
