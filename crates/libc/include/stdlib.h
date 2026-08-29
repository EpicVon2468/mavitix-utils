#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_STDLIB_H__ 202311L

#define EXIT_FAILURE 1

#define EXIT_SUCCESS 0

#include <internal/__null.h>   /* NULL */
#include <internal/__size_t.h> /* size_t */

__NORETURN__ extern void _Exit(int status);
