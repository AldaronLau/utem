// english/mod.rs
// UTEM: Universal Text Encoding as Meaning
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

mod greetings;
mod dictionary;

use std::collections::HashMap;

use dictionary::*;

fn is_ender(c: char) -> bool {
	match c {
		'.' | '!' | '?' | '‽' => true,
		_ => false,
	}
}

fn unphrase(_/*dict*/: &HashMap<String, Vec<&'static str>>, string: &str) -> String {
	let out = string.to_string();

	let out = out.replace("'sup", PHRASE_ASK_GOINGS_SLANG);
	let out = out.replace("what's been going on with you", PHRASE_ASK_GOINGS_FORMAL);
	let out = out.replace("what's up", PHRASE_ASK_GOINGS_INFORMAL);

	out
}

fn reformat(dict: &HashMap<String, Vec<&'static str>>, string: &str)
	-> String
{
	let mut out = String::new();
	let unphrase = unphrase(dict, &string.to_lowercase());
	let a : Vec<_> = unphrase.split(|x| x == ' ' || x == ',').collect();

	for i in a {
		if i.is_empty() == false {
			let i = i.to_lowercase();
			let i = if let Some(i) = dict.get(&i) {
				i[0]
			} else {
				&i
//				panic!("no word \"{}\" in the dictionary", i)
			};

//			println!("i {}", i);
			out.push_str(&i);
//			out.push_str(i);
			out.push(' ');
		}
	}

	out
}

/*pub fn decode(utem_text: &[u8]) -> String {
	String::new()
}*/

pub fn encode(utf8_text: &str) -> Vec<u8> {
	let dict = dictionary::from();
	let utem = Vec::new();
//	let mut last = 0;
	let mut index = 0;
	let group: Vec<_> = utf8_text.split(is_ender).collect();

	for i in utf8_text.match_indices(is_ender) {
		let reformatted = reformat(&dict, group[index]);

		match i.1 {
			"." => println!(". {}", reformatted),
			"!" => println!("! {}", reformatted),
			"?" => println!("? {}", reformatted),
			"‽" => println!("‽ {}", reformatted),
			_ => unreachable!(),
		}

//		last = i.0;
		index += 1;
	}

	let end = reformat(&dict, group[index]);
	if end.is_empty() == false {
		println!("_ {}", end);
	}
	println!();

	utem
}

pub fn translate(ranslat: &str) -> String {
	// Direct include syntax: %this text is not translated.
	if ranslat.starts_with("%") {
		return ranslat[1..].to_string();
	}

	ranslat.to_string()
}
