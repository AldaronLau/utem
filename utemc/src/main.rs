// main.rs
// UTEMC: Universal Text Encoding as Meaning Compiler
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

extern crate utem;

use utem::Language;

fn main() {
	println!("				!? Example");
	utem::encode(Language::English, "Hello, world!  What's Up?");
	println!("				. Example");
	utem::encode(Language::English, "Hello, world.");
	println!("				No Punctuation Example");
	utem::encode(Language::English, "Hello, world");

//	println!("{:x}", code);
}
