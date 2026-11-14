#define __MAVITIX_INTERNAL__
#include <internal/__defs.h>

#include <errno.h>
#include <threads.h>

// TODO: https://github.com/rust-lang/rust/issues/29594
thread_local int __mavitix_errno = 0;

__CONST__ __NOTHROW__ extern
int *__errno_location(void) {
	return &__mavitix_errno;
}
