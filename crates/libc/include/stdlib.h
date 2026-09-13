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

extern void *aligned_alloc(size_t align, size_t size);
extern void *calloc(size_t num, size_t size);
extern void free(void *ptr);
extern void free_sized(void *ptr, size_t size);
extern void free_aligned_sized(void *ptr, size_t align, size_t size);
extern void *malloc(size_t size);
extern void *realloc(void *ptr, size_t size);

__NORETURN__ extern void _Exit(int status);

#ifdef __cplusplus
}
#endif
