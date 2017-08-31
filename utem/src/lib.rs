// Universal Text Encoding as Meaning
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/lib.rs

#![doc(
	html_logo_url =
		"https://raw.githubusercontent.com/plopgrizzly/utem/master/res/icon.svg",
	html_favicon_url =
		"https://raw.githubusercontent.com/plopgrizzly/utem/master/res/symbol.svg",
	html_root_url = "http://plopgrizzly.com/utem"
)]

mod dictionary;

mod english;

pub enum Language {
	/// The language that the system uses.
	Default,
	/// Intermediate representation between languages.
	RanSlat,
	English,
	Spanish,
}

// const END: u8 = 0b_0000;
// const NOUN: u8 = 0b_0001;
// const VERB: u8 = 0b_0010;
// const ADJECTIVE: u8 = 0b_0011;
// const ARTICLE: u8 = 0b_0100;

// const END_TEXT: u8 = END & (0b_0000 << 4);
// const END_PARAGRAPH: u8 = END & (0b_0001 << 4);
// const END_SENTENCE: u8 = END & (0b_0010 << 4);
// const END_CLAUSE: u8 = END & (0b_0011 << 4);

// const NOUN_UNDERIVED: u8 = NOUN & (0b_0000 << 4);
// const NOUN_VERB: u8 = NOUN & (0b_0001 << 4); // A Noun Form of A Verb ( *Hiking* is fun from *to hike* )
// const NOUN_ADJECTIVE: u8 = NOUN & (0b_0010 << 4); // A Noun Form of An Adjective ( *Red* is a great color from *red* )

// const VERB_INFINITIVE: u8 = VERB & (0b_0000 << 4);
// const VERB_: u8 = VERB & (0b_0001 << 4);

// const ADJECTIVE_ADNOUN: u8 = ADJECTIVE & (0b_0000 << 4);
// const ADJECTIVE_ADVERB: u8 = ADJECTIVE & (0b_0001 << 4);

// const ARTICLE_INDEFINITE: u8 = ARTICLE & (0b_0000 << 4); // A, An, Some
// const ARTICLE_DEFINITE: u8 = ARTICLE & (0b_0001 << 4); // The

// Independant Clause Types
const IC_STATE: u8 = 0b_0000; // IC ending in .
const IC_ASK: u8 = 0b_0001; // IC ending in ?
const IC_EXCLAIM: u8 = 0b_0010; // IC ending in !
const IC_SURPRISE: u8 = 0b_0011; // IC ending in ‽
const IC_EXCLASK: u8 = 0b_0100; // IC ending in !?
const IC_ASKEXCL: u8 = 0b_0101; // IC ending in ?!
const IC_TRAIL: u8 = 0b_0110; // IC ending in ....
const IC_OTHER: u8 = 0b_0111; // Next Codepoint Tells The Punctuation

const IC_N: u8 = 0b_1000; // IC ending in no punctuation
// const IC_NASK: u8 = 0b_0001; // IC ending in ?s, number defined by next codepoint
// const IC_NEXCLAIM: u8 = 0b_0010; // IC ending in !s, number defined by next codepoint
// const IC_NSURPRISE: u8 = 0b_0011; // IC ending in ‽s, number defined by next codepoint
// const IC_NEXCLASK: u8 = 0b_0100; // IC ending in !?s, number defined by next codepoint
// const IC_NASKEXCL: u8 = 0b_0101; // IC ending in ?!s, number defined by next codepoint
// const IC_NTRAIL: u8 = 0b_0110; // IC ending in ...'s, number defined by next codepoint
// const IC_NOTHER: u8 = 0b_0111; // number defined by next codepoint, codepoint after next tells The punctuation to repeat, 

const IC_C_NORM: u8 = (0b_00 << 6);
const IC_C_QUOTE: u8 = (0b_01 << 6);
const IC_C_PARENTHETICAL: u8 = (0b_10 << 6);
const IC_C_OTHER: u8 = (0b_11 << 6); // Other forms

const IC_SENT: u8 = 0b_00 << 4; // Complete sentence
const IC_WORD: u8 = 0b_01 << 4; // A noun phrase
const IC_COFF: u8 = 0b_10 << 4; // Cut off sentence
// const IC_SENT_QUOTE: u8 = IC_C_QUOTE & IC_SENT; // Complete sentence
// const IC_WORD_QUOTE: u8 = IC_C_QUOTE & IC_WORD; // A noun phrase
// const IC_COFF_QUOTE: u8 = IC_C_QUOTE & IC_COFF; // Cut off sentence
// const IC_SENT_PARENTHETICAL: u8 = IC_C_PARENTHETICAL & IC_SENT; // Complete sentence
// const IC_WORD_PARENTHETICAL: u8 = IC_C_PARENTHETICAL & IC_WORD; // A noun phrase
// const IC_COFF_PARENTHETICAL: u8 = IC_C_PARENTHETICAL & IC_COFF; // Cut off sentence

fn unwrap<T>(a: Option<T>) -> T {
	if let Some(r) = a { r }
	else { panic!("Unexpected end of UTEM text.") }
}

/// Decode UTEM as UTF8 using the specified language.
pub fn decode(_/*language*/: Language, utem_text: &[u8]) -> String {
	let mut text_it = utem_text.iter();
	let mut string = String::new();

	loop {
		let ic_type = unwrap(text_it.next());

		match ic_type & 0b_11_00_0000 {
			IC_C_NORM => string.push('x'),
			IC_C_QUOTE => string.push('"'),
			IC_C_PARENTHETICAL => string.push('('),
			IC_C_OTHER => string.push('_'), // placeholder
			_ => unreachable!(),
		}
		match ic_type & 0b_00_11_0000 {
			IC_SENT => string.push(';'),
			IC_WORD => string.push('w'),
			IC_COFF => string.push('-'),
			_ => panic!("invalid UTEM"),
		}
		match ic_type & 0b_00_00_1000 {
			IC_N => string.push('#'),
			_ => string.push('1'),
		}
		match ic_type & 0b_00_00_0111 {
			IC_STATE => string.push('.'),
			IC_ASK => string.push('?'),
			IC_EXCLAIM => string.push('!'),
			IC_SURPRISE => string.push('‽'),
			IC_EXCLASK => string.push('/'), // !?
			IC_ASKEXCL => string.push('\\'), // ?!
			IC_TRAIL => string.push('…'), // ...
			IC_OTHER => string.push('_'), // placeholder
			_ => unreachable!(),
		}
	}
}

/// Encode UTF-8 Text as UTEM Text
pub fn encode(language: Language, utf8_text: &str) -> Vec<u8> {
	match language {
		Language::English => english::encode(utf8_text),
		_ => panic!("Not Supported Yet")
	}
}

/// Translate ran-slat into a language.
pub fn translate(language: Language, ranslat: &str) -> String {
	match language {
		Language::English => english::translate(ranslat),
		_ => panic!("Language not supported yet")
	}
}
