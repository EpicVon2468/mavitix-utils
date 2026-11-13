#pragma once
#include <internal/__defs.h>
#include <internal/__locale_t.h>

#ifdef __cplusplus
extern "C" {
#endif

#define EOF -1

/* Classification */

__NOTHROW__ extern
int isalnum(int ch);
__NOTHROW__ extern
int isalnum_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isalnum_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isalpha(int ch);
__NOTHROW__ extern
int isalpha_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isalpha_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isblank(int ch);
__NOTHROW__ extern
int isblank_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isblank_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int iscntrl(int ch);
__NOTHROW__ extern
int iscntrl_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __iscntrl_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isdigit(int ch);
__NOTHROW__ extern
int isdigit_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isdigit_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isgraph(int ch);
__NOTHROW__ extern
int isgraph_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isgraph_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int islower(int ch);
__NOTHROW__ extern
int islower_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __islower_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isprint(int ch);
__NOTHROW__ extern
int isprint_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isprint_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int ispunct(int ch);
__NOTHROW__ extern
int ispunct_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __ispunct_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isspace(int ch);
__NOTHROW__ extern
int isspace_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isspace_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isupper(int ch);
__NOTHROW__ extern
int isupper_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isupper_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isxdigit(int ch);
__NOTHROW__ extern
int isxdigit_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isxdigit_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int isascii(int ch);
__NOTHROW__ extern
int isascii_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __isascii_l(int ch, locale_t __ignored);

/* Case mapping */

__NOTHROW__ extern
int tolower(int ch);
__NOTHROW__ extern
int tolower_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __tolower_l(int ch, locale_t __ignored);

__NOTHROW__ extern
int toupper(int ch);
__NOTHROW__ extern
int tolower_l(int ch, locale_t __ignored);
__NOTHROW__ extern
int __tolower_l(int ch, locale_t __ignored);

__UNAVAILABLE__("Use of `toascii` is strictly prohibited")
__DEPRECATED__("Use of `toascii` is highly discouraged") __CONST__ __INLINE__
__NOTHROW__ extern
int toascii(int ch) {
	return ch & 0x7F;
}

#ifdef __cplusplus
}
#endif
