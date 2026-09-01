#pragma once
#include <internal/__defs.h>

/*
 * This link is very annoying to find, so hold my beer:
 * https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/unistd.h.html
 */

#define _POSIX_VERSION 202405L
#define _POSIX2_VERSION 202405L
#define _XOPEN_VERSION 800

/*
 * My network does not support IPv6; I cannot perform any tests for
 * potential function implementations.
 */
#define _POSIX_IPV6 -1
#define _POSIX_NO_TRUNC 1
#define _POSIX_REGEXP 1
#define _POSIX_SHELL 1
#define _POSIX_SPAWN _POSIX_VERSION
#define _POSIX_SPIN_LOCKS _POSIX_VERSION
#define _POSIX_THREADS _POSIX_VERSION
#define _POSIX_TIMEOUTS _POSIX_VERSION
#define _POSIX_TIMERS _POSIX_VERSION
#define _POSIX2_C_BIND _POSIX2_VERSION
#define _POSIX2_C_DEV _POSIX2_VERSION
#define _XOPEN_SHM 1
#define _XOPEN_UNIX 1

#include <internal/__null.h> /* NULL */

#define _PC_PATH_MAX 4

#define STDIN_FILENO 0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

#include <internal/__size_t.h> /* size_t */

__NORETURN__ extern void _exit(int status);
extern long fpathconf(int desc, int name);
extern char *getcwd(char *buf, size_t size);
extern int isatty(int fd);
extern int link(const char *oldpath, const char *newpath);
extern int linkat(
	int oldfd, const char *oldpath, int newfd, const char *newpath, int flags
);
extern long pathconf(const char *path, int name);
extern int symlink(const char *target, const char *linkpath);
extern int symlinkat(const char *target, int newdirfd, const char *linkpath);
extern char *ttyname(int fd);
extern int unlink(const char *path);
extern int unlinkat(int fd, const char *path, int flags);

extern char *optarg;
extern int opterr, optind, optopt;
