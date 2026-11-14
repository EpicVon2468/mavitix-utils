# Mavitix libc

A mostly-compliant libc implementation.

## Architecture Support

- `x86-64`
- `aarch64`
- `riscv64`

## Notable Differences:

- The `setlocale` function returns `const char *` instead of `char *`.
- The `toascii` function is attributed with `unavailable` and/or `deprecated` if available.
- Functions in the `{__,}isCHARACTERISTIC_l` set are implemented as symbol aliases to `isCHARACTERISTIC`; the `locale_t` parameter is ignored.
- The buildscript aggressively optimises for the host machine.  Cross-compilation is a work-in-progress.
- POSIX IPv6 support is not likely to be added.
