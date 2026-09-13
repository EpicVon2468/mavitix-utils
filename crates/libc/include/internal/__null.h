#pragma once

#ifdef __cplusplus
#define NULL 0
#elif defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L
#define NULL nullptr
#else
#define NULL ((void*) 0)
#endif /* NULL */
