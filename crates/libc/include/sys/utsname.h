#pragma once
#include <internal/__defs.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SYS_NMLN 65

struct utsname {
	char sysname[65];
	char nodename[65];
	char release[65];
	char version[65];
	char machine[65];
	char domainname[65];
};

__NOTHROW__ extern
int uname(struct utsname *name);

#ifdef __cplusplus
}
#endif
