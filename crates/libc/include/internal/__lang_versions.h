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

/* C++ (ab)uses C headers when it has no right to, causing pain and suffering */

#define __CXX98__ 0
#define __CXX11__ 0
#define __CXX14__ 0
#define __CXX17__ 0
#define __CXX20__ 0
#define __CXX23__ 0
#define __CXX26__ 0

#ifdef __cplusplus

#if __cplusplus >= 199711L
#undef __CXX98__
#define __CXX98__ 1
#endif /* C++98 & C++03 */

#if __cplusplus >= 201103L
#undef __CXX11__
#define __CXX11__ 1
#endif /* C++11 */

#if __cplusplus >= 201402L
#undef __CXX14__
#define __CXX14__ 1
#endif /* C++14 */

#if __cplusplus >= 201703L
#undef __CXX17__
#define __CXX17__ 1
#endif /* C++17 */

#if __cplusplus >= 202002L
#undef __CXX20__
#define __CXX20__ 1
#endif /* C++20 */

#if __cplusplus >= 202302L
#undef __CXX23__
#define __CXX23__ 1
#endif /* C++23 */

#if __cplusplus >= 202603L
#undef __CXX26__
#define __CXX26__ 1
#endif /* C++26 */

#endif /* __cplusplus */
