#pragma once

#if !(defined(__x86_64__) || defined(__amd64__)) && !defined(__aarch64__)
#error Unsupported target architecture!
#endif /* x86-64 OR aarch64 */

#if !defined(__linux__) && !defined(__linux)
#error Unsupported operating system!
#endif /* Linux */
