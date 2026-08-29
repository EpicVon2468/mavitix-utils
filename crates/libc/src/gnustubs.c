#include <gnu/libc-version.h>
#include <internal/__defs.h>

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
__attribute__((noinline, pure))
#endif
__THROW__ __USED__ const char *gnu_get_libc_release(void) {
	return "stable";
}

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
__attribute__((noinline, pure))
#endif
__THROW__ __USED__ const char *gnu_get_libc_version(void) {
	return "2.44.0";
}
