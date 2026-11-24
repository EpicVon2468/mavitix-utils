# Mavitix C Standard Library

A mostly-compliant `libc` implementation.<br>
Aims to (mostly) follow POSIX.1-2024, C23, and any Linux-specific behaviour.

## Architecture Support

- `x86-64`
- `aarch64`
- `riscv64`

Only 64-bit processors are supported, and only under the `LP64` programming model.

## Notable Differences

- The `setlocale` function returns `const char *` instead of `char *`.
- The `strerror` family of functions return `const char *` instead of `char *`.
- The `getenv` function returns `const char *` instead of `char *`.
- The `lconv` struct uses `const char *` instead of `char *` for its string fields.
- the `localeconv` function returns `const struct lconv *` instead of `struct lconv *`.
- The `toascii` function is attributed with `unavailable` and/or `deprecated` if available.
- The `gets` function is attributed with `unavailable` and/or `deprecated` if available.
- Mutation of the `environ` static variable by user programs or libraries is considered undefined behaviour under all circumstances, [as per spec](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html#tag_08:~:text=If%20the%20application%20modifies%20the%20pointers%20to%20which%20environ%20points%2C%20the%20behavior%20of%20all%20interfaces%20described%20in%20the%20System%20Interfaces%20volume%20of%20POSIX%2E1%2D2024%20is%20undefined).
- Functions in the `{__,}isCHARACTERISTIC_l` set are implemented as symbol aliases to `isCHARACTERISTIC`; the `locale_t` parameter is ignored.
- The buildscript aggressively optimises for the host machine.  Cross-compilation is a work-in-progress.
- POSIX IPv6 support is not likely to be added.
- `localedef` support won't be added unless the specification is changed to require it.
- The localisation categories `LC_PAPER`, `LC_MEASUREMENT`, and `LC_IDENTIFICATION` are not supported.
- Language codes can be arbitrarily used alongside territory codes without restriction.
- The UNIX-style `num$` syntax for the `printf` family of functions is unlikely to be supported due to incredibly high technical complexity.
