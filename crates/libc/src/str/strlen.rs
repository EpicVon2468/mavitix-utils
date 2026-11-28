#![no_std]
#![crate_name = "strlen"]
#![crate_type = "cdylib"]
#![allow(non_camel_case_types)]

// The x86-64 implementation of `strlen` in this file is based upon cosmopolitan-libc's implementation(s).
// You can view their licence at https://github.com/jart/cosmopolitan/blob/master/LICENSE.
// At the time of writing, the notice was:
// -----------------------------------------------------------------------
// ISC License
//
// Copyright 2020 Justine Alexandra Roberts Tunney
//
// Permission to use, copy, modify, and/or distribute this software for
// any purpose with or without fee is hereby granted, provided that the
// above copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
// WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
// AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL
// DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR
// PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
// -----------------------------------------------------------------------

use core::ffi::c_char;

cfg_select! {
	target_arch = "x86_64" => {
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn strlen(r#str: *const c_char) -> usize {
			cfg_select! {
				all(target_feature = "avx512f", target_feature = "avx512bw") => {
					use core::arch::x86_64::{
						__m512i as __simd_ivec,
						__mmask64,
						_mm512_cmpeq_epi8_mask as __cmpeq_epi8,
						_mm512_loadu_si512 as __loadu_si,
						_mm512_setzero_si512 as __setzero_si,
					};

					type __mask = u64;

					#[inline(always)]
					fn __movemask_epi8(mask: __mmask64) -> __mask {
						return mask as __mask;
					}

					const NEG_BITS: isize = -64_isize;
					const POS_BITS: isize = 63_isize;
				},
				target_feature = "avx2" => {
					use core::arch::x86_64::{
						__m256i as __simd_ivec,
						_mm256_cmpeq_epi8 as __cmpeq_epi8,
						_mm256_loadu_si256 as __loadu_si,
						_mm256_movemask_epi8 as __movemask_epi8,
						_mm256_setzero_si256 as __setzero_si,
					};

					type __mask = u32;

					const NEG_BITS: isize = -32_isize;
					const POS_BITS: isize = 31_isize;
				},
				// sse2 is x86-64-v1
				target_feature = "sse2" => {
					use core::arch::x86_64::{
						__m128i as __simd_ivec,
						_mm_cmpeq_epi8 as __cmpeq_epi8,
						_mm_loadu_si128 as __loadu_si,
						_mm_movemask_epi8 as __movemask_epi8,
						_mm_setzero_si128 as __setzero_si,
					};

					type __mask = u32;

					const NEG_BITS: isize = -16_isize;
					const POS_BITS: isize = 15_isize;
				},
			};

			let zeroed_vector: __simd_ivec = unsafe { __setzero_si() };
			let mut vector: *mut __simd_ivec = (r#str as isize & NEG_BITS) as *mut __simd_ivec;
			let skew: isize = r#str as isize & POS_BITS;
			let mut mask: __mask =
				unsafe { __movemask_epi8(__cmpeq_epi8(__loadu_si(vector), zeroed_vector)) }
					as __mask;
			mask >>= skew;
			mask <<= skew;
			while mask == 0 {
				mask = unsafe {
					__movemask_epi8(__cmpeq_epi8(
						__loadu_si({
							vector = vector.add(1);
							vector
						}),
						zeroed_vector,
					))
				} as __mask;
			}
			unsafe {
				(vector as *const c_char)
					.add(mask.trailing_zeros() as usize)
					.offset_from_unsigned(r#str)
			}
		}
	},
	_ => {
		#[unsafe(no_mangle)]
		pub const unsafe extern "C" fn strlen(mut r#str: *const c_char) -> usize {
			let start: *const c_char = r#str;
			loop {
				// SAFETY: Callers ensure input is well-formed.
				if unsafe { *r#str } == 0 {
					// SAFETY:
					return unsafe { r#str.offset_from_unsigned(start) };
				};
				// SAFETY: Callers ensure input is well-formed.
				r#str = unsafe { r#str.add(1) };
			}
		}
	},
}
