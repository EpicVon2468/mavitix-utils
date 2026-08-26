#pragma once

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 202311L
typedef typeof_unqual(sizeof(int)) size_t;
#elif defined(__SIZE_TYPE__)
typedef __SIZE_TYPE__ size_t;
#endif /* size_t */
