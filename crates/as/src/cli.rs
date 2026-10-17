use std::{
	env::{Args, args},
	iter::Peekable,
};

#[derive(Debug)]
pub struct CLI {
	pub debug_info: bool,
	pub relax_relocations: bool,
	pub warn: bool,
	pub verbose: bool,
	pub files: Vec<String>,
}

impl Default for CLI {
	fn default() -> Self {
		CLI {
			debug_info: false,
			relax_relocations: false,
			warn: true,
			verbose: false,
			files: Vec::with_capacity(4),
		}
	}
}

pub struct ArgIter {
	args: Peekable<Args>,
	pub seen_double_dash: bool,
}

impl ArgIter {
	pub fn new() -> Self {
		let mut instance: Self = Self {
			args: args().peekable(),
			seen_double_dash: false,
		};
		let _ = instance.args.next();
		instance
	}

	#[inline]
	pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
		self.args.peek()
	}
}

impl Iterator for ArgIter {
	type Item = String;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.args.next()
	}
}
