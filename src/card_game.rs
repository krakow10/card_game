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
pub struct Card(u8);
pub struct CardValue(deranged::RangedU8<1, 13>);
pub enum Suit {
	Spades,
	Hearts,
	Clubs,
	Diamonds,
}

pub struct Stack(Vec<Card>);

pub struct Session<G: Game> {
	state: G,
	history: Vec<G::Instruction>,
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
