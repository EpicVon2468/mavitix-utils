#![allow(non_upper_case_globals)]

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {

	pub const x86_64_v4: bool = cfg!(all(
		target_feature = "avx512f",
		target_feature = "avx512bw",
		target_feature = "avx512cd",
		target_feature = "avx512dq",
		target_feature = "avx512vl",
	));

	pub const x86_64_v3: bool = cfg!(all(
		target_feature = "avx",
		target_feature = "avx2",
		target_feature = "bmi1",
		target_feature = "bmi2",
		target_feature = "f16c",
		target_feature = "fma",
		target_feature = "lzcnt",
		target_feature = "movbe",
		target_feature = "xsave",
	));

	pub const x86_64_v2: bool = cfg!(all(
		target_feature = "cmpxchg16b",
		target_feature = "lahfsahf",
		target_feature = "popcnt",
		target_feature = "sse3",
		target_feature = "sse4.1",
		target_feature = "sse4.2",
		target_feature = "ssse3",
	));

	pub const x86_64_v1: bool = true;
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {

	pub const ARMv9_5_A: bool = cfg!(target_feature = "v9.5a");

	pub const ARMv9_4_A: bool = cfg!(target_feature = "v9.4a");

	pub const ARMv9_3_A: bool = cfg!(target_feature = "v9.3a");

	pub const ARMv9_2_A: bool = cfg!(target_feature = "v9.2a");

	pub const ARMv9_1_A: bool = cfg!(target_feature = "v9.1a");

	pub const ARMv9_0_A: bool = cfg!(target_feature = "v9a");

	pub const ARMv8_9_A: bool = cfg!(target_feature = "v8.9a");

	pub const ARMv8_8_A: bool = cfg!(target_feature = "v8.8a");

	pub const ARMv8_7_A: bool = cfg!(target_feature = "v8.7a");

	pub const ARMv8_6_A: bool = cfg!(target_feature = "v8.6a");

	pub const ARMv8_5_A: bool = cfg!(target_feature = "v8.5a");

	pub const ARMv8_4_A: bool = cfg!(target_feature = "v8.4a");

	pub const ARMv8_3_A: bool = cfg!(target_feature = "v8.3a");

	pub const ARMv8_2_A: bool = cfg!(target_feature = "v8.2a");

	pub const ARMv8_1_A: bool = cfg!(target_feature = "v8.1a");

	pub const ARMv8_0_A: bool = true;
}
