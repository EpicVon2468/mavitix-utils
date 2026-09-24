#define __MAVITIX_INTERNAL__
#include <internal/__defs.h>

#include <gnu/libc-version.h>

__NOINLINE__ __PURE__ __NOTHROW__ __USED__
const char *gnu_get_libc_release(void) {
	return "stable";
}

__NOINLINE__ __PURE__ __NOTHROW__ __USED__
const char *gnu_get_libc_version(void) {
	return "2.44.0";
}
