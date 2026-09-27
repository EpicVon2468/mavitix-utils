#define __MAVITIX_INTERNAL__
#include <internal/__defs.h>

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

FILE *stdin = NULL;
FILE *stdout = NULL;
FILE *stderr = NULL;

int __mavitix_libc__stdio_init(void) {
	// FIXME:
	// This isn't needed or normal, it's a temporary solution because I haven't
	// implemented the `FILE` type yet.
	// In reality, the three streams can just be initialised to the FILE type
	// without any opening whatsoever.
	if ((stdin = fdopen(STDIN_FILENO, "r")) == NULL) {
		return false;
	};
	if ((stdout = fdopen(STDOUT_FILENO, "a")) == NULL) {
		return false;
	};
	if ((stderr = fdopen(STDERR_FILENO, "a")) == NULL) {
		return false;
	};
	return true;
}

int __mavitix_libc__stdio_fini(void) {
	// SANITY(unusual):
	// `fclose` automagically flushes the stream(s).
	if (fclose(stdin) == EOF) {
		return false;
	};
	if (fclose(stdout) == EOF) {
		return false;
	};
	if (fclose(stderr) == EOF) {
		return false;
	};
	return true;
}
