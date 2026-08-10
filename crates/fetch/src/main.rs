#![feature(derive_const, const_default, const_trait_impl)]

use std::env::{consts::ARCH, var as get_var};

use mavitix_utils::{
	bold,
	const_println,
	passwd,
	uname::{get_uname, utsname},
};

pub fn main() {
	let utsname: utsname = get_uname();
	let info: FetchInfo = FetchInfo {
		username: 'username: {
			if let Some(value) = passwd::get_username() {
				break 'username value;
			};
			if let Ok(value) = get_var("USER") {
				break 'username value;
			};
			"<unknown>".to_owned()
		},
		hostname: utsname.get_nodename(),
		kernel: utsname.get_release(),
		terminal: 'term: {
			if let Ok(value) = get_var("LC_TERMINAL") {
				break 'term value;
			};
			if let Ok(_) = get_var("ALACRITTY_WINDOW_ID") {
				break 'term "alacritty".to_owned();
			};
			// Kitty is PythonSlop™ (read: low-quality (read: Python), vibecoded).
			// if let Ok(_) = get_var("KITTY_INSTALLATION_DIR") {
			// 	break 'term "kitty".to_owned();
			// };
			if let Ok(_) = get_var("KONSOLE_VERSION") {
				break 'term "konsole".to_owned();
			};
			"<unknown>".to_owned()
		},
		shell: match get_var("SHELL") {
			Ok(value) => value,
			Err(_) => "<unknown>".to_owned(),
		},
		editor: 'editor: {
			if let Ok(value) = get_var("VISUAL") {
				break 'editor value;
			};
			if let Ok(value) = get_var("EDITOR") {
				break 'editor value;
			};
			"<unknown>".to_owned()
		},
		locale: 'locale: {
			if let Ok(value) = get_var("LC_ALL") {
				break 'locale value;
			};
			if let Ok(value) = get_var("LANG") {
				break 'locale value;
			};
			if let Ok(value) = get_var("LANGUAGE") {
				break 'locale value;
			};
			"C.UTF-8".to_owned()
		},
		..Default::default()
	};
	do_print(info);
}

#[derive_const(Default)]
pub struct FetchInfo {
	pub username: String,
	pub hostname: String,
	pub pretty_hostname: Option<String>,
	pub host: String,
	pub kernel: String,
	pub os_name: String,
	pub terminal: String,
	pub shell: String,
	pub editor: String,
	pub locale: String,
}

pub fn do_print(info: FetchInfo) {
	{
		let hostname: String = info.hostname;
		let username: String = info.username;
		println!(concat!(bold!("{}"), '@', bold!("{}")), username, hostname);
		let len: usize = username.len() + 1 + hostname.len();
		// SAFETY: Trusted value.
		println!("{}", unsafe {
			String::from_utf8_unchecked(vec![b'-'; len])
		});
	};
	const_println!(concat!(bold!("Architecture:"), ' '), ARCH);
	println!(concat!(bold!("Kernel:"), " Linux {}"), info.kernel);
	println!(concat!(bold!("Terminal:"), " {}"), info.terminal);
	println!(concat!(bold!("Shell:"), " {}"), info.shell);
	println!(concat!(bold!("Locale:"), " {}"), info.locale);
}
