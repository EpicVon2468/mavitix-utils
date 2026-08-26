#pragma once

#define __MAVITIX_LIBC__ 1

#define SYS_NMLN 65

struct utsname {
	char sysname[65];
	char nodename[65];
	char release[65];
	char version[65];
	char machine[65];
	char domainname[65];
};

extern int uname(struct utsname *__name);
