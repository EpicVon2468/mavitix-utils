#pragma once
#include <internal/__defs.h>

#if __C23__
#warning Use of <stdnoreturn.h> is deprecated in C23.
#endif

/* Use definition from <internal/__defs.h>. */
#ifdef __NORETURN__
#define noreturn __NORETURN__
#endif /* noreturn */
