use crate::utils::is_in_ascii_table;
use num::integer::gcd;
use std::process::exit;

pub fn find_key_length(text: &str, min_pair_length: usize) -> usize {
	if !is_in_ascii_table(text) {
		exit(1);
	}
	let text_len = text.len();
	let mut distances: Vec<usize> = Vec::new();
	let mut is_a_pair = false;
	let mut first_pair_current_pos = 0;

	while first_pair_current_pos < text_len && min_pair_length + first_pair_current_pos < text_len {
		let mut first_pair =
			&text[first_pair_current_pos..min_pair_length + first_pair_current_pos];

		let mut second_pair_current_pos = first_pair_current_pos + 1;
		while second_pair_current_pos < text_len
			&& min_pair_length + second_pair_current_pos < text_len
		{
			let mut second_pair =
				&text[second_pair_current_pos..min_pair_length + second_pair_current_pos];
			let mut offset = 1;
			while first_pair == second_pair {
				second_pair = &text
					[second_pair_current_pos..min_pair_length + offset + second_pair_current_pos];
				first_pair = &text
					[first_pair_current_pos..min_pair_length + first_pair_current_pos + offset];
				is_a_pair = true;
				offset += 1;
			}
			if is_a_pair {
				offset -= 2;
				first_pair_current_pos += min_pair_length + offset;
				second_pair_current_pos += min_pair_length + offset;
				is_a_pair = false;
				distances.push(second_pair_current_pos - first_pair_current_pos);
			}
			second_pair_current_pos += 1;
		}
		first_pair_current_pos += 1;
	}
	distances.iter().copied().reduce(gcd).unwrap_or(0)
}
