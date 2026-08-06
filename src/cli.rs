use std::{
	env::{Args, args},
	str::Chars,
};

pub struct ArgIter {
	args: Args,
	cached: Vec<String>,
}

impl ArgIter {
	pub fn new() -> Self {
		let mut instance: Self = Self {
			args: args(),
			cached: Vec::with_capacity(4),
		};
		// Skip executable name.
		let _ = instance.args.next();
		instance
	}

	pub fn restore(&mut self, arg: String) {
		self.cached.push(arg);
	}

	pub fn destructure(arg: String) -> (char, char, String) {
		debug_assert!(!arg.is_empty());
		let mut chars: Chars = arg.chars();
		let first: char = chars.next().unwrap();
		let second: char = chars.next().unwrap();
		let rest: String = chars.as_str().to_owned();
		(first, second, rest)
	}

	pub fn join_long(&mut self, mut arg: String) -> String {
		// Handle space separated long arguments.
		if arg.split_once('=') == None {
			if let Some(next) = self.next() {
				if next.chars().next() == Some('-') {
					// current arg is flag
					self.restore(next);
				} else {
					// pull in the value
					arg = format!("{arg}={next}");
				};
			};
		};
		arg
	}
}

impl Iterator for ArgIter {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self.cached.pop() {
			some @ Some(_) => some,
			None => self.args.next(),
		}
	}
}
