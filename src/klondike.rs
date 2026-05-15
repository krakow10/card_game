use crate::Rng;
use crate::card_game::{Card, Game, Stack};

struct Pile {
	face_down: Stack,
	face_up: Stack,
}
struct KlondikeConfig {}
struct KlondikeState {
	piles: [Pile; 14],
}
enum KlondikePileId {
	Stock,
	Hand,
	Foundation0,
	Foundation1,
	Foundation2,
	Foundation3,
	Tableau0,
	Tableau1,
	Tableau2,
	Tableau3,
	Tableau4,
	Tableau5,
	Tableau6,
	Tableau7,
}
pub struct KlondikeInstruction {
	src: KlondikePileId,
	dst: KlondikePileId,
}
pub struct Klondike {
	config: KlondikeConfig,
	state: KlondikeState,
}
impl Klondike {
	pub fn new(mut seed: Rng) -> Self {
		let mut deck = Stack::full_deck(0);
		deck.shuffle(&mut seed);
		unimplemented!()
	}
}
impl Game for Klondike {
	type Instruction = KlondikeInstruction;
	fn enumerate_instructions(&self) -> impl Iterator<Item = Self::Instruction> {
		vec![].into_iter()
	}
	fn validate_instruction(&self, instruction: Self::Instruction) -> bool {
		todo!()
	}
	fn process_instruction(&mut self, instruction: Self::Instruction) {
		todo!()
	}
}
