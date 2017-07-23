// dictionary/mod.rs
// UTEM: Universal Text Encoding as Meaning
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

/*pub struct Greetings {
	/// Formal Greeting
	pub hello: &'static str,
	/// Semi-Formal Greeting
	pub hi: &'static str,
	/// Informal Greeting
	pub hey: &'static str,
	/// Slang Greeting
	pub yo: &'static str,
	/// Neutral Greeting
	pub greetings: &'static str,
	/// Weird Greeting
	pub salutations: &'static str,
	/// Eg. good day
	pub i_wish_you_a: &'static str,
}

pub struct Dictionary {
	pub greetings: Greetings,
}*/

pub const GREETING_FORMAL: &'static str = "greeting.formal";
pub const GREETING_SEMIFORMAL: &'static str = "greeting.semiformal";
pub const GREETING_INFORMAL: &'static str = "greeting.informal";
pub const GREETING_SLANG: &'static str = "greeting.slang";
pub const GREETING_NEUTRAL: &'static str = "greeting.neutral";
pub const GREETING_WEIRD: &'static str = "greeting.weird";
pub const GREETING_WISH: &'static str = "greeting.wish";

pub const PLACE_WORLD: &'static str = "place.world";
pub const PLACE_PLANET: &'static str = "place.planet";
pub const PLACE_PLANET_EARTH: &'static str = "place.planet.earth";
pub const PLACE_CONTINENT: &'static str = "place.continent";
pub const PLACE_CONTINENT_AFRICA: &'static str = "place.continent.africa";
pub const PLACE_CONTINENT_ANTARCTICA: &'static str = "place.continent.antarctica";
pub const PLACE_CONTINENT_ASIA: &'static str = "place.continent.asia";
pub const PLACE_CONTINENT_AUSTRALIA: &'static str = "place.continent.australia";
pub const PLACE_CONTINENT_EUROPE: &'static str = "place.continent.europe";
pub const PLACE_CONTINENT_NORTH_AMERICA: &'static str = "place.continent.north_america";
pub const PLACE_CONTINENT_SOUTH_AMERICA: &'static str = "place.continent.south_america";
pub const PLACE_REGION_AMERICA: &'static str = "place.region.america";
pub const PLACE_REGION_OCEANIA: &'static str = "place.region.oceania";

pub const PHRASE_ASK_GOINGS_INFORMAL: &'static str = "phrase.ask.goings.informal";
pub const PHRASE_ASK_GOINGS_FORMAL: &'static str = "phrase.ask.goings.formal";
pub const PHRASE_ASK_GOINGS_SLANG: &'static str = "phrase.ask.goings.slang";
