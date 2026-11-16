#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#if !defined(alloca)

#if __has_builtin(__builtin_alloca)
#define alloca(size) __builtin_alloca(size)
#else
#include <internal/__size_t.h> /* size_t */
__NOTHROW__ extern
void *alloca(size_t size);
#endif

#endif /* alloca */

__EXT_CXX_END__
