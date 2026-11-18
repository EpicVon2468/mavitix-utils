# Mavitix C Standard Library

A mostly-compliant `libc` implementation.

## Architecture Support

- `x86-64`
- `aarch64`
- `riscv64`

## Notable Differences

- The `setlocale` function returns `const char *` instead of `char *`.
- The `strerror` family of functions return `const char *` instead of `char *`.
- The `getenv` function returns `const char *` instead of `char *`.
- The `lconv` struct uses `const char *` instead of `char *` for its string fields.
- the `localeconv` function returns `const struct lconv *` instead of `struct lconv *`.
- The `toascii` function is attributed with `unavailable` and/or `deprecated` if available.
- The `gets` function is attributed with `unavailable` and/or `deprecated` if available.
- Functions in the `{__,}isCHARACTERISTIC_l` set are implemented as symbol aliases to `isCHARACTERISTIC`; the `locale_t` parameter is ignored.
- The buildscript aggressively optimises for the host machine.  Cross-compilation is a work-in-progress.
- POSIX IPv6 support is not likely to be added.
- The localisation categories `LC_PAPER`, `LC_MEASUREMENT`, and `LC_IDENTIFICATION` are not supported.
