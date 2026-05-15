use crate::Rng;
use crate::card_game::{CardValue, Game, Pile, Stack};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KlondikeConfig {}
impl Default for KlondikeConfig {
	fn default() -> Self {
		KlondikeConfig {}
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KlondikePileId {
	Tableau1,
	Tableau2,
	Tableau3,
	Tableau4,
	Tableau5,
	Tableau6,
	Tableau7,
	Foundation1,
	Foundation2,
	Foundation3,
	Foundation4,
	Stock,
}
impl KlondikePileId {
	fn next(self) -> Option<Self> {
		use KlondikePileId::*;
		Some(match self {
			Tableau1 => Tableau2,
			Tableau2 => Tableau3,
			Tableau3 => Tableau4,
			Tableau4 => Tableau5,
			Tableau5 => Tableau6,
			Tableau6 => Tableau7,
			Tableau7 => Foundation1,
			Foundation1 => Foundation2,
			Foundation2 => Foundation3,
			Foundation3 => Foundation4,
			Foundation4 => Stock,
			Stock => return None,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KlondikeInstruction {
	pub src: KlondikePileId,
	pub dst: KlondikePileId,
}
impl KlondikeInstruction {
	pub fn stock() -> Self {
		Self {
			src: KlondikePileId::Stock,
			dst: KlondikePileId::Stock,
		}
	}
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

const STOCKS: usize = 1;
const FOUNDATIONS: usize = 4;
const TABLEAUS: usize = 7;
const fn sum(n: usize) -> usize {
	n * (n + 1) / 2
}
const MAX_STACK: usize = 52 - sum(TABLEAUS);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct KlondikeState {
	piles: [Pile<MAX_STACK>; TABLEAUS + FOUNDATIONS + STOCKS],
}
impl KlondikeState {
	fn pile(&self, index: KlondikePileId) -> &Pile<MAX_STACK> {
		&self.piles[index as usize]
	}
	fn pile_mut(&mut self, index: KlondikePileId) -> &mut Pile<MAX_STACK> {
		&mut self.piles[index as usize]
	}
	fn is_instruction_valid(&self, instruction: KlondikeInstruction) -> bool {
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
					KlondikePileId::Foundation1
						| KlondikePileId::Foundation2
						| KlondikePileId::Foundation3
						| KlondikePileId::Foundation4
				) =>
			{
				// get the top cards
				if let Some(src_card) = self.pile(src).face_up().last() {
					match self.pile(dst).face_up().last() {
						// destination card exists
						Some(dst_card) => {
							// suit matches?
							src_card.suit() == dst_card.suit()
							// value is +1?
							&& dst_card.value().checked_add(1) == Some(src_card.value())
						}
						// only ace is allowed to go onto empty foundation
						None => src_card.value() == CardValue::ACE,
					}
				} else {
					false
				}
			}
			// other = move to tableau
			KlondikeInstruction { src, dst } => {
				// get the top cards
				if let Some(src_card) = self.pile(src).face_up().last() {
					match self.pile(dst).face_up().last() {
						// destination card exists
						Some(dst_card) => {
							// red-ness is opposite?
							src_card.is_red() != dst_card.is_red()
							// value is -1?
							&& dst_card.value().checked_sub(1) == Some(src_card.value())
						}
						// only king is allowed to go onto empty tableau
						None => src_card.value() == CardValue::KING,
					}
				} else {
					false
				}
			}
		}
	}
}

pub struct KlondikeIter {
	instruction: Option<KlondikeInstruction>,
}
impl KlondikeIter {
	fn new() -> Self {
		Self {
			instruction: Some(KlondikeInstruction {
				src: KlondikePileId::Tableau1,
				dst: KlondikePileId::Tableau2,
			}),
		}
	}
}
impl Iterator for KlondikeIter {
	type Item = KlondikeInstruction;
	fn next(&mut self) -> Option<Self::Item> {
		let instruction = self.instruction;
		self.instruction = instruction?.next();
		instruction
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Klondike {
	config: KlondikeConfig,
	state: KlondikeState,
}
impl Klondike {
	pub fn new_random_default() -> Self {
		Self::new(Rng::default(), KlondikeConfig::default())
	}
	pub fn new(mut seed: Rng, config: KlondikeConfig) -> Self {
		// shuffle a new deck
		let mut deck = Stack::full_deck(0);
		use rand::seq::SliceRandom;
		deck.shuffle(&mut seed);
		let mut deck = deck.into_iter();

		// generate tableaus
		let [t0, t1, t2, t3, t4, t5, t6] = core::array::from_fn(|i| {
			let stack = arrayvec::ArrayVec::from_iter((&mut deck).take(i)).into();
			let mut pile = Pile::new_face_down(stack);
			pile.push(deck.next().unwrap());
			pile
		});

		// stock is remaining cards
		let stock = Pile::new_face_down(arrayvec::ArrayVec::from_iter(deck).into());

		let state = KlondikeState {
			piles: [
				t0,
				t1,
				t2,
				t3,
				t4,
				t5,
				t6,
				Pile::new(),
				Pile::new(),
				Pile::new(),
				Pile::new(),
				stock,
			],
		};
		Self { config, state }
	}
	#[inline]
	pub fn pile(&self, index: KlondikePileId) -> &Pile<MAX_STACK> {
		self.state.pile(index)
	}
	#[inline]
	fn pile_mut(&mut self, index: KlondikePileId) -> &mut Pile<MAX_STACK> {
		self.state.pile_mut(index)
	}
}

impl Game for Klondike {
	type Instruction = KlondikeInstruction;
	fn possible_instructions(&self) -> impl Iterator<Item = Self::Instruction> + use<> {
		let state = self.state.clone();
		KlondikeIter::new().filter(move |&instruction| state.is_instruction_valid(instruction))
	}
	fn is_instruction_valid(&self, instruction: Self::Instruction) -> bool {
		self.state.is_instruction_valid(instruction)
	}
	fn process_instruction(&mut self, instruction: Self::Instruction) {
		match instruction {
			// Reset the stock if it's empty
			KlondikeInstruction {
				src: KlondikePileId::Stock,
				dst: KlondikePileId::Stock,
			} => {
				if self.pile(KlondikePileId::Stock).face_down().is_empty() {
					self.pile_mut(KlondikePileId::Stock)
						.flip_it_and_reverse_it();
				} else {
					self.pile_mut(KlondikePileId::Stock).flip_up();
				}
			}
			KlondikeInstruction { src, dst } => {
				if let Some(card) = self.pile_mut(src).pop_flip_up() {
					self.pile_mut(dst).push(card);
				} else {
					println!("Attempted to move from an empty src");
					dbg!(instruction);
				}
			}
		}
	}
	fn is_win(&self) -> bool {
		// assuming only valid moves, tableau empty and stock empty means win
		self.state.piles[0..9].iter().all(|pile| pile.is_empty())
	}
}
