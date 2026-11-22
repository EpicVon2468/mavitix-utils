#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#include <internal/__locale_t.h> /* locale_t */
#define NULL __nullptr__

#define LC_CTYPE 0
#define LC_NUMERIC 1
#define LC_TIME 2
#define LC_COLLATE 3
#define LC_MONETARY 4
#define LC_MESSAGES 5
#define LC_ALL 6
#define LC_PAPER 7
#define LC_NAME 8
#define LC_ADDRESS 9
#define LC_TELEPHONE 10
#define LC_MEASUREMENT 11
#define LC_IDENTIFICATION 12

#define LC_GLOBAL_LOCALE ((locale_t) -1L)

/*
 * NOTE:
 * This function's prototype is different under Mavitix libc.
 * There is no reason for `setlocale` to return a mutable pointer,
 * and it would only cause either memory leaks or threading issues,
 * as well as increasing code complexity for implementation.
 * For the above reasons, `setlocale`'s function prototype has been
 * modified to return `const char *` instead of `char *`.
 */
__NOTHROW__ extern
const char *setlocale(int category, const char *__nullable__ locale);

/* ABI compat */
struct lconv {
	const char *decimal_point;
	const char *thousands_sep;
	const char *grouping;
	const char *int_curr_symbol;
	const char *currency_symbol;
	const char *mon_decimal_point;
	const char *mon_thousands_sep;
	const char *mon_grouping;
	const char *positive_sign;
	const char *negative_sign;
	char int_frac_digits;
	char frac_digits;
	char p_cs_precedes;
	char p_sep_by_space;
	char n_cs_precedes;
	char n_sep_by_space;
	char p_sign_posn;
	char n_sign_posn;
	char int_p_cs_precedes;
	char int_p_sep_by_space;
	char int_n_cs_precedes;
	char int_n_sep_by_space;
	char int_p_sign_posn;
	char int_n_sign_posn;
};

__RETURNS_NONNULL__ __NOTHROW__ extern
const struct lconv *localeconv(void);

__NOTHROW__ extern
locale_t newlocale(int category_mask, const char *locale, locale_t base);

__NOTHROW__ extern
locale_t duplocale(locale_t locobj);

__NOTHROW__ extern
locale_t uselocale(locale_t newloc);

/*
 * TODO: May have to refactor `locale_t` given what this specification says.
 * https://pubs.opengroup.org/onlinepubs/9799919799/functions/getlocalename_l.html
 */
__NOTHROW__ extern
const char *getlocalename_l(int category, locale_t locobj);

__EXT_CXX_END__
