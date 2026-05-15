use crate::Rng;
use crate::card_game::{Card, Game, Pile, Stack};

pub struct KlondikeConfig {}
impl Default for KlondikeConfig {
	fn default() -> Self {
		KlondikeConfig {}
	}
}
struct KlondikeState {
	piles: [Pile; 13],
}
pub enum KlondikePileId {
	Tableau0,
	Tableau1,
	Tableau2,
	Tableau3,
	Tableau4,
	Tableau5,
	Tableau6,
	Tableau7,
	Stock,
	Foundation0,
	Foundation1,
	Foundation2,
	Foundation3,
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
	pub fn new(mut seed: Rng, config: KlondikeConfig) -> Self {
		// shuffle a new deck
		let mut deck = Stack::full_deck(0);
		use rand::seq::SliceRandom;
		deck.shuffle(&mut seed);

		// generate tableaus
		let [t0, t1, t2, t3, t4, t5, t6, t7] = core::array::from_fn(|i| {
			let stack = deck.split_off(i).into();
			let mut pile = Pile::new_face_down(stack);
			pile.push(deck.pop().unwrap());
			pile
		});

		// stock is remaining cards
		let stock = Pile::new_face_down(deck);

		let state = KlondikeState {
			piles: [
				t0,
				t1,
				t2,
				t3,
				t4,
				t5,
				t6,
				t7,
				stock,
				Pile::new(),
				Pile::new(),
				Pile::new(),
				Pile::new(),
			],
		};
		Self { config, state }
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
