#pragma once
#include <internal/__defs.h>

#define EX_OK 0

#define EX__BASE 64

/* cli use */
#define EX_USAGE 64
/* data format */
#define EX_DATAERR 65
#define EX_NOINPUT 66
/* no such user */
#define EX_NOUSER 67
#define EX_NOHOST 68
/* service unavailable */
#define EX_UNAVAILABLE 69
/* internal software problem */
#define EX_SOFTWARE 70
/* system problem */
#define EX_OSERR 71
/* critial os file missing */
#define EX_OSFILE 72
/* cannot create output file */
#define EX_CANTCREAT 73
/* i/o */
#define EX_IOERR 74
/* temporary failure */
#define EX_TEMPFAIL 75
#define EX_PROTOCOL 76
/* insufficient permission */
#define EX_NOPERM 77
/* misconfiguration or invalid configuration */
#define EX_CONFIG 78

#define EX__MAX 78
