pub const ASCIITABLE: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ";
pub const ASCIILIMIT: i32 = ASCIITABLE.len() as i32;

pub fn is_char_in_ascii_table(char: char) -> bool {
	for i in 0..ASCIILIMIT as usize {
		if ASCIITABLE.chars().nth(i).unwrap() == char {
			return true;
		}
	}
	println!("'{char}' is not in ascii table nor is printable");
	false
}

pub fn is_in_ascii_table(text: &str) -> bool {
	for i in 0..text.len() {
		if !is_char_in_ascii_table(text.chars().nth(i).unwrap()) {
			return false;
		}
	}
	true
}

pub fn get_value_ascii_table(char: char) -> u32 {
	for i in 0..ASCIILIMIT as usize {
		if ASCIITABLE.chars().nth(i).unwrap() == char {
			return i as u32;
		}
	}
	0
}
