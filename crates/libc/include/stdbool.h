#pragma once
#include <internal/__defs.h>

#define __bool_true_false_are_defined 1

/*
 * Technically __STDC_VERSION__ isn't defined in C89, but if the symbol is
 * defined by an extension, it's better to handle it than not.
 */
#if defined(__STDC_VERSION__) && __STDC_VERSION__ < 202311L &&                 \
	__STDC_VERSION__ >= 199901L
#define bool _Bool
#define true 1
#define false 0
#endif
