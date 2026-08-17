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
pub struct Install {}

impl Default for Install {
	fn default() -> Self {
		Self
	}
}

#[derive(Debug)]
pub struct Remove {}

impl Default for Remove {
	fn default() -> Self {
		Self
	}
}

#[derive(Debug)]
pub struct MarkInstalled {}

impl Default for MarkInstalled {
	fn default() -> Self {
		Self
	}
}

#[derive(Debug)]
pub struct PkgEdit {}

impl Default for PkgEdit {
	fn default() -> Self {
		Self
	}
}
