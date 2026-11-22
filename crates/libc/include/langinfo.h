#pragma once
#include <internal/__defs.h>

#include <internal/__locale_t.h>
#include <nl_types.h>

__NOTHROW__ extern
const char *nl_langinfo(nl_item item);

__NOTHROW__ extern
const char *nl_langinfo_l(nl_item item, locale_t locale);
