#pragma once

#define __MAVITIX_LIBC__ 1

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L
#warning Use of <stdnoreturn.h> is deprecated in C23.
#define noreturn [[noreturn]]
#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
#define noreturn _Noreturn
#endif /* noreturn */
