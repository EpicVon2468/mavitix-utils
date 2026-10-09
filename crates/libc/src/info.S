.intel_syntax noprefix

.section .rodata, "", @progbits
	.global __MAVITIX_LIBC_VERSION__
	.type __MAVITIX_LIBC_VERSION__, @object

	__MAVITIX_LIBC_VERSION__:
		.asciz "0.0.0-git"

/*
 * Why is `.comment` like this...
 *
 * `.comment` is `@progbits`, despite `@note` existing and being far more relevant to how `.comment` is used.
 * `.comment` is "", despite "S" existing and being far more relevant to how `.comment` is used.
 */
.section .comment, "", @progbits
	.asciz "libc: mavitix libc 0.0.0-git"

# https://wiki.gentoo.org/wiki/Hardened/GNU_stack_quickstart
.section .note.GNU-stack, "", @progbits
