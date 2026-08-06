use anyhow::Result;

use mavitix_utils::{cli::ArgIter, main};

pub mod cli;

main!(main_impl());

pub fn main_impl() -> Result<()> {
	let mut args: ArgIter = ArgIter::new();
	while let Some(arg) = args.next() {
		let (first, second, rest): (char, char, String) = ArgIter::destructure(arg);
		match (first, second) {
			('-', 'V') => {
				println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			('-', '-') => {
				let arg: &str = &args.join_long(rest);
				match arg.split_once('=') {
					None => match arg {
						"version" => {
							println!(concat!(":3 v", env!("CARGO_PKG_VERSION")));
							return Ok(());
						},
						_ => panic!(),
					},
					_ => panic!(),
				}
			},
			_ => panic!(),
		}
	}
	Ok(())
}
