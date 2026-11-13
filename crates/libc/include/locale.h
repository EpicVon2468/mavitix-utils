#pragma once
#include <internal/__defs.h>

#include <internal/__locale_t.h> /* locale_t */
#include <internal/__null.h>	 /* NULL */

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
const char *setlocale(int category, const char *locale);

/* ABI compat */
struct lconv {
	char *decimal_point;
	char *thousands_sep;
	char *grouping;
	char *int_curr_symbol;
	char *currency_symbol;
	char *mon_decimal_point;
	char *mon_thousands_sep;
	char *mon_grouping;
	char *positive_sign;
	char *negative_sign;
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

__PURE__ __NOTHROW__ extern
struct lconv *localeconv(void);
