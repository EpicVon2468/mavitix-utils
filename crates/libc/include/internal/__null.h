#pragma once
#include <internal/__defs.h>

#ifdef __cplusplus
#define NULL 0
#elif __C23__
#define NULL nullptr
#else
#define NULL ((void*) 0)
#endif /* NULL */
