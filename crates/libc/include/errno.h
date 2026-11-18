#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#include <linux/errno.h>

__RETURNS_NONNULL__ __CONST__ __NOTHROW__ extern
int *__errno_location(void);

#define errno (*__errno_location())

__EXT_CXX_END__
