#pragma once
#include <internal/__defs.h>

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L
#warning Use of <stdnoreturn.h> is deprecated in C23.
#endif

/* Use definition from <internal/__defs.h>. */
#ifdef __NORETURN__
#define noreturn __NORETURN__
#endif /* noreturn */
