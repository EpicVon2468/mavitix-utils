#pragma once
#include <internal/__defs.h>

/*
 * https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/utmpx.h.html
 */

#ifdef __cplusplus
extern "C" {
#endif

extern void endutxent(void);
extern void setutxent(void);

#ifdef __cplusplus
}
#endif
