#pragma once
#include <internal/__defs.h>

typedef struct __libc_auxv {
	const char *exec_fn;
	unsigned long page_size;
	bool secure;
} __libc_auxv;
