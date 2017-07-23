# UTEM: Universal Text Encoding as Meaning

## RanSlat Format
This is an intermediate format between a language and bytecode

### Basic Syntax
Each line of RanSlat is a sentence in spoken language.

	!(place.world) greeting.formal

Is how you'd say "Hello, world!".  The first character on the line is the type
of sentence:
* `!` - Statement
* `?` - Question
* `.` - Phrase
* `_` - Unwritten ( Technically not a sentence )

In this situation there's more to our sentence.  There's a vocative address.
We're addressing the world to say hello, so a sentence that addresses something
looks like this:

	.(`NOUN`)

Next, the actual meaning of the sentence.

	greeting.formal

That means it's a type of `greeting` and the type is `formal`.  In English, this
is Hello.

### Adjectives

Adjectives always glob onto nouns:

	_place.world-size.small

"Small world"

	_place.world-size.small-þ

"The small world" - This adjective implies there's only one small world.

	_place.world-size.small-α

"A small world" - This adjective implies there are more than one small world.

	_place.world-size.small-12

"12 small worlds" - Numbers are adjectives, too.

	_place.world-size.small-ß

"Some small worlds"

### Conjunctions

We use the logical operators for this.

	_place.continent.america ∧ _place.world-þ

"America and the world"

	_place.continent.america ∨ _place.world-þ

"America or the world"

	_place.continent.america ⊕ _place.world-þ

"Either America or the world"

	_place.continent.america ∵ _place.world-þ

"America because the world"

	_place.continent.america → _place.world-þ

"America, then the world"

	_place.continent.america ⇒ _place.world-þ

"America, so the world"

## UTEM Format
This is the bytecode.

	0x07 0xE3 0xAA 'U' 'T' 'E' 'M' 0x00 // 8 byte header
	{} // Independant Clause type.
	____ // 
