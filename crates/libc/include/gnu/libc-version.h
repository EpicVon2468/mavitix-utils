#pragma once
#include <internal/__defs.h>

/*
 * Fun fact!  Linux Standard Base Core says you need these extensions to be
 * conformant!  I fucking hate this!!!
 *
 * From Linux Standard Base 5.0 Core (Generic); Chapter 14.3.22: "GNU Extensions
 * for libc":
 *
 * > An LSB conforming implementation shall provide the generic functions for
 * > GNU Extensions for libc specified in Table 14-37, with the full mandatory
 * > functionality as described in the referenced underlying specification.
 *
 * From the function specifications in Linux Standard Base 5.0 Core (Generic):
 *
 * > These functions are specific to GNU libc (glibc).  This specification does
 * > not require the implementation of libc to be glibc, although it requires
 * > these functions.
 */

__THROW__ extern const char *gnu_get_libc_release(void);
__THROW__ extern const char *gnu_get_libc_version(void);
