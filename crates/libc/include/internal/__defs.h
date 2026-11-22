#pragma once
#include <internal/__checks.h>
#include <internal/__lang_versions.h>

#define __MAVITIX_LIBC__ 1

#define _POSIX_C_SOURCE 202405L
#define _XOPEN_SOURCE 800

#if !defined(__has_c_attribute)
#define __has_c_attribute(x) 0
#endif /* __has_c_attribute */

#if !defined(__has_declspec_attribute)
#define __has_declspec_attribute(x) 0
#endif /* __has_declspec_attribute */

#if !defined(__has_attribute)
#define __has_attribute(x) 0
#endif /* __has_attribute */

#if !defined(__has_builtin)
#define __has_builtin(x) 0
#endif /* __has_builtin */

#if defined(__is_identifier) && !defined(__has_keyword)
#define __has_keyword(x) !__is_identifier(x)
#elif !defined(__has_keyword)
#define __has_keyword(x) 0
#endif /* __has_keyword */

#if !defined(__has_warning)
#define __has_warning(x) 0
#endif /* __has_warning */

#if !defined(__has_feature)
#define __has_feature(x) 0
#endif /* __has_feature */

#if !defined(__has_extension)
#define __has_extension(x) __has_feature(x)
#endif /* __has_extension */

#if defined(__clang__) || defined(__llvm__) || defined(__GNUC__)
#define __CLANG_LIKE__ 1
#else
#define __CLANG_LIKE__ 0
#endif /* __CLANG_LIKE__ */

#if defined(__GNUC__) && !(defined(__clang__) || defined(__llvm__))
#define __gcc_bad__ __extension__
#else
#define __gcc_bad__
#endif /* __gcc_bad__ */

#if __has_warning("-Wincompatible-library-redeclaration") && __CLANG_LIKE__
#define __INCOMPAT_LIB_REDECL__ 1
#else
#define __INCOMPAT_LIB_REDECL__ 0
#endif /* __INCOMPAT_LIB_REDECL__ */

/*
 * The "complicated" relationship between libc++ and libc [paraphrased]:
 * 1. Currently, libc++'s <cstddef> includes libc++'s own <stddef.h>.
 * 2. Currently, libc++'s <stddef.h> unconditionally includes libc's <stddef.h>.
 * 3. libc++'s <stddef.h> also defines nullptr_t as decltype(nullptr).
 */
#ifdef __cplusplus
#define __EXT_CXX_BEGIN__ extern "C" {
#define __EXT_CXX_END__ }
#else
#define __EXT_CXX_BEGIN__
#define __EXT_CXX_END__
#endif /* __EXT_CXX_BEGIN__ & __EXT_CXX_END__ */

#if __has_keyword(__restrict__)
#define __restrict__ __restrict__
#elif __has_keyword(__restrict)
#define __restrict__ __restrict
#elif __C99__ || __has_keyword(restrict)
#define __restrict__ restrict
#else
#define __restrict__
#endif /* __restrict__ */

#if __has_keyword(__nullptr__)
#define __nullptr__ __nullptr__
#elif __C23__
#define __nullptr__ nullptr
#elif __has_keyword(__nullptr)
#define __nullptr__ __nullptr
#else
#define __nullptr__ ((void *) 0)
#endif /* __nullptr__ */

#if __has_keyword(__typeof__)
#define __typeof__(x) __typeof__(x)
#elif __C23__
#define __typeof__(x) typeof(x)
#elif __has_keyword(__typeof)
#define __typeof__(x) __typeof(x)
#endif /* __typeof__ */

#if __has_keyword(_Nonnull)
#define __nonnull__ _Nonnull
#else
#define __nonnull__
#endif /* __nonnull__ */

#if __has_keyword(_Nullable)
#define __nullable__ _Nullable
#else
#define __nullable__
#endif /* __nullable__ */

#if __has_attribute(returns_nonnull)
#define __RETURNS_NONNULL__ __attribute__((returns_nonnull))
#elif __C23__ && __has_c_attribute(gnu::returns_nonnull)
#define __RETURNS_NONNULL__ [[gnu::returns_nonnull]]
#else
#define __RETURNS_NONNULL__
#endif /* __RETURNS_NONNULL__ */

#if __has_attribute(callback)
#define __CALLBACK__(...) __attribute__((callback(__VA_ARGS__)))
#elif __C23__ && __has_c_attribute(clang::callback)
#define __CALLBACK__(...) [[clang::callback(__VA_ARGS__)]]
#else
#define __CALLBACK__(...)
#endif /* __CALLBACK__ */

#if __has_attribute(unavailable)
#if __has_extension(attribute_unavailable_with_message)
#define __UNAVAILABLE__(x) __attribute__((unavailable(x)))
#else
#define __UNAVAILABLE__(x) __attribute__((unavailable))
#endif /* __attribute__((unavailable)) */
#elif __C23__ && __has_c_attribute(clang::unavailable)
#if __has_extension(attribute_unavailable_with_message)
#define __UNAVAILABLE__(x) [[clang::unavailable(x)]]
#else
#define __UNAVAILABLE__(x) [[clang::unavailable]]
#endif /* [[clang::unavailable]] */
#else
#define __UNAVAILABLE__(x)
#endif /* __UNAVAILABLE__ */

