use crate::Rng;

// TODO: pub struct ValidInstruction<I>(I);
pub trait Game {
	type Instruction;
	fn enumerate_instructions(&self) -> impl Iterator<Item = Self::Instruction>;
	fn validate_instruction(&self, instruction: Self::Instruction) -> bool;
	fn process_instruction(&mut self, instruction: Self::Instruction);
}

/// An identifier which specifies the deck id, suit, and card value.
/// 2 bits for deck ID
/// 2 bits for suit ID
/// 4 bits for card Value
/// TODO: better encoding for slightly more decks
#[derive(Clone, Debug)]
pub struct Card(u8);
pub struct CardValue(deranged::RangedU8<1, 13>);
pub enum Suit {
	Spades,
	Hearts,
	Clubs,
	Diamonds,
}
impl Card {
	pub fn value(&self) -> CardValue {
		let masked = self.0 & 0b1111;
		let value = unsafe { deranged::RangedU8::new_unchecked(masked) };
		CardValue(value)
	}
	pub fn suit(&self) -> Suit {
		let red = self.is_red();
		let kiki = self.is_kiki();
		match (kiki, red) {
			(false, false) => Suit::Spades,
			(false, true) => Suit::Hearts,
			(true, false) => Suit::Clubs,
			(true, true) => Suit::Diamonds,
		}
	}
	/// Is the suit red.
	pub fn is_red(&self) -> bool {
		self.0 & 0b010000 != 0
	}
	/// Is the suit shape spikey.  (Bouba/kiki)
	pub fn is_kiki(&self) -> bool {
		self.0 & 0b100000 != 0
	}
	pub fn deck(&self) -> u8 {
		self.0 >> 6
	}
}

pub struct Stack(Vec<Card>);
impl Stack {
	/// Generate a full deck of cards with the specified deck id.
	pub fn full_deck(deck_id: u8) -> Stack {
		let mut stack = Vec::with_capacity(52);
		for suit in 0..4 {
			for value in 1..=13 {
				stack.push(Card(deck_id << 6 | suit << 4 | value));
			}
		}
		Stack(stack)
	}
	pub fn shuffle<R: rand::Rng>(&mut self, rng: &mut R) {
		use rand::seq::SliceRandom;
		self.0.shuffle(rng);
	}
}

pub struct Session<G: Game> {
	seed: Rng,
	state: G,
	history: Vec<G::Instruction>,
}
impl<G: Game> Session<G> {
	pub fn new(seed: Rng, state: G) -> Self {
		Self {
			seed,
			state,
			history: Vec::new(),
		}
	}
}
impl<G: Game> Game for Session<G>
where
	G::Instruction: Clone,
{
	type Instruction = G::Instruction;
	fn enumerate_instructions(&self) -> impl Iterator<Item = Self::Instruction> {
		self.state.enumerate_instructions()
	}
	fn validate_instruction(&self, instruction: Self::Instruction) -> bool {
		self.state.validate_instruction(instruction)
	}
	fn process_instruction(&mut self, instruction: Self::Instruction) {
		self.history.push(instruction.clone());
		self.state.process_instruction(instruction);
	}
}
