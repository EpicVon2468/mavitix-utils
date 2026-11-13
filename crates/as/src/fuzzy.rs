use std::{cmp::min, hint::cold_path};

pub fn filter_cli_range<'a, 'b>(
	input: &'a str,
	options: &'b [&'b str],
	limit: usize,
) -> Vec<&'b str> {
	let mut dest: Vec<&'b str> = Vec::with_capacity((options.len() + 2) / 2);
	let input: &[u8] = match input.as_bytes() {
		// &[b'-', b'-', ref input @ ..] => input,
		&[b'-', ref input @ ..] => input,
		input => {
			cold_path();
			input
		},
	};
	for option in options {
		let sliced: &[u8] = match option.as_bytes() {
			// &[b'-', b'-', ref option @ ..] => option,
			&[b'-', ref option @ ..] => option,
			option => {
				cold_path();
				option
			},
		};
		if distance(input, sliced) <= limit || sliced.get(0..input.len()) == Some(input) {
			dest.push(option);
		};
	}
	dest
}

// Evil fucking Damerau–Levenshtein Distance
pub fn distance(first: &[u8], second: &[u8]) -> usize {
	let first_len: usize = first.len();
	let second_len: usize = second.len();
	let mut dest: Vec<Vec<usize>> = Vec::with_capacity(first_len + 1);

	macro_rules! initialise {
		($elements:expr $(,)?) => {
			let result: Result<_, _> = dest.push_within_capacity($elements);
			cfg_select! {
				debug_assertions => {
					if result.is_err() {
						unreachable!("Always within capacity; this case should be unreachable!");
					};
				},
				not(debug_assertions) => {
					// SAFETY: Always within capacity here.
					unsafe { result.unwrap_unchecked() };
				},
			};
		};
	}

	initialise!(vec![0; second_len + 1]);
	for j in 1..=second_len {
		dest[0][j] = j;
	}
	for i in 1..=first_len {
		let mut elements: Vec<usize> = vec![0; second_len + 1];
		elements[0] = i;
		initialise!(elements);
	}

	for i in 1..=first_len {
		for j in 1..=second_len {
			let cost: usize = if first[i - 1] == second[j - 1] { 0 } else { 1 };
			dest[i][j] = {
				let mut min: usize = dest[i - 1][j] + 1;
				{
					let value: usize = dest[i][j - 1] + 1;
					if value < min {
						min = value;
					};
				};
				{
					let value: usize = dest[i - 1][j - 1] + cost;
					if value < min {
						min = value;
					};
				};
				min
			};
			if i > 1 && j > 1 && first[i - 1] == second[j - 2] && first[i - 2] == second[j - 1] {
				dest[i][j] = min(dest[i][j], dest[i - 2][j - 2] + cost);
			};
		}
	}

	dest[first_len][second_len]
}
