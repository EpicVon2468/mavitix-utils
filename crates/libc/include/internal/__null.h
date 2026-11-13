#pragma once
#include <internal/__defs.h>

#ifdef __CXX98__
#define NULL 0
#elif __C23__ || __CXX11__
#define NULL nullptr
#else
#define NULL ((void *) 0)
#endif /* NULL */
