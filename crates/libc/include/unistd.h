#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

/*
 * This link is very annoying to find, so hold my beer:
 * https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/unistd.h.html
 */

#define _POSIX_VERSION 202405L
#define _POSIX2_VERSION 202405L
#define _XOPEN_VERSION 800

#define _POSIX_CHOWN_RESTRICTED 0
/*
 * My network does not support IPv6; I cannot perform any tests for
 * potential function implementations.
 */
#define _POSIX_IPV6 -1
#define _POSIX_NO_TRUNC _POSIX_VERSION
#define _POSIX_REGEXP _POSIX_VERSION
#define _POSIX_SAVED_IDS _POSIX_VERSION
#define _POSIX_SHELL _POSIX_VERSION
#define _POSIX_SPAWN _POSIX_VERSION
#define _POSIX_SPIN_LOCKS _POSIX_VERSION
#define _POSIX_THREAD_SAFE_FUNCTIONS _POSIX_VERSION
#define _POSIX_THREADS _POSIX_VERSION
#define _POSIX_TIMEOUTS _POSIX_VERSION
#define _POSIX_TIMERS _POSIX_VERSION
/* Begin LP64 */
#define _XBS5_LP64_OFF64
#define _POSIX_V6_LP64_OFF64
#define _POSIX_V7_LP64_OFF64
#define _POSIX_V8_LP64_OFF64
/* End LP64 */
#define _POSIX2_C_BIND _POSIX2_VERSION
#define _POSIX2_C_DEV _POSIX2_VERSION
#define _POSIX2_CHAR_TERM _POSIX2_VERSION
/*
 * Unless this becomes a hard requirement by specification,
 * it isn't worth the effort to support.
 */
#define _POSIX2_LOCALEDEF -1
#define _POSIX2_SYMLINKS 1
#define _XOPEN_SHM 1
#define _XOPEN_UNIX 1

#define NULL __nullptr__

#define _PC_LINK_MAX 0
#define _PC_MAX_CANON 1
#define _PC_MAX_INPUT 2
#define _PC_NAME_MAX 3
#define _PC_PATH_MAX 4
#define _PC_PIPE_BUF 5
#define _PC_CHOWN_RESTRICTED 6
#define _PC_NO_TRUNC 7
#define _PC_VDISABLE 8
#define _PC_SYNC_IO 9
#define _PC_ASYNC_IO 10
#define _PC_PRIO_IO 11
#define _PC_SOCK_MAXBUF 12
#define _PC_FILESIZEBITS 13
#define _PC_REC_INCR_XFER_SIZE 14
#define _PC_REC_MAX_XFER_SIZE 15
#define _PC_REC_MIN_XFER_SIZE 16
#define _PC_REC_XFER_ALIGN 17
#define _PC_ALLOC_SIZE_MIN 18
#define _PC_SYMLINK_MAX 19
#define _PC_2_SYMLINKS 20

#define STDIN_FILENO 0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

typedef unsigned int uid_t;
typedef unsigned int gid_t;
#include <internal/__size_t.h>
#include <internal/__ssize_t.h>

__NOTHROW__ extern
ssize_t write(int fd, const void *buf, size_t len);

__NORETURN__ extern
void _exit(int status);

__NOTHROW__ extern
char *getcwd(char *buf, size_t size);

__NOTHROW__ extern
int isatty(int fd);

__NOTHROW__ extern
char *ttyname(int fd);

__NOTHROW__ extern
int link(const char *oldpath, const char *newpath);
__NOTHROW__ extern
int linkat(
	int oldfd,
	const char *oldpath,
	int newfd,
	const char *newpath,
	int flags
);

__NOTHROW__ extern
int symlink(const char *target, const char *linkpath);
__NOTHROW__ extern
int symlinkat(const char *target, int newdirfd, const char *linkpath);

__NOTHROW__ extern
int unlink(const char *path);
__NOTHROW__ extern
int unlinkat(int fd, const char *path, int flags);

__NOTHROW__ extern
uid_t getuid(void);
__NOTHROW__ extern
uid_t geteuid(void);

__NOTHROW__ extern
gid_t getgid(void);
__NOTHROW__ extern
gid_t getegid(void);

__NOTHROW__ extern
long int pathconf(const char *__nonnull__ path, int name);
__NOTHROW__ extern
long int fpathconf(int fd, int name);

__NOTHROW__ extern
void sync(void);
__NOTHROW__ extern
int syncfs(int fd);

__NOTHROW__ extern
int fsync(int fd);
__NOTHROW__ extern
int fdatasync(int fd);

__NOTHROW__ extern
long int syscall(long int number, ...);

__NOTHROW__ extern
int getopt(int argc, char *const argv[], const char *optstr);

extern char *optarg;
extern int opterr, optind, optopt;

__EXT_CXX_END__
