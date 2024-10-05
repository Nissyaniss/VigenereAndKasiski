use crate::utils::{get_value_ascii_table, is_in_ascii_table, ASCIILIMIT, ASCIITABLE, SPACEASINT};
use std::process::exit;

pub fn encrypt(text: String, key: String) -> String {
	let mut result: String = String::new();

	if !is_in_ascii_table(&key) || !is_in_ascii_table(&text) {
		exit(1);
	}

	for i in 0..text.len() {
		let text_current_char_as_int = get_value_ascii_table(text.chars().nth(i).unwrap());
		let key_current_char_as_int =
			get_value_ascii_table(key.chars().nth(i % key.len()).unwrap());

		if text_current_char_as_int == SPACEASINT {
			result.push(' ');
		} else {
			let mut final_char: i32 =
				text_current_char_as_int as i32 + key_current_char_as_int as i32;

			while final_char > ASCIILIMIT {
				final_char -= ASCIILIMIT;
			}
			if final_char == SPACEASINT as i32 {
				final_char -= 1;
			}
			result.push(ASCIITABLE.chars().nth(final_char as usize).unwrap());
		}
	}

	result
}

pub fn decrypt(text: String, key: String) -> String {
	let mut result: String = String::new();

	if !is_in_ascii_table(&key) || !is_in_ascii_table(&text) {
		exit(1);
	}

	for i in 0..text.len() {
		let text_current_char_as_int = get_value_ascii_table(text.chars().nth(i).unwrap());
		let key_current_char_as_int =
			get_value_ascii_table(key.chars().nth(i % key.len()).unwrap());

		if text_current_char_as_int == SPACEASINT {
			result.push(' ');
		} else {
			let mut final_char: i32 =
				text_current_char_as_int as i32 - key_current_char_as_int as i32;

			while final_char < 0 {
				final_char += ASCIILIMIT;
			}
			if final_char == SPACEASINT as i32 {
				final_char -= 1;
			}
			result.push(ASCIITABLE.chars().nth(final_char as usize).unwrap());
		}
	}
	result
}
