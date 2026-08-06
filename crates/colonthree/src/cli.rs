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
	Install {},
	Remove {},
	MarkInstalled {},
	PkgEdit {},
}
