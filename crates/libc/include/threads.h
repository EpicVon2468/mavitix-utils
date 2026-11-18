#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#if (__C11__ || __has_extension(c_thread_local)) && !__C23__
#define thread_local _Thread_local
#endif /* thread_local */

__EXT_CXX_END__
