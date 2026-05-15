use crate::Rng;
use crate::card_game::{Card, Game, Pile, Stack};

struct KlondikeConfig {}
struct KlondikeState {
	piles: [Pile; 14],
}
pub enum KlondikePileId {
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
impl std::ops::Index<KlondikePileId> for KlondikeState {
	type Output = Pile;
	fn index(&self, index: KlondikePileId) -> &Self::Output {
		&self.piles[index as usize]
	}
}
impl std::ops::IndexMut<KlondikePileId> for KlondikeState {
	fn index_mut(&mut self, index: KlondikePileId) -> &mut Self::Output {
		&mut self.piles[index as usize]
	}
}

pub struct KlondikeInstruction {
	pub src: KlondikePileId,
	pub dst: KlondikePileId,
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
		let card = self.state[instruction.src].pop().unwrap();
		self.state[instruction.dst].push(card);
	}
}
