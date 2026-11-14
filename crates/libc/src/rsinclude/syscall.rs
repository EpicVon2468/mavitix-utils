macro_rules! syscall {
	(
		$number:expr $(
		, $arg0:expr $(
		, $arg1:expr $(
		, $arg2:expr $(
		, $arg3:expr $(
		, $arg4:expr $(
		, $arg5:expr
		)?)?)?)?)?)?
		$(; $($opt:ident),*)?
		$(,)?
	) => {{
		let result: i64;
		// SAFETY: UB-free
		unsafe {
			cfg_select! {
				target_arch = "x86_64" => core::arch::asm!(
					"syscall",
					in("rax") $number, $(
					in("rdi") $arg0, $(
					in("rsi") $arg1, $(
					in("rdx") $arg2, $(
					in("r10") $arg3, $(
					in("r8") $arg4, $(
					in("r9") $arg5,
					)?)?)?)?)?)?
					lateout("rax") result,
					options(raw $(, $($opt),*)?),
				),
				target_arch = "aarch64" => core::arch::asm!(
					"svc #0",
					in("x8") $number, $(
					in("x0") $arg0, $(
					in("x1") $arg1, $(
					in("x2") $arg2, $(
					in("x3") $arg3, $(
					in("x4") $arg4, $(
					in("x5") $arg5,
					)?)?)?)?)?)?
					lateout("x0") result,
					options(raw $(, $($opt),*)?),
				),
				target_arch = "riscv64" => core::arch::asm!(
					"ecall",
					in("a7") $number, $(
					in("a0") $arg0, $(
					in("a1") $arg1, $(
					in("a2") $arg2, $(
					in("a3") $arg3, $(
					in("a4") $arg4, $(
					in("a5") $arg5,
					)?)?)?)?)?)?
					lateout("a0") result,
					options(raw $(, $($opt),*)?),
				),
				_ => compile_error!(),
			}
		};
		if core::hint::unlikely(result as u64 > const { -4096i64 as u64 }) {
			unsafe extern "C" {

				#[unsafe(ffi_const)]
				pub safe fn __errno_location() -> *mut i32;
			}

			// SAFETY: `__errno_location()` is always non-null.
			unsafe {
				*__errno_location() = -result as i32;
			};
			-1
		} else {
			result
		}
	}};
}
