#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

#define __STDC_VERSION_STDIO_H__ 202311L

#define STDIN_FILENO 0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

typedef void FILE;

#define EOF -1

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

__NOTHROW__ extern
int fileno(FILE *__nonnull__ stream);

__NOTHROW__ extern
int printf(const char *__restrict__ __nonnull__ fmt, ...);

__NOTHROW__ extern
int fprintf(
	FILE *__restrict__ __nonnull__ stream,
	const char *__restrict__ __nonnull__ fmt,
	...
);

__NOTHROW__ extern
int dprintf(int fd, const char *__restrict__ __nonnull__ fmt, ...);

__NOTHROW__ extern
int fclose(FILE *__nonnull__ stream);

__NOTHROW__ extern
int fflush(FILE *__nonnull__ stream);

__NOTHROW__ extern
FILE *fopen(
	const char *__restrict__ __nonnull__ filename,
	const char *__restrict__ __nonnull__ mode
);

__NOTHROW__ extern
FILE *fdopen(int fd, const char *__nonnull__ mode);

__UNAVAILABLE__("Use of `gets` is strictly prohibited")
__DEPRECATED__("Use of `gets` is highly discouraged") __CONST__ __INLINE__
__NOTHROW__ extern
char *gets(char *const) {
	return __nullptr__;
}

__EXT_CXX_END__