#if __has_attribute(deprecated)
#if __has_extension(attribute_deprecated_with_message)
#define __DEPRECATED__(x) __attribute__((deprecated(x)))
#else
#define __DEPRECATED__(x) __attribute__((deprecated))
#endif /* __attribute__((deprecated)) */
#elif __C23__ && __has_c_attribute(gnu::deprecated)
#if __has_extension(attribute_deprecated_with_message)
#define __DEPRECATED__(x) [[gnu::deprecated(x)]]
#else
#define __DEPRECATED__(x) [[gnu::deprecated]]
#endif /* [[gnu::deprecated]] */
#elif __C23__ && __has_c_attribute(deprecated)
#if __has_extension(attribute_deprecated_with_message)
#define __DEPRECATED__(x) [[deprecated(x)]]
#else
#define __DEPRECATED__(x) [[deprecated]]
#endif /* [[deprecated]] */
#elif __has_declspec_attribute(deprecated)
#if __has_extension(attribute_deprecated_with_message)
#define __DEPRECATED__(x) __declspec(deprecated(x))
#else
#define __DEPRECATED__(x) __declspec(deprecated)
#endif /* __declspec(deprecated) */
#else
#define __DEPRECATED__(x)
#endif /* __DEPRECATED__ */

#if __has_attribute(nothrow)
#define __NOTHROW__ __attribute__((nothrow))
#elif __C23__ && __has_c_attribute(gnu::nothrow)
#define __NOTHROW__ [[gnu::nothrow]]
#elif __has_declspec_attribute(nothrow)
#define __NOTHROW__ __declspec(nothrow)
#else
#define __NOTHROW__
#endif /* __NOTHROW__ */

#if __has_attribute(used)
#define __USED__ __attribute__((used))
#elif __C23__ && __has_c_attribute(gnu::used)
#define __USED__ [[gnu::used]]
#else
#define __USED__
#endif /* __USED__ */

#if __has_attribute(const)
#define __CONST__ __attribute__((const))
#elif __has_attribute(__const)
#define __CONST__ __attribute__((__const))
#elif __C23__ && __has_c_attribute(gnu::const)
#define __CONST__ [[gnu::const]]
#elif __C23__ && __has_c_attribute(gnu::__const)
#define __CONST__ [[gnu::__const]]
#else
#define __CONST__
#endif /* __CONST__ */

#if __C11__ || defined(__cplusplus) || __has_keyword(inline)
#define __INLINE__ inline
#elif __has_keyword(__inline__)
#define __INLINE__ __inline__
#elif __has_keyword(__inline)
#define __INLINE__ __inline
#else
#define __INLINE__
#endif /* __INLINE__ */

#if __has_attribute(noinline)
#define __NOINLINE__ __attribute__((noinline))
#elif __C23__ && __has_c_attribute(clang::noinline)
#define __NOINLINE__ [[clang::noinline]]
#elif __C23__ && __has_c_attribute(gnu::noinline)
#define __NOINLINE__ [[gnu::noinline]]
#elif __C23__ && __has_c_attribute(msvc::noinline)
#define __NOINLINE__ [[msvc::noinline]]
#elif __has_declspec_attribute(noinline)
#define __NOINLINE__ __declspec(noinline)
#else
#define __NOINLINE__
#endif /* __NOINLINE__ */

#if __has_attribute(pure)
#define __PURE__ __attribute__((pure))
#elif __C23__ && __has_c_attribute(gnu::pure)
#define __PURE__ [[gnu::pure]]
#else
#define __PURE__
#endif /* __PURE__ */

#if __has_attribute(visibility)
#define __PROTECTED__ __attribute__((visibility("protected")))
#define __HIDDEN__ __attribute__((visibility("hidden")))
#elif __C23__ && __has_c_attribute(gnu::visibility)
#define __PROTECTED__ [[gnu::visibility("protected")]]
#define __HIDDEN__ [[gnu::visibility("hidden")]]
#else
#define __PROTECTED__
#define __HIDDEN__
#endif /* __PROTECTED__ & __HIDDEN__ */

#if __has_attribute(weak)
#define __WEAK__ __attribute__((weak))
#elif __C23__ && __has_c_attribute(gnu::weak)
#define __WEAK__ [[gnu::weak]]
#else
#define __WEAK__
#endif /* __WEAK__ */

#if __has_attribute(noreturn)
#define __NORETURN__ __attribute__((noreturn))
#elif __C23__ && __has_c_attribute(gnu::noreturn)
#define __NORETURN__ [[gnu::noreturn]]
#elif __C23__ && __has_c_attribute(noreturn)
#define __NORETURN__ [[noreturn]]
#elif __C23__ && __has_c_attribute(_Noreturn)
#define __NORETURN__ [[_Noreturn]]
#elif __has_declspec_attribute(noreturn)
#define __NORETURN__ __declspec(noreturn)
#elif __C11__ || __has_keyword(_Noreturn)
#define __NORETURN__ _Noreturn
#else
#define __NORETURN__
#endif /* __NORETURN__ */

#if __C23__ && __has_c_attribute(clang::likely)
#define _AttrLikely(x) (x) [[clang::likely]]
#else
#define _AttrLikely(x) (x)
#endif /* _AttrLikely */

#if __C23__ && __has_c_attribute(clang::unlikely)
#define _AttrUnlikely(x) (x) [[clang::unlikely]]
#else
#define _AttrUnlikely(x) (x)
#endif /* _AttrUnlikely */

#if __has_builtin(__builtin_expect)
#define __likely(x) _AttrLikely(__builtin_expect((x), 1))
#define __unlikely(x) _AttrUnlikely(__builtin_expect((x), 0))
#else
#define __likely(x) _AttrLikely(x)
#define __unlikely(x) _AttrUnlikely(x)
#endif /* __likely & __unlikely */

#ifdef __MAVITIX_INTERNAL__

#define __LINKER_SYMBOL__(x) \
	__HIDDEN__ __WEAK__ extern \
	void (*const x[])(void)

typedef void (*__init_t)(void);
typedef void (*__fini_t)(void);
typedef int (*__main_t)(int argc, char **argv, char **envp);

#endif
