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
int fclose(FILE *stream);

__NOTHROW__ extern
int fflush(FILE *stream);

__NOTHROW__ extern
FILE *fopen(const char *__restrict__ filename, const char *__restrict__ mode);

__NOTHROW__ extern
FILE *fdopen(int fd, const char *mode);

__NOTHROW__ extern
int fileno(FILE *stream);

__UNAVAILABLE__("Use of `gets` is strictly prohibited")
__DEPRECATED__("Use of `gets` is highly discouraged") __CONST__ __INLINE__
__NOTHROW__ extern
char *gets(char *const) {
	return __nullptr__;
}

__EXT_CXX_END__
