#pragma once
#include <internal/__defs.h>

#define __bool_true_false_are_defined 1

#if __C99__ && !__C23__
#define bool _Bool
#define true 1
#define false 0
#endif
