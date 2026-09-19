#![no_std]
#![crate_name = "__checks"]
#![crate_type = "cdylib"]

const _: () = cfg_select! {
	target_os = "linux" => (),
	_ => compile_error!("Unsupported operating system!"),
};

const _: () = cfg_select! {
	target_pointer_width = "64" => (),
	_ => compile_error!("Unsupported target pointer width!"),
};

const _: () = cfg_select! {
	target_arch = "x86_64" => (),
	target_arch = "aarch64" => (),
	_ => compile_error!("Unsupported target architecture!"),
};
