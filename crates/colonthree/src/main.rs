use anyhow::{Result, bail};

use mavitix_utils::{const_println, main};

pub mod cli;
pub mod cli_parser;
pub mod errors;
pub mod pkg;

use crate::{
	cli::{Install, MarkInstalled, PkgEdit, Remove, Subcommand, UserConfig},
	cli_parser::ArgIter,
	errors::CLIError,
};

main!(main_impl());

pub fn main_impl() -> Result<()> {
	let mut config: UserConfig = UserConfig::default();
	let mut args: ArgIter = ArgIter::new();
	while let Some(arg) = args.next() {
		// Global arguments.
		match {
			let arg: &str = &arg;
			arg
		} {
			"-v" => {
				const_println!(env!("CARGO_PKG_VERSION"));
				return Ok(());
			},
			"--version" => {
				const_println!(concat!(
					env!("CARGO_BIN_NAME"),
					" v",
					env!("CARGO_PKG_VERSION"),
				));
				return Ok(());
			},
			"-h" => {
				todo!();
			},
			"--help" => {
				todo!();
			},
			_ => (),
		};
		let Some(ref mut subcommand): Option<Subcommand> = config.subcommand else {
			cli(&mut config, &mut args, arg)?;
			continue;
		};
		match subcommand {
			&mut Subcommand::Install(ref mut inner) => cli_install(inner, &mut args, arg),
			&mut Subcommand::Remove(ref mut inner) => cli_remove(inner, &mut args, arg),
			&mut Subcommand::MarkInstalled(ref mut inner) =>
				cli_mark_installed(inner, &mut args, arg),
			&mut Subcommand::PkgEdit(ref mut inner) => cli_pkg_edit(inner, &mut args, arg),
		}?;
	}
	Ok(())
}

fn cli(config: &mut UserConfig, args: &mut ArgIter, arg: String) -> Result<()> {
	let (first, second, rest): (char, char, String) = cli_parser::destructure(&arg);
	match (first, second) {
		// ('-', 'V') => {
		// 	const_println!(env!("CARGO_PKG_VERSION"));
		// 	return Ok(());
		// },
		('-', '-') => {
			let arg: &str = &args.join_long(rest);
			match arg.split_once('=') {
				// no '=', therefore a flag
				None => match arg {
					// "version" => {
					// 	const_println!(concat!(
					// 		env!("CARGO_BIN_NAME"),
					// 		" v",
					// 		env!("CARGO_PKG_VERSION"),
					// 	));
					// 	return Ok(());
					// },
					invalid => bail!(CLIError::UnknownArgument {
						arg_name: String::from(invalid),
					}),
				},
				invalid => bail!(CLIError::UnknownArgument {
					// SAFETY: The `None` case is handled above.
					arg_name: unsafe { invalid.unwrap_unchecked() }.0.to_owned(),
				}),
			};
		},
		// After this branch, this function will no longer be called.
		_ =>
			config.subcommand = Some(
				match {
					let arg: &str = &arg;
					arg
				} {
					"install" => Subcommand::Install(Default::default()),
					"remove" => Subcommand::Remove(Default::default()),
					"mark-installed" => Subcommand::MarkInstalled(Default::default()),
					"pkg-edit" => Subcommand::PkgEdit(Default::default()),
					_ => bail!(CLIError::UnknownArgument { arg_name: arg }),
				},
			),
	};
	Ok(())
}

fn cli_install(config: &mut Install, args: &mut ArgIter, arg: String) -> Result<()> {
	let (first, second, rest): (char, char, String) = cli_parser::destructure(&arg);
	match (first, second) {
		// ('-', 'V') => {
		// 	const_println!(env!("CARGO_PKG_VERSION"));
		// 	return Ok(());
		// },
		('-', '-') => {
			let arg: &str = &args.join_long(rest);
			match arg.split_once('=') {
				// no '=', therefore a flag
				None => match arg {
					// "version" => {
					// 	const_println!(concat!(
					// 		env!("CARGO_BIN_NAME"),
					// 		" v",
					// 		env!("CARGO_PKG_VERSION"),
					// 	));
					// 	return Ok(());
					// },
					invalid => bail!(CLIError::UnknownArgument {
						arg_name: String::from(invalid),
					}),
				},
				invalid => bail!(CLIError::UnknownArgument {
					// SAFETY: The `None` case is handled above.
					arg_name: unsafe { invalid.unwrap_unchecked() }.0.to_owned(),
				}),
			};
		},
		_ => bail!(CLIError::UnknownArgument { arg_name: arg }),
	};
	Ok(())
}

fn cli_remove(config: &mut Remove, args: &mut ArgIter, arg: String) -> Result<()> {
	Ok(())
}

fn cli_mark_installed(config: &mut MarkInstalled, args: &mut ArgIter, arg: String) -> Result<()> {
	Ok(())
}

fn cli_pkg_edit(config: &mut PkgEdit, args: &mut ArgIter, arg: String) -> Result<()> {
	Ok(())
}
