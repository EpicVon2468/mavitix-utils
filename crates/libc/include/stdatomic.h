#include <internal/__defs.h>

#define __STDC_VERSION_STDATOMIC_H__ 202311L

#ifdef __cplusplus
extern "C" {
#endif

typedef enum memory_order {
	memory_order_relaxed,
	memory_order_consume,
	memory_order_acquire,
	memory_order_release,
	memory_order_acq_rel,
	memory_order_seq_cst,
} memory_order;

#include <stdbool.h>

typedef _Atomic bool atomic_bool;

typedef _Atomic char atomic_char;
typedef _Atomic signed char atomic_schar;
typedef _Atomic unsigned char atomic_uchar;

typedef _Atomic signed short atomic_short;
typedef _Atomic unsigned short atomic_ushort;

typedef _Atomic signed int atomic_int;
typedef _Atomic unsigned int atomic_uint;

typedef _Atomic signed long atomic_long;
typedef _Atomic unsigned long atomic_ulong;

typedef _Atomic signed long long atomic_llong;
typedef _Atomic unsigned long long atomic_ullong;

#include <stdint.h>

// typedef _Atomic char8_t atomic_char8_t;
// typedef _Atomic char16_t atomic_char16_t;
// typedef _Atomic char32_t atomic_char32_t;
// typedef _Atomic wchar_t atomic_wchar_t;

typedef _Atomic int_least8_t atomic_int_least8_t;
typedef _Atomic uint_least8_t atomic_uint_least8_t;

typedef _Atomic int_least16_t atomic_int_least16_t;
typedef _Atomic uint_least16_t atomic_uint_least16_t;

typedef _Atomic int_least32_t atomic_int_least32_t;
typedef _Atomic uint_least32_t atomic_uint_least32_t;

typedef _Atomic int_least64_t atomic_int_least64_t;
typedef _Atomic uint_least64_t atomic_uint_least64_t;

// typedef _Atomic int_fast8_t atomic_int_fast8_t;
// typedef _Atomic uint_fast8_t atomic_uint_fast8_t;

// typedef _Atomic int_fast16_t atomic_int_fast16_t;
// typedef _Atomic uint_fast16_t atomic_uint_fast16_t;

// typedef _Atomic int_fast32_t atomic_int_fast32_t;
// typedef _Atomic uint_fast32_t atomic_uint_fast32_t;

// typedef _Atomic int_fast64_t atomic_int_fast64_t;
// typedef _Atomic uint_fast64_t atomic_uint_fast64_t;

typedef _Atomic intptr_t atomic_intptr_t;
typedef _Atomic uintptr_t atomic_uintptr_t;

#include <internal/__size_t.h>
typedef _Atomic size_t atomic_size_t;

// typedef _Atomic ptrdiff_t atomic_ptrdiff_t;

// typedef _Atomic intmax_t atomic_intmax_t;
// typedef _Atomic uintmax_t atomic_uintmax_t;

/*
 * https://clang.llvm.org/docs/LanguageExtensions.html#c11-atomic-builtins
 * https://gcc.gnu.org/onlinedocs/gcc/_005f_005fatomic-Builtins.html
 */

#if __has_builtin(__c11_atomic_init)
#define atomic_init(ptr, value) __c11_atomic_init(ptr, value)
#else
#define atomic_init(ptr, value) atomic_store_explicit(ptr, value, memory_order_relaxed)
#endif /* atomic_init */

__NOTHROW__ extern
void atomic_thread_fence(memory_order order);
__NOTHROW__ extern
void atomic_signal_fence(memory_order order);

#define atomic_store(ptr, value) atomic_store_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_store)
#define atomic_store_explicit(ptr, value, order) __c11_atomic_store(ptr, value, order)
#elif __has_builtin(__atomic_store_n)
#define atomic_store_explicit(ptr, value, order) __atomic_store_n(ptr, value, order)
#endif /* atomic_store_explicit */

#define atomic_load(ptr) atomic_load_explicit(ptr, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_load)
#define atomic_load_explicit(ptr, order) __c11_atomic_load(ptr, order)
#elif __has_builtin(__atomic_load_n)
#define atomic_load_explicit(ptr, order) __atomic_load_n(ptr, order)
#endif /* atomic_load_explicit */

#define atomic_exchange(ptr, value) atomic_exchange_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_exchange)
#define atomic_exchange_explicit(ptr, value, order) __c11_atomic_exchange(ptr, value, order)
#elif __has_builtin(__atomic_exchange_n)
#define atomic_exchange_explicit(ptr, value, order) __atomic_exchange_n(ptr, value, order)
#endif /* atomic_exchange_explicit */

#define atomic_fetch_add(ptr, value) atomic_fetch_add_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_fetch_add)
#define atomic_fetch_add_explicit(ptr, value, order) __c11_atomic_fetch_add(ptr, value, order)
#elif __has_builtin(__atomic_fetch_add)
#define atomic_fetch_add_explicit(ptr, value, order) __atomic_fetch_add(ptr, value, order)
#endif /* atomic_fetch_add_explicit */

#define atomic_fetch_sub(ptr, value) atomic_fetch_sub_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_fetch_sub)
#define atomic_fetch_sub_explicit(ptr, value, order) __c11_atomic_fetch_sub(ptr, value, order)
#elif __has_builtin(__atomic_fetch_sub)
#define atomic_fetch_sub_explicit(ptr, value, order) __atomic_fetch_sub(ptr, value, order)
#endif /* atomic_fetch_sub_explicit */

#define atomic_fetch_or(ptr, value) atomic_fetch_or_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_fetch_or)
#define atomic_fetch_or_explicit(ptr, value, order) __c11_atomic_fetch_or(ptr, value, order)
#elif __has_builtin(__atomic_fetch_or)
#define atomic_fetch_or_explicit(ptr, value, order) __atomic_fetch_or(ptr, value, order)
#endif /* atomic_fetch_or_explicit */

#define atomic_fetch_xor(ptr, value) atomic_fetch_xor_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_fetch_xor)
#define atomic_fetch_xor_explicit(ptr, value, order) __c11_atomic_fetch_xor(ptr, value, order)
#elif __has_builtin(__atomic_fetch_xor)
#define atomic_fetch_xor_explicit(ptr, value, order) __atomic_fetch_xor(ptr, value, order)
#endif /* atomic_fetch_xor_explicit */

#define atomic_fetch_and(ptr, value) atomic_fetch_and_explicit(ptr, value, memory_order_seq_cst)

#if __has_builtin(__c11_atomic_fetch_and)
#define atomic_fetch_and_explicit(ptr, value, order) __c11_atomic_fetch_and(ptr, value, order)
#elif __has_builtin(__atomic_fetch_and)
#define atomic_fetch_add_explicit(ptr, value, order) __atomic_fetch_and(ptr, value, order)
#endif /* atomic_fetch_and_explicit */

typedef struct atomic_flag {
	atomic_bool state;
} atomic_flag;

#define ATOMIC_FLAG_INIT { false }

__NOTHROW__ extern
bool atomic_flag_test_and_set(volatile atomic_flag *ptr);
__NOTHROW__ extern
bool atomic_flag_test_and_set_explicit(
	volatile atomic_flag *ptr,
	memory_order order
);

__NOTHROW__ extern
void atomic_flag_clear(volatile atomic_flag *ptr);
__NOTHROW__ extern
void atomic_flag_clear_explicit(volatile atomic_flag *ptr, memory_order order);

#ifdef __cplusplus
}
#endif
