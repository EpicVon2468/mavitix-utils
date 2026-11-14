#pragma once
#include <internal/__defs.h>

#if !__C23__ && __C11__
#define thread_local _Thread_local
#endif /* thread_local */
