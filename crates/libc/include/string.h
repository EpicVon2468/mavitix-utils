#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#define __STDC_VERSION_STRING_H__ 202311L

#include <internal/__locale_t.h>
#include <internal/__size_t.h>

__NOTHROW__ extern
char *strdup(const char *str);

__NOTHROW__ extern
size_t strlen(const char *str);

#if __INCOMPAT_LIB_REDECL__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wincompatible-library-redeclaration"
#endif

/* "The array pointed to shall not be modified by the program." */
__NOTHROW__ extern
const char *strerror(int errnum);

#if __INCOMPAT_LIB_REDECL__
#pragma clang diagnostic pop
#endif

__EXT_CXX_END__
