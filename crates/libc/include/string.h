#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#define __STDC_VERSION_STRING_H__ 202311L

#include <internal/__locale_t.h>
#include <internal/__size_t.h>

__NOTHROW__ extern
char *strdup(const char *__nonnull__ str);

__NOTHROW__ extern
size_t strlen(const char *__nonnull__ str);

__NOTHROW__ extern
int strcmp(const char *__nonnull__ first, const char *__nonnull__ second);

__NOTHROW__ extern
int strncmp(
	const char *__nonnull__ first,
	const char *__nonnull__ second,
	size_t len
);

#if __INCOMPAT_LIB_REDECL__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wincompatible-library-redeclaration"
#endif

/* "The array pointed to shall not be modified by the program." */
__RETURNS_NONNULL__ __NOTHROW__ extern
const char *strerror(int errnum);

/* NOTE: `locale` is not ignored for these functions. */
__RETURNS_NONNULL__ __NOTHROW__ extern
const char *strerror_l(int errnum, locale_t locale);
__RETURNS_NONNULL__ __NOTHROW__ extern
const char *__strerror_l(int errnum, locale_t locale);

#if __INCOMPAT_LIB_REDECL__
#pragma clang diagnostic pop
#endif

__EXT_CXX_END__
