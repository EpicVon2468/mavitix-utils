#pragma once
#include <internal/__defs.h>

#define __STDC_VERSION_STDLIB_H__ 202311L

#ifdef __cplusplus
extern "C" {
#endif

#define EXIT_FAILURE 1

#define EXIT_SUCCESS 0

#include <internal/__null.h>   /* NULL */
#include <internal/__size_t.h> /* size_t */

__NOTHROW__ extern
void *aligned_alloc(size_t align, size_t size);
__NOTHROW__ extern
void *calloc(size_t num, size_t size);
__NOTHROW__ extern
void free(void *ptr);
__NOTHROW__ extern
void free_sized(void *ptr, size_t size);
__NOTHROW__ extern
void free_aligned_sized(void *ptr, size_t align, size_t size);
__NOTHROW__ extern
void *malloc(size_t size);
__NOTHROW__ extern
void *realloc(void *ptr, size_t size);

__NORETURN__ extern
void abort(void);
__NORETURN__ extern
void exit(int status);
__NORETURN__ extern
void _exit(int status);
__NORETURN__ extern
void _Exit(int status);

__NOTHROW__ extern
int on_exit(void (*fn)(int status, void *usr_ptr), void *usr_ptr);
__NOTHROW__ extern
int atexit(void (*fn)(void));

#ifdef __cplusplus
}
#endif
