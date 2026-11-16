#pragma once
#include <internal/__defs.h>

__EXT_CXX_BEGIN__

/*
 * https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/basedefs/utmpx.h.html
 */

__NOTHROW__ extern
void endutxent(void);

__NOTHROW__ extern
void setutxent(void);

__EXT_CXX_END__
