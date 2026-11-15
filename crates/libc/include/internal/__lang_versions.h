#pragma once

#define __C89__ 1
#define __C90__ 1
#define __C95__ 0
#define __C99__ 0
#define __C11__ 0
#define __C17__ 0
#define __C23__ 0

#ifdef __STDC_VERSION__

/*
 * __STDC_VERSION__ was first defined in N325 (C95).
 * Value table:
 * C89: Undefined
 * C90: Undefined
 * C95: 199409L
 * C99: 199901L
 * C11: 201112L
 * C17: 201710L
 * C23: 202311L
 */

#if __STDC_VERSION__ >= 199409L
#undef __C95__
#define __C95__ 1
#endif /* C95 */

#if __STDC_VERSION__ >= 199901L
#undef __C99__
#define __C99__ 1
#endif /* C99 */

#if __STDC_VERSION__ >= 201112L
#undef __C11__
#define __C11__ 1
#endif /* C11 */

#if __STDC_VERSION__ >= 201710L
#undef __C17__
#define __C17__ 1
#endif /* C17 */

#if __STDC_VERSION__ >= 202311L
#undef __C23__
#define __C23__ 1
#endif /* C23 */

#endif /* __STDC_VERSION__ */
