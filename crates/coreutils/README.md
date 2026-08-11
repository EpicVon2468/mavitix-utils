# Mavitix Coreutils

A GNU-compatible coreutils implementation.<br>
Implementation behaviour is deduced from man pages, texinfo pages, and brute force testing of the GNU coreutils.<br>
The source code of the GNU coreutils has not been inspected during creation for legal reasons.

The following extensions have been added:

- Support for multiple files per invocation in `mavitix-unlink`.
- Support for `-h` as an alias for `--help` in all programs.

Many utilities from the coreutils may not be featured in this package for complexity reasons.<br>
The currently implemented utilities are as follows:

- `whoami(1)`
- `printenv(1)`
- `arch(1)`
- `logname(1)`
- `unlink(1)`
- `false(1)`
- `true(1)`
- `tty(1)`
- `yes(1)`

FIXME: Some issues in compat around how GNU coreutils handle `--help` & `--version`.