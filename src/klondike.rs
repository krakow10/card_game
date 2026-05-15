use crate::Rng;
use crate::card_game::{Card, Game, Pile, Stack};

pub struct KlondikeConfig {}
impl Default for KlondikeConfig {
	fn default() -> Self {
		KlondikeConfig {}
	}
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KlondikePileId {
	Stock,
	Tableau0,
	Tableau1,
	Tableau2,
	Tableau3,
	Tableau4,
	Tableau5,
	Tableau6,
	Tableau7,
	Foundation0,
	Foundation1,
	Foundation2,
	Foundation3,
}
impl KlondikePileId {
	fn next(self) -> Option<Self> {
		use KlondikePileId::*;
		Some(match self {
			Stock => Tableau0,
			Tableau0 => Tableau1,
			Tableau1 => Tableau2,
			Tableau2 => Tableau3,
			Tableau3 => Tableau4,
			Tableau4 => Tableau5,
			Tableau5 => Tableau6,
			Tableau6 => Tableau7,
			Tableau7 => Foundation0,
			Foundation0 => Foundation1,
			Foundation1 => Foundation2,
			Foundation2 => Foundation3,
			Foundation3 => return None,
		})
	}
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KlondikeInstruction {
	pub src: KlondikePileId,
	pub dst: KlondikePileId,
}
impl KlondikeInstruction {
	fn next(self) -> Option<Self> {
		let KlondikeInstruction { src, dst } = self;
		if let Some(next_dst) = dst.next() {
			return Some(Self { src, dst: next_dst });
		}
		if let Some(next_src) = src.next() {
			return Some(Self {
				src: next_src,
				dst: KlondikePileId::Stock,
			});
		}
		None
	}
}

#[derive(Clone, Hash)]
struct KlondikeState {
	piles: [Pile; 13],
}
impl KlondikeState {
	fn pile(&self, index: KlondikePileId) -> &Pile {
		&self.piles[index as usize]
	}
	fn pile_mut(&mut self, index: KlondikePileId) -> &mut Pile {
		&mut self.piles[index as usize]
	}
	fn validate_instruction(&self, instruction: KlondikeInstruction) -> bool {
		match instruction {
			// Stock -> Stock draws a card or resets the stock
			KlondikeInstruction {
				src: KlondikePileId::Stock,
				dst: KlondikePileId::Stock,
			} => {
				// cannot move stock when stock is empty
				!self.pile(KlondikePileId::Stock).is_empty()
			}

			// cannot move cards to stock
			KlondikeInstruction {
				src: _,
				dst: KlondikePileId::Stock,
			} => false,

			// moving to foundation has special rules
			KlondikeInstruction { src, dst }
				if matches!(
					dst,
					KlondikePileId::Foundation0
						| KlondikePileId::Foundation1
						| KlondikePileId::Foundation2
						| KlondikePileId::Foundation3
				) =>
			{
				// get the top cards
				if let Some(src_card) = self.pile(src).face_up().last()
					&& let Some(dst_card) = self.pile(dst).face_up().last()
					// suit matches?
					&& src_card.suit() == dst_card.suit()
					// value is +1?
					&& dst_card.value().checked_add(1) == Some(src_card.value())
				{
					true
				} else {
					false
				}
			}
			// other = move to tableau
			KlondikeInstruction { src, dst } => {
				// get the top cards
				if let Some(src_card) = self.pile(src).face_up().last()
					&& let Some(dst_card) = self.pile(dst).face_up().last()
					// red-ness is opposite?
					&& src_card.is_red() != dst_card.is_red()
					// value is -1?
					&& dst_card.value().checked_sub(1) == Some(src_card.value())
				{
					true
				} else {
					false
				}
			}
		}
	}
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
	#[inline]
	pub fn pile(&self, index: KlondikePileId) -> &Pile {
		self.state.pile(index)
	}
	#[inline]
	fn pile_mut(&mut self, index: KlondikePileId) -> &mut Pile {
		self.state.pile_mut(index)
	}
}
pub struct KlondikeIter {
	instruction: Option<KlondikeInstruction>,
}
impl KlondikeIter {
	fn new() -> Self {
		Self {
			instruction: Some(KlondikeInstruction {
				src: KlondikePileId::Stock,
				dst: KlondikePileId::Stock,
			}),
		}
	}
}
impl Iterator for KlondikeIter {
	type Item = KlondikeInstruction;
	fn next(&mut self) -> Option<Self::Item> {
		self.instruction = self.instruction?.next();
		self.instruction
	}
}
impl Game for Klondike {
	type Instruction = KlondikeInstruction;
	fn possible_instructions(&self) -> impl Iterator<Item = Self::Instruction> + use<> {
		let state = self.state.clone();
		KlondikeIter::new().filter(move |&instruction| state.validate_instruction(instruction))
	}
	fn validate_instruction(&self, instruction: Self::Instruction) -> bool {
		self.state.validate_instruction(instruction)
	}
	fn process_instruction(&mut self, instruction: Self::Instruction) {
		let card = self.pile_mut(instruction.src).pop().unwrap();
		self.pile_mut(instruction.dst).push(card);
	}
	fn is_win(&self) -> bool {
		// assuming only valid moves, tableau empty and stock empty means win
		self.state.piles[0..9].iter().all(|pile| pile.is_empty())
	}
}
