.intel_syntax noprefix

.file "info.s"

.section .rodata, "", @progbits
	# .internal __version
	# .type __version, @object

	.global __MAVITIX_LIBC_VERSION__
	.type __MAVITIX_LIBC_VERSION__, @object

	# __version:
		# .asciz "0.0.0-git"
		# .size __version, . - __version

	# __MAVITIX_LIBC_VERSION__:
		# .quad __version@GOTPCREL + .
		# .size __MAVITIX_LIBC_VERSION__, . - __MAVITIX_LIBC_VERSION__

	# Current type is actually `char[0xA]`.
	# Below this point I did some messing around trying to fix it
	# However, I ended up with with segfaults, issues with PIC,
	# and in one case (`@gotoff`) text relocations.
	# Tried:
	# - `@got{,off}`
	# - `@plt`
	# - `@GOTPCREL` (this one doesn't even have support from `ld.wild` or `ld.lld`)
	__MAVITIX_LIBC_VERSION__:
		.asciz "0.0.0-git"

.ident "libc: mavitix libc 0.0.0-git"

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", @progbits
