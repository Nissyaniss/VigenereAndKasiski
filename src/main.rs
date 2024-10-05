use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf, time::Instant};
mod kasiski;
mod utils;
mod viginere;

#[derive(Debug, Parser)]
#[command(author, version, about)]
#[allow(clippy::struct_excessive_bools)]
struct Args {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
	EncryptVigenere {
		text: String,
		key: String,
	},
	DecryptVigenere {
		text: String,
		key: String,
	},
	Kasiski {
		file: PathBuf,
		min_pair_length: usize,
	},
}
fn main() {
	let args = Args::parse();
	let now = Instant::now();
	match args.command {
		Command::EncryptVigenere { text, key } => {
			let encrypted = viginere::encrypt(text, key.clone());
			println!("Your encrypted string using the key {key} is : {encrypted}");
		}

		Command::DecryptVigenere { text, key } => {
			let decrypted = viginere::decrypt(text, key.clone());
			println!("Your decrypted string using the key {key} is : {decrypted}");
		}

		Command::Kasiski {
			file,
			min_pair_length,
		} => {
			let text = fs::read_to_string(file).expect("Unable to read file");
			let result = kasiski::find_key_length(&text, min_pair_length);
			if result == 1 || result == 0 || result == 2 {
				println!("The key is either too short for practical use or my calculations are just wrong.
					I can recommend using another minimal key length because more lengthy texts tend to need more for the length of the key to be found.");
			} else {
				println!("The key length of the provided text is {result}");
			}
		}
	}
	let elapsed_time = now.elapsed();
	println!(
		"The finding took {}.{}{}s ",
		elapsed_time.as_secs(),
		elapsed_time.as_millis() % 1000,
		elapsed_time.as_micros() % 1000,
	);
}
