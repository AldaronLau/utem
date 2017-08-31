// Universal Text Encoding as Meaning
// Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE
//
// src/english/dictionary.rs

use std::collections::HashMap;

use dictionary::*;

// TODO: NOT NEEDED ANYMORE
// All the meanings for a word.
// const WORD_HELLO: &'static [&'static str] = &[GREETING_FORMAL];
// const WORD_HI: &'static [&'static str] = &[GREETING_SEMIFORMAL];
// const WORD_HEY: &'static [&'static str] = &[GREETING_INFORMAL];
// const WORD_YO: &'static [&'static str] = &[GREETING_SLANG];
// const WORD_GREETINGS: &'static [&'static str] = &[GREETING_NEUTRAL];
// const WORD_SALUTATIONS: &'static [&'static str] = &[GREETING_WEIRD];

// Generate a dictionary to english.
#[inline(always)]
pub(super) fn to() -> HashMap<&'static str, &'static str> {
	let mut dict = HashMap::new();

	// Greetings
	dict.insert(GREETING_FORMAL, "hello");
	dict.insert(GREETING_SEMIFORMAL, "hi");
	dict.insert(GREETING_INFORMAL, "hey");
	dict.insert(GREETING_SLANG, "yo");
	dict.insert(GREETING_NEUTRAL, "greetings");
	dict.insert(GREETING_WEIRD, "salutations");
	dict.insert(GREETING_WISH, "I wish you a");

	// Places
	dict.insert(PLACE_WORLD, "world");
	dict.insert(PLACE_PLANET, "planet");
	dict.insert(PLACE_PLANET_EARTH, "earth");
	dict.insert(PLACE_CONTINENT, "continent");
	dict.insert(PLACE_CONTINENT_AFRICA, "Africa");
	dict.insert(PLACE_CONTINENT_ANTARCTICA, "Antarctica");
	dict.insert(PLACE_CONTINENT_ASIA, "Asia");
	dict.insert(PLACE_CONTINENT_AUSTRALIA, "Australia");
	dict.insert(PLACE_CONTINENT_EUROPE, "Europe");
	dict.insert(PLACE_CONTINENT_NORTH_AMERICA, "North America");
	dict.insert(PLACE_CONTINENT_SOUTH_AMERICA, "South America");
	dict.insert(PLACE_REGION_AMERICA, "America");
	dict.insert(PLACE_REGION_OCEANIA, "Oceania");

	// Phrases
	dict.insert(PHRASE_ASK_GOINGS_SLANG, "'sup");
	dict.insert(PHRASE_ASK_GOINGS_FORMAL, "what's been going on with you");
	dict.insert(PHRASE_ASK_GOINGS_INFORMAL, "what's up");

	dict
}

// Generate a dictionary from english.
#[inline(always)]
pub(super) fn from() -> HashMap<String, Vec<&'static str>> {
	let dict_in = to();
	let mut dict_out : HashMap<String, Vec<&'static str>> = HashMap::new();

	for (ranslat, english) in dict_in {
		if let Some(a) = dict_out.get_mut(english) {
			a.push(ranslat);
			continue;
		}

		dict_out.insert(english.to_string(), vec![ranslat]);
	}

	// Greetings
/*	dict.insert("hello".to_string(), WORD_HELLO);
	dict.insert("hi".to_string(), WORD_HI);
	dict.insert("hey".to_string(), WORD_HEY);
	dict.insert("yo".to_string(), WORD_YO);
	dict.insert("greetings".to_string(), WORD_GREETINGS);
	dict.insert("salutations".to_string(), WORD_SALUTATIONS);
//	dict.insert("I wish you a", GREETING_WISH); // TODO: Phrasses*/

	dict_out
}
