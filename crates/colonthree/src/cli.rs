use std::path::PathBuf;

#[derive(Debug)]
pub struct UserConfig {
	pub subcommand: Option<Subcommand>,
}

impl Default for UserConfig {
	fn default() -> Self {
		Self { subcommand: None }
	}
}

#[derive(Debug)]
pub enum Subcommand {
	Install(Install),
	Remove(Remove),
	MarkInstalled(MarkInstalled),
	PkgEdit(PkgEdit),
}

#[derive(Debug)]
#[derive_const(Default)]
pub enum StdoutMode {
	Verbose,
	#[default]
	Normal,
	Quiet,
	Silent,
}

#[derive(Debug)]
pub struct Install {
	/// Whether to preserve the external environment variables during the build process.
	pub impure: bool,
	/// Paths to extra `.patch` git files to apply before building.
	///
	/// Files matching `/var/lib/colonthree/pkgs/<name>/patches/*.patch` are applied automagically, and should not be edited by users.
	pub patches: Vec<PathBuf>,
	pub stdout_mode: StdoutMode,
}

impl Default for Install {
	fn default() -> Self {
		Self {
			impure: false,
			patches: Default::default(),
			stdout_mode: Default::default(),
		}
	}
}

#[derive(Debug)]
pub struct Remove {}

impl Default for Remove {
	fn default() -> Self {
		Self {}
	}
}

#[derive(Debug)]
pub struct MarkInstalled {}

impl Default for MarkInstalled {
	fn default() -> Self {
		Self {}
	}
}

#[derive(Debug)]
pub struct PkgEdit {}

impl Default for PkgEdit {
	fn default() -> Self {
		Self {}
	}
}
