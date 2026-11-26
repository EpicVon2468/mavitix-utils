#![no_std]
#![crate_name = "strlen"]
#![crate_type = "cdylib"]

// The SSE2 and AVX2 implementations of `strlen` in this file are based upon cosmopolitan-libc's implementation(s).
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
	target_feature = "avx2" => {
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn strlen(r#str: *const c_char) -> usize {
			use core::arch::x86_64::{
				__m256i,
				_mm256_cmpeq_epi8,
				_mm256_loadu_si256,
				_mm256_movemask_epi8,
				_mm256_setzero_si256,
			};

			let zeroed_vector: __m256i = unsafe { _mm256_setzero_si256() };
			let mut vector: *mut __m256i = (r#str as isize & -32_isize) as *mut __m256i;
			let skew: isize = r#str as isize & 31_isize;
			let mut mask: u32 = unsafe {
				_mm256_movemask_epi8(_mm256_cmpeq_epi8(_mm256_loadu_si256(vector), zeroed_vector))
			} as u32;
			mask >>= skew;
			mask <<= skew;
			while mask == 0 {
				mask = unsafe {
					_mm256_movemask_epi8(_mm256_cmpeq_epi8(
						_mm256_loadu_si256({
							vector = vector.add(1);
							vector
						}),
						zeroed_vector,
					))
				} as u32;
			}
			unsafe {
				(vector as *const c_char)
					.add(mask.trailing_zeros() as usize)
					.offset_from_unsigned(r#str)
			}
		}
	},
	target_feature = "sse2" => {
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn strlen(r#str: *const c_char) -> usize {
			use core::arch::x86_64::{
				__m128i,
				_mm_cmpeq_epi8,
				_mm_loadu_si128,
				_mm_movemask_epi8,
				_mm_setzero_si128,
			};

			let zeroed_vector: __m128i = unsafe { _mm_setzero_si128() };
			let mut vector: *mut __m128i = (r#str as isize & -16_isize) as *mut __m128i;
			let skew: isize = r#str as isize & 15_isize;
			let mut mask: u32 = unsafe {
				_mm_movemask_epi8(_mm_cmpeq_epi8(_mm_loadu_si128(vector), zeroed_vector))
			} as u32;
			mask >>= skew;
			mask <<= skew;
			while mask == 0 {
				mask = unsafe {
					_mm_movemask_epi8(_mm_cmpeq_epi8(
						_mm_loadu_si128({
							vector = vector.add(1);
							vector
						}),
						zeroed_vector,
					))
				} as u32;
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
