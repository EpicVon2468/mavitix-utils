#pragma once

#if defined(__x86_64__) || defined(__amd64__)
#define __X86_64__ 1
#else
#define __X86_64__ 0
#endif /* __X86_64__ */

#if defined(__aarch64__)
#define __AARCH64__ 1
#else
#define __AARCH64__ 0
#endif /* __AARCH64__ */

#if (defined(__riscv__) || defined(__riscv)) && defined(__LP64__)
#define __RISCV64__ 1
#else
#define __RISCV64__ 0
#endif /* __RISCV64__ */

#if !(defined(__linux__) || defined(__linux))
#error "Unsupported operating system!"
#endif

#if !defined(__LP64__)
/* https://archive.opengroup.org/public/tech/aspen/lp64_wp.htm */
#error "Unsupported 64-bit programming model!"
#endif

#if !__X86_64__ && !__AARCH64__ && !__RISCV64__
#error "Unsupported target architecture!"
#endif

#if defined(__cplusplus) && defined(__MAVITIX_FORBID_CXX)
#error "libc headers used in C++ compilation when C++ is forbidden!"
#endif /* C++ */
