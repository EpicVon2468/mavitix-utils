#![feature(derive_const, const_default, const_trait_impl)]

use std::env::var as get_var;

use mavitix_utils::{
	passwd,
	uname::{UnixTimesharingSystemName as UTSName, get_uname},
};

pub fn main() {
	let utsname: UTSName = get_uname();
	let info: FetchInfo = FetchInfo {
		username: 'username: {
			match passwd::get_username() {
				Some(value) => break 'username value,
				None => (),
			};
			match get_var("USER") {
				Ok(value) => break 'username value,
				Err(_) => (),
			};
			"<unknown>".to_owned()
		},
		hostname: utsname.nodename,
		kernel: utsname.release,
		terminal: 'term: {
			match get_var("LC_TERMINAL") {
				Ok(value) => break 'term value,
				Err(_) => (),
			};
			match get_var("ALACRITTY_WINDOW_ID") {
				Ok(_) => break 'term "alacritty".to_owned(),
				Err(_) => (),
			};
			// Kitty is PythonSlop™ (read: low-quality (read: Python), vibecoded).
			// match get_var("KITTY_INSTALLATION_DIR") {
			// 	Ok(_) => break 'term "kitty".to_owned(),
			// 	Err(_) => (),
			// };
			match get_var("KONSOLE_VERSION") {
				Ok(_) => break 'term "konsole".to_owned(),
				Err(_) => (),
			};
			"<unknown>".to_owned()
		},
		shell: match get_var("SHELL") {
			Ok(value) => value,
			Err(_) => "<unknown>".to_owned(),
		},
		editor: 'editor: {
			match get_var("VISUAL") {
				Ok(value) => break 'editor value,
				Err(_) => (),
			}
			match get_var("EDITOR") {
				Ok(value) => break 'editor value,
				Err(_) => (),
			};
			"<unknown>".to_owned()
		},
		locale: 'locale: {
			match get_var("LC_ALL") {
				Ok(value) => break 'locale value,
				Err(_) => (),
			};
			match get_var("LANG") {
				Ok(value) => break 'locale value,
				Err(_) => (),
			};
			match get_var("LANGUAGE") {
				Ok(value) => break 'locale value,
				Err(_) => (),
			};
			"C".to_owned()
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

macro_rules! bold {
	($value:expr $(,)?) => {
		concat!("\x1B[1m", $value, "\x1B[22m")
	};
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
	println!(
		concat!(bold!("Architecture:"), " {}"),
		std::env::consts::ARCH,
	);
	println!(concat!(bold!("Kernel:"), " Linux {}"), info.kernel);
	println!(concat!(bold!("Terminal:"), " {}"), info.terminal);
	println!(concat!(bold!("Shell:"), " {}"), info.shell);
	println!(concat!(bold!("Locale:"), " {}"), info.locale);
}
