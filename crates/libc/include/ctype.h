#pragma once
#include <internal/__defs.h>

#define EOF -1

/* Classification */

__NOTHROW__ extern
int isalnum(int ch);

__NOTHROW__ extern
int isalpha(int ch);

__NOTHROW__ extern
int isblank(int ch);

__NOTHROW__ extern
int iscntrl(int ch);

__NOTHROW__ extern
int isdigit(int ch);

__NOTHROW__ extern
int isgraph(int ch);

__NOTHROW__ extern
int islower(int ch);

__NOTHROW__ extern
int isprint(int ch);

__NOTHROW__ extern
int ispunct(int ch);

__NOTHROW__ extern
int isspace(int ch);

__NOTHROW__ extern
int isupper(int ch);

__NOTHROW__ extern
int isxdigit(int ch);

/* Case mapping */

__NOTHROW__ extern
int tolower(int ch);

__NOTHROW__ extern
int toupper(int ch);
