// Universal Text Encoding as Meaning
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// utemc/src/main.rs

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
