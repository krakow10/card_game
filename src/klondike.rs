use crate::Rng;
use crate::card_game::{CardValue, Game, Pile, Stack};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KlondikeConfig {}
impl Default for KlondikeConfig {
	fn default() -> Self {
		KlondikeConfig {}
	}
}

#[repr(u8)]
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
	const fn next(self) -> Option<Self> {
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

/// high four bits for stack depth, low four bits for Pile Id
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InstructionSrc(u8);
impl InstructionSrc {
	const STOCK: Self = InstructionSrc::new(KlondikePileStack::Stock);
	pub const fn new(src: KlondikePileStack) -> Self {
		match src {
			KlondikePileStack::Tableau1(skip_cards) => {
				Self(KlondikePileId::Tableau1 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Tableau2(skip_cards) => {
				Self(KlondikePileId::Tableau2 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Tableau3(skip_cards) => {
				Self(KlondikePileId::Tableau3 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Tableau4(skip_cards) => {
				Self(KlondikePileId::Tableau4 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Tableau5(skip_cards) => {
				Self(KlondikePileId::Tableau5 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Tableau6(skip_cards) => {
				Self(KlondikePileId::Tableau6 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Tableau7(skip_cards) => {
				Self(KlondikePileId::Tableau7 as u8 + ((skip_cards as u8) << 4))
			}
			KlondikePileStack::Foundation1 => Self(KlondikePileId::Foundation1 as u8),
			KlondikePileStack::Foundation2 => Self(KlondikePileId::Foundation2 as u8),
			KlondikePileStack::Foundation3 => Self(KlondikePileId::Foundation3 as u8),
			KlondikePileStack::Foundation4 => Self(KlondikePileId::Foundation4 as u8),
			KlondikePileStack::Stock => Self(KlondikePileId::Stock as u8),
		}
	}
	const fn into_spec(self) -> KlondikePileStack {
		// SAFETY: there is no way to construct an invalid InstructionSrc
		let pile = unsafe { core::mem::transmute(self.0 & 0b1111) };
		match pile {
			KlondikePileId::Tableau1 => {
				KlondikePileStack::Tableau1(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Tableau2 => {
				KlondikePileStack::Tableau2(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Tableau3 => {
				KlondikePileStack::Tableau3(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Tableau4 => {
				KlondikePileStack::Tableau4(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Tableau5 => {
				KlondikePileStack::Tableau5(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Tableau6 => {
				KlondikePileStack::Tableau6(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Tableau7 => {
				KlondikePileStack::Tableau7(unsafe { core::mem::transmute(self.0 >> 4) })
			}
			KlondikePileId::Foundation1 => KlondikePileStack::Foundation1,
			KlondikePileId::Foundation2 => KlondikePileStack::Foundation2,
			KlondikePileId::Foundation3 => KlondikePileStack::Foundation3,
			KlondikePileId::Foundation4 => KlondikePileStack::Foundation4,
			KlondikePileId::Stock => KlondikePileStack::Stock,
		}
	}
	const fn next(self) -> Option<Self> {
		match self.into_spec().next() {
			Some(s) => Some(Self::new(s)),
			None => None,
		}
	}
}
impl From<InstructionSrc> for KlondikePileId {
	fn from(value: InstructionSrc) -> Self {
		match value.into_spec() {
			KlondikePileStack::Tableau1(_) => KlondikePileId::Tableau1,
			KlondikePileStack::Tableau2(_) => KlondikePileId::Tableau2,
			KlondikePileStack::Tableau3(_) => KlondikePileId::Tableau3,
			KlondikePileStack::Tableau4(_) => KlondikePileId::Tableau4,
			KlondikePileStack::Tableau5(_) => KlondikePileId::Tableau5,
			KlondikePileStack::Tableau6(_) => KlondikePileId::Tableau6,
			KlondikePileStack::Tableau7(_) => KlondikePileId::Tableau7,
			KlondikePileStack::Foundation1 => KlondikePileId::Foundation1,
			KlondikePileStack::Foundation2 => KlondikePileId::Foundation2,
			KlondikePileStack::Foundation3 => KlondikePileId::Foundation3,
			KlondikePileStack::Foundation4 => KlondikePileId::Foundation4,
			KlondikePileStack::Stock => KlondikePileId::Stock,
		}
	}
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SkipCards {
	Zero,
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Eleven,
	Twelve,
	Thirteen,
}
impl SkipCards {
	const fn next(self) -> Option<Self> {
		use SkipCards::*;
		Some(match self {
			Zero => One,
			One => Two,
			Two => Three,
			Three => Four,
			Four => Five,
			Five => Six,
			Six => Seven,
			Seven => Eight,
			Eight => Nine,
			Nine => Ten,
			Ten => Eleven,
			Eleven => Twelve,
			Twelve => Thirteen,
			Thirteen => return None,
		})
	}
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KlondikePileStack {
	Tableau1(SkipCards),
	Tableau2(SkipCards),
	Tableau3(SkipCards),
	Tableau4(SkipCards),
	Tableau5(SkipCards),
	Tableau6(SkipCards),
	Tableau7(SkipCards),
	Foundation1,
	Foundation2,
	Foundation3,
	Foundation4,
	Stock,
}
impl KlondikePileStack {
	const fn next(self) -> Option<Self> {
		use KlondikePileStack::*;
		Some(match self {
			Tableau1(skip) => match skip.next() {
				Some(next) => Tableau1(next),
				None => Tableau2(SkipCards::Zero),
			},
			Tableau2(skip) => match skip.next() {
				Some(next) => Tableau2(next),
				None => Tableau3(SkipCards::Zero),
			},
			Tableau3(skip) => match skip.next() {
				Some(next) => Tableau3(next),
				None => Tableau4(SkipCards::Zero),
			},
			Tableau4(skip) => match skip.next() {
				Some(next) => Tableau4(next),
				None => Tableau5(SkipCards::Zero),
			},
			Tableau5(skip) => match skip.next() {
				Some(next) => Tableau5(next),
				None => Tableau6(SkipCards::Zero),
			},
			Tableau6(skip) => match skip.next() {
				Some(next) => Tableau6(next),
				None => Tableau7(SkipCards::Zero),
			},
			Tableau7(skip) => match skip.next() {
				Some(next) => Tableau7(next),
				None => Foundation1,
			},
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
	pub src: InstructionSrc,
	pub dst: KlondikePileId,
}
impl KlondikeInstruction {
	pub const fn stock() -> Self {
		Self {
			src: InstructionSrc::STOCK,
			dst: KlondikePileId::Stock,
		}
	}
	const fn next(self) -> Option<Self> {
		let KlondikeInstruction { src, dst } = self;
		if let Some(next_dst) = dst.next() {
			return Some(Self { src, dst: next_dst });
		}
		if let Some(next_src) = src.next() {
			return Some(Self {
				src: next_src,
				dst: KlondikePileId::Tableau1,
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
				src: InstructionSrc::STOCK,
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
				if let Some(src_card) = self.pile(src.into()).face_up().last() {
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
				if let Some(src_card) = self.pile(src.into()).face_up().last() {
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
	const fn new() -> Self {
		Self {
			instruction: Some(KlondikeInstruction {
				src: InstructionSrc::new(KlondikePileStack::Tableau1(SkipCards::Zero)),
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
				src: InstructionSrc::STOCK,
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
				if let Some(card) = self.pile_mut(src.into()).pop_flip_up() {
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
