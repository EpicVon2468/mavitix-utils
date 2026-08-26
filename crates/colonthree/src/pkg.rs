use std::{fs::File, hint::cold_path, io::ErrorKind, path::PathBuf};

use crate::{PKG_DIR, PKG_DIR_LEN};

pub const INFO_DIR: &str = "pkgs";
pub const INFO_DIR_LEN: usize = INFO_DIR.len();

pub const INFO_FILE: &str = "inf";
pub const INFO_FILE_LEN: usize = INFO_FILE.len();

pub struct PkgInfo {
	name: String,
	paths: Vec<String>,
	version: String,
}

impl PkgInfo {
	#[inline]
	pub fn get_name(&self) -> &str {
		&self.name
	}

	#[inline]
	pub fn get_paths(&self) -> &[String] {
		self.paths.as_slice()
	}

	#[inline]
	pub fn get_version(&self) -> &str {
		&self.version
	}
}

impl PkgInfo {
	pub fn query_pkg(name: &str) -> Option<Self> {
		let path: PathBuf = {
			let mut buf: PathBuf = PathBuf::with_capacity(
				// '{PKG_DIR}/'
				PKG_DIR_LEN + 1 +
				// '{INFO_DIR}/'
				INFO_DIR_LEN + 1 +
				// '{name}/'
				name.len() + 1 +
				// '{INFO_FILE}'
				INFO_FILE_LEN,
			);
			buf.push(PKG_DIR);
			buf.push(name);
			buf.push(INFO_FILE);
			buf
		};
		let mut file: File = match File::options().read(true).open(&path) {
			Ok(file) => file,
			Err(error) => {
				match error.kind() {
					ErrorKind::NotFound => return None,
					ErrorKind::IsADirectory
					| ErrorKind::FilesystemLoop
					| ErrorKind::CrossesDevices
					| ErrorKind::TooManyLinks
					| ErrorKind::InvalidFilename
					| ErrorKind::InputOutputError => {
						cold_path();
						unreachable!(":3: package hierarchy is invalid @ {path:?}; {error}");
					},
					_ => {
						cold_path();
						unreachable!(
							":3: unexpected or unreachable error whilst reading {path:?}; {error}",
						);
					},
				};
				todo!();
			},
		};
		todo!();
	}
}
