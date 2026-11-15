#pragma once
#include <internal/__defs.h>

/* #include <linux/errno.h> */

__CONST__ __NOTHROW__ extern
int *__errno_location(void);

#define errno (*__errno_location())
