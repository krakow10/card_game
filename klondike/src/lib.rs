pub type Rng = rand::rngs::StdRng;

use card_game::{Card, Game, Pile, Rank, Stack};

// test readme
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DrawStockConfig {
	#[default]
	DrawOne = 1,
	DrawThree = 3,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MoveFromFoundationConfig {
	#[default]
	Allowed,
	Disallowed,
}

#[derive(Clone, Copy, Debug)]
pub struct ScoringConfig {
	pub move_to_foundation: i32,
	pub flip_up_bonus: i32,
	pub move_to_tableau: i32,
	pub move_from_foundation: i32,
	pub recycle: i32,
}
impl ScoringConfig {
	pub const DEFAULT: Self = Self {
		move_to_foundation: 10,
		flip_up_bonus: 5,
		move_to_tableau: 5,
		move_from_foundation: -15,
		recycle: 0,
	};
}
impl Default for ScoringConfig {
	fn default() -> Self {
		Self::DEFAULT
	}
}

#[derive(Clone, Debug, Default)]
pub struct KlondikeConfig {
	pub draw_stock: DrawStockConfig,
	pub move_from_foundation: MoveFromFoundationConfig,
	pub scoring: ScoringConfig,
}

#[derive(Clone, Debug, Default)]
pub struct KlondikeStats {
	moves: u32,
	move_to_foundation_count: u32,
	flip_up_bonus_count: u32,
	move_to_tableau_count: u32,
	move_from_foundation_count: u32,
	recycle_count: u32,
}
impl KlondikeStats {
	pub const fn new() -> Self {
		KlondikeStats {
			moves: 0,
			move_to_foundation_count: 0,
			flip_up_bonus_count: 0,
			move_to_tableau_count: 0,
			move_from_foundation_count: 0,
			recycle_count: 0,
		}
	}
	pub const fn score(&self, config: &ScoringConfig) -> i32 {
		self.move_to_foundation_count as i32 * config.move_to_foundation
			+ self.flip_up_bonus_count as i32 * config.flip_up_bonus
			+ self.move_to_tableau_count as i32 * config.move_to_tableau
			+ self.move_from_foundation_count as i32 * config.move_from_foundation
			+ self.recycle_count as i32 * config.recycle
	}
	pub const fn moves(&self) -> u32 {
		self.moves
	}
	pub const fn move_to_foundation_count(&self) -> u32 {
		self.move_to_foundation_count
	}
	pub const fn flip_up_bonus_count(&self) -> u32 {
		self.flip_up_bonus_count
	}
	pub const fn move_to_tableau_count(&self) -> u32 {
		self.move_to_tableau_count
	}
	pub const fn move_from_foundation_count(&self) -> u32 {
		self.move_from_foundation_count
	}
	pub const fn recycle_count(&self) -> u32 {
		self.recycle_count
	}
	/// A card was moved to a foundation.
	const fn increment_move_to_foundation(&mut self) {
		self.move_to_foundation_count += 1;
	}
	/// A card on the tableau was flipped up.
	const fn increment_flip_up_bonus(&mut self) {
		self.flip_up_bonus_count += 1;
	}
	/// A card was moved from stock to tableau.
	const fn increment_move_to_tableau(&mut self) {
		self.move_to_tableau_count += 1;
	}
	/// A card was moved from foundation to tableau.
	const fn increment_move_from_foundation(&mut self) {
		self.move_from_foundation_count += 1;
	}
	const fn increment_recycle_count(&mut self) {
		self.recycle_count += 1;
	}
	const fn increment_moves(&mut self) {
		self.moves += 1;
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Tableau {
	Tableau1,
	Tableau2,
	Tableau3,
	Tableau4,
	Tableau5,
	Tableau6,
	Tableau7,
}
impl Tableau {
	const ITER_BEGIN: Self = Self::Tableau1;
	const fn next(self) -> Option<Self> {
		use Tableau::*;
		Some(match self {
			Tableau1 => Tableau2,
			Tableau2 => Tableau3,
			Tableau3 => Tableau4,
			Tableau4 => Tableau5,
			Tableau5 => Tableau6,
			Tableau6 => Tableau7,
			Tableau7 => return None,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Foundation {
	Foundation1,
	Foundation2,
	Foundation3,
	Foundation4,
}
impl Foundation {
	const ITER_BEGIN: Self = Self::Foundation1;
	const fn next(self) -> Option<Self> {
		use Foundation::*;
		Some(match self {
			Foundation1 => Foundation2,
			Foundation2 => Foundation3,
			Foundation3 => Foundation4,
			Foundation4 => return None,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KlondikePile {
	Tableau(Tableau),
	Stock,
	Foundation(Foundation),
}
impl KlondikePile {
	const ITER_BEGIN: Self = Self::Tableau(Tableau::ITER_BEGIN);
	const fn next(self) -> Option<Self> {
		Some(match self {
			Self::Tableau(tableau_stack) => match tableau_stack.next() {
				Some(tableau_stack) => Self::Tableau(tableau_stack),
				None => Self::Stock,
			},
			Self::Stock => Self::Foundation(Foundation::ITER_BEGIN),
			Self::Foundation(foundation) => match foundation.next() {
				Some(foundation) => Self::Foundation(foundation),
				None => return None,
			},
		})
	}
}
impl From<Tableau> for KlondikePile {
	fn from(value: Tableau) -> Self {
		KlondikePile::Tableau(value)
	}
}
impl From<Foundation> for KlondikePile {
	fn from(value: Foundation) -> Self {
		KlondikePile::Foundation(value)
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SkipCards {
	Skip0,
	Skip1,
	Skip2,
	Skip3,
	Skip4,
	Skip5,
	Skip6,
	Skip7,
	Skip8,
	Skip9,
	Skip10,
	Skip11,
	Skip12,
}
impl SkipCards {
	const ITER_BEGIN: Self = Self::Skip0;
	const fn next(self) -> Option<Self> {
		use SkipCards::*;
		Some(match self {
			Skip0 => Skip1,
			Skip1 => Skip2,
			Skip2 => Skip3,
			Skip3 => Skip4,
			Skip4 => Skip5,
			Skip5 => Skip6,
			Skip6 => Skip7,
			Skip7 => Skip8,
			Skip8 => Skip9,
			Skip9 => Skip10,
			Skip10 => Skip11,
			Skip11 => Skip12,
			Skip12 => return None,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TableauStack {
	pub tableau: Tableau,
	pub skip_cards: SkipCards,
}

impl TableauStack {
	const ITER_BEGIN: Self = Self {
		tableau: Tableau::ITER_BEGIN,
		skip_cards: SkipCards::ITER_BEGIN,
	};
	const fn next(self) -> Option<Self> {
		let TableauStack {
			tableau,
			skip_cards,
		} = self;
		if let Some(skip_cards) = skip_cards.next() {
			return Some(Self {
				tableau,
				skip_cards,
			});
		}
		if let Some(tableau) = tableau.next() {
			let skip_cards = SkipCards::ITER_BEGIN;
			return Some(Self {
				tableau,
				skip_cards,
			});
		}
		None
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KlondikePileStack {
	Tableau(TableauStack),
	Stock,
	Foundation(Foundation),
}
impl KlondikePileStack {
	const ITER_BEGIN: Self = Self::Tableau(TableauStack::ITER_BEGIN);
	const fn next(self) -> Option<Self> {
		Some(match self {
			Self::Tableau(tableau_stack) => match tableau_stack.next() {
				Some(tableau_stack) => Self::Tableau(tableau_stack),
				None => Self::Stock,
			},
			Self::Stock => Self::Foundation(Foundation::ITER_BEGIN),
			Self::Foundation(foundation) => match foundation.next() {
				Some(foundation) => Self::Foundation(foundation),
				None => return None,
			},
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DstFoundation {
	pub src: KlondikePile,
	pub foundation: Foundation,
}
impl DstFoundation {
	const ITER_BEGIN: Self = Self {
		src: KlondikePile::ITER_BEGIN,
		foundation: Foundation::ITER_BEGIN,
	};
	const fn next(self) -> Option<Self> {
		let DstFoundation { src, foundation } = self;
		if let Some(src) = src.next() {
			return Some(Self { src, foundation });
		}
		if let Some(foundation) = foundation.next() {
			let src = KlondikePile::ITER_BEGIN;
			return Some(Self { src, foundation });
		}
		None
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DstTableau {
	pub src: KlondikePileStack,
	pub tableau: Tableau,
}
impl DstTableau {
	const ITER_BEGIN: Self = Self {
		src: KlondikePileStack::ITER_BEGIN,
		tableau: Tableau::ITER_BEGIN,
	};
	const fn next(self) -> Option<Self> {
		let DstTableau { src, tableau } = self;
		if let Some(src) = src.next() {
			return Some(Self { src, tableau });
		}
		if let Some(tableau) = tableau.next() {
			let src = KlondikePileStack::ITER_BEGIN;
			return Some(Self { src, tableau });
		}
		None
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum KlondikeInstruction {
	DstFoundation(DstFoundation),
	DstTableau(DstTableau),
	RotateStock,
}
impl KlondikeInstruction {
	const ITER_BEGIN: Self = Self::DstFoundation(DstFoundation::ITER_BEGIN);
	const fn next(self) -> Option<Self> {
		Some(match self {
			Self::DstFoundation(dst_foundation) => match dst_foundation.next() {
				Some(dst_foundation) => Self::DstFoundation(dst_foundation),
				None => Self::DstTableau(DstTableau::ITER_BEGIN),
			},
			Self::DstTableau(tableau) => match tableau.next() {
				Some(tableau) => Self::DstTableau(tableau),
				None => Self::RotateStock,
			},
			Self::RotateStock => return None,
		})
	}
	/// foundation -> foundation is a useless move
	pub fn is_useless(&self) -> bool {
		matches!(
			self,
			KlondikeInstruction::DstFoundation(DstFoundation {
				src: KlondikePile::Foundation(_),
				..
			})
		)
	}
}

const TABLEAUS: usize = 7;
const fn sum(n: usize) -> usize {
	n * (n + 1) / 2
}
const STOCK: usize = 52 - sum(TABLEAUS);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct KlondikeState {
	stock: Pile<STOCK, STOCK>,
	foundations: [Stack<13>; 4],
	tableau1: Pile<0, 13>,
	tableau2: Pile<1, 13>,
	tableau3: Pile<2, 13>,
	tableau4: Pile<3, 13>,
	tableau5: Pile<4, 13>,
	tableau6: Pile<5, 13>,
	tableau7: Pile<6, 13>,
}
impl KlondikeState {
	pub const fn stock(&self) -> &Pile<STOCK, STOCK> {
		&self.stock
	}
	pub const fn foundation1(&self) -> &Stack<13> {
		&self.foundations[Foundation::Foundation1 as usize]
	}
	pub const fn foundation2(&self) -> &Stack<13> {
		&self.foundations[Foundation::Foundation2 as usize]
	}
	pub const fn foundation3(&self) -> &Stack<13> {
		&self.foundations[Foundation::Foundation3 as usize]
	}
	pub const fn foundation4(&self) -> &Stack<13> {
		&self.foundations[Foundation::Foundation4 as usize]
	}
	pub const fn tableau1(&self) -> &Pile<0, 13> {
		&self.tableau1
	}
	pub const fn tableau2(&self) -> &Pile<1, 13> {
		&self.tableau2
	}
	pub const fn tableau3(&self) -> &Pile<2, 13> {
		&self.tableau3
	}
	pub const fn tableau4(&self) -> &Pile<3, 13> {
		&self.tableau4
	}
	pub const fn tableau5(&self) -> &Pile<4, 13> {
		&self.tableau5
	}
	pub const fn tableau6(&self) -> &Pile<5, 13> {
		&self.tableau6
	}
	pub const fn tableau7(&self) -> &Pile<6, 13> {
		&self.tableau7
	}
	pub fn tableau_face_down_cards(&self, tableau: Tableau) -> &[Card] {
		match tableau {
			Tableau::Tableau1 => self.tableau1.face_down(),
			Tableau::Tableau2 => self.tableau2.face_down(),
			Tableau::Tableau3 => self.tableau3.face_down(),
			Tableau::Tableau4 => self.tableau4.face_down(),
			Tableau::Tableau5 => self.tableau5.face_down(),
			Tableau::Tableau6 => self.tableau6.face_down(),
			Tableau::Tableau7 => self.tableau7.face_down(),
		}
	}
	pub fn tableau_face_up_cards(&self, tableau: Tableau) -> &[Card] {
		match tableau {
			Tableau::Tableau1 => self.tableau1.face_up(),
			Tableau::Tableau2 => self.tableau2.face_up(),
			Tableau::Tableau3 => self.tableau3.face_up(),
			Tableau::Tableau4 => self.tableau4.face_up(),
			Tableau::Tableau5 => self.tableau5.face_up(),
			Tableau::Tableau6 => self.tableau6.face_up(),
			Tableau::Tableau7 => self.tableau7.face_up(),
		}
	}
	pub fn stack_bottom_card(&self, src: KlondikePileStack) -> Option<&Card> {
		match src {
			KlondikePileStack::Tableau(TableauStack {
				tableau,
				skip_cards,
			}) => self.tableau_face_up_cards(tableau).get(skip_cards as usize),
			KlondikePileStack::Foundation(foundation) => {
				self.foundations[foundation as usize].last()
			}
			KlondikePileStack::Stock => self.stock.face_up().last(),
		}
	}
	pub fn top_card<S: Into<KlondikePile>>(&self, src: S) -> Option<&Card> {
		match src.into() {
			KlondikePile::Tableau(tableau) => self.tableau_face_up_cards(tableau).last(),
			KlondikePile::Foundation(foundation) => self.foundations[foundation as usize].last(),
			KlondikePile::Stock => self.stock.face_up().last(),
		}
	}
	fn take_stack(&mut self, src: KlondikePileStack) -> (Stack<13>, bool) {
		match src {
			KlondikePileStack::Tableau(TableauStack {
				tableau,
				skip_cards,
			}) => match tableau {
				Tableau::Tableau1 => self.tableau1.take_range_flip_up(skip_cards as usize..),
				Tableau::Tableau2 => self.tableau2.take_range_flip_up(skip_cards as usize..),
				Tableau::Tableau3 => self.tableau3.take_range_flip_up(skip_cards as usize..),
				Tableau::Tableau4 => self.tableau4.take_range_flip_up(skip_cards as usize..),
				Tableau::Tableau5 => self.tableau5.take_range_flip_up(skip_cards as usize..),
				Tableau::Tableau6 => self.tableau6.take_range_flip_up(skip_cards as usize..),
				Tableau::Tableau7 => self.tableau7.take_range_flip_up(skip_cards as usize..),
			},
			KlondikePileStack::Foundation(foundation) => (
				Stack::from_iter(self.foundations[foundation as usize].pop()),
				false,
			),
			KlondikePileStack::Stock => (Stack::from_iter(self.stock.pop()), false),
		}
	}
	fn take_top_card<S: Into<KlondikePile>>(&mut self, src: S) -> (Option<Card>, bool) {
		match src.into() {
			KlondikePile::Tableau(tableau) => match tableau {
				Tableau::Tableau1 => self.tableau1.pop_flip_up(),
				Tableau::Tableau2 => self.tableau2.pop_flip_up(),
				Tableau::Tableau3 => self.tableau3.pop_flip_up(),
				Tableau::Tableau4 => self.tableau4.pop_flip_up(),
				Tableau::Tableau5 => self.tableau5.pop_flip_up(),
				Tableau::Tableau6 => self.tableau6.pop_flip_up(),
				Tableau::Tableau7 => self.tableau7.pop_flip_up(),
			},
			KlondikePile::Foundation(foundation) => {
				(self.foundations[foundation as usize].pop(), false)
			}
			KlondikePile::Stock => (self.stock.pop(), false),
		}
	}
	fn extend_foundation<I: IntoIterator<Item = Card>>(
		&mut self,
		foundation: Foundation,
		cards: I,
	) {
		self.foundations[foundation as usize].extend(cards)
	}
	fn extend_tableau<I: IntoIterator<Item = Card>>(&mut self, tableau: Tableau, cards: I) {
		match tableau {
			Tableau::Tableau1 => self.tableau1.extend(cards),
			Tableau::Tableau2 => self.tableau2.extend(cards),
			Tableau::Tableau3 => self.tableau3.extend(cards),
			Tableau::Tableau4 => self.tableau4.extend(cards),
			Tableau::Tableau5 => self.tableau5.extend(cards),
			Tableau::Tableau6 => self.tableau6.extend(cards),
			Tableau::Tableau7 => self.tableau7.extend(cards),
		}
	}
	pub fn is_instruction_valid(
		&self,
		config: &KlondikeConfig,
		instruction: KlondikeInstruction,
	) -> bool {
		match instruction {
			// Stock -> Stock draws a card or resets the stock
			KlondikeInstruction::RotateStock => {
				// cannot move stock when stock is empty
				!self.stock.is_empty()
			}

			// moving to foundation has special rules
			KlondikeInstruction::DstFoundation(dst_foundation) => {
				// get the top cards
				if let Some(src_card) = self.top_card(dst_foundation.src) {
					match self.top_card(dst_foundation.foundation) {
						// destination card exists
						Some(dst_card) => {
							// suit matches?
							src_card.suit() == dst_card.suit()
							// value is +1?
							&& dst_card.rank().checked_add(1) == Some(src_card.rank())
						}
						// only ace is allowed to go onto empty foundation
						None => src_card.rank() == Rank::Ace,
					}
				} else {
					false
				}
			}
			// other = move to tableau
			KlondikeInstruction::DstTableau(dst_tableau) => {
				if config.move_from_foundation == MoveFromFoundationConfig::Disallowed
					&& let KlondikePileStack::Foundation(_) = dst_tableau.src
				{
					return false;
				}
				// get the cards
				if let Some(src_card) = self.stack_bottom_card(dst_tableau.src) {
					match self.top_card(dst_tableau.tableau) {
						// destination card exists
						Some(dst_card) => {
							// red-ness is opposite?
							src_card.is_red() != dst_card.is_red()
							// value is -1?
							&& dst_card.rank().checked_sub(1) == Some(src_card.rank())
						}
						// only king is allowed to go onto empty tableau
						None => src_card.rank() == Rank::King,
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
			instruction: Some(KlondikeInstruction::ITER_BEGIN),
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
#[test]
fn test_klondike_iter() {
	assert_eq!(KlondikeIter::new().count(), 721);
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Klondike {
	state: KlondikeState,
}
impl Klondike {
	pub fn with_seed(seed: u64) -> Self {
		use rand::SeedableRng;
		let mut rng = Rng::seed_from_u64(seed);
		Self::with_rng(&mut rng)
	}
	pub fn with_rng(rng: &mut Rng) -> Self {
		// shuffle a new deck
		let mut deck = Stack::full_deck(card_game::Deck::Deck1);
		use rand::seq::SliceRandom;
		deck.shuffle(rng);
		let mut deck = deck.into_iter();

		// generate tableaus
		fn pile<const DN: usize>(deck: &mut <Stack<52> as IntoIterator>::IntoIter) -> Pile<DN, 13> {
			let stack = Stack::from_iter(deck.take(DN));
			let mut pile = Pile::new_face_down(stack);
			pile.push(deck.next().unwrap());
			pile
		}
		let tableau1 = pile(&mut deck);
		let tableau2 = pile(&mut deck);
		let tableau3 = pile(&mut deck);
		let tableau4 = pile(&mut deck);
		let tableau5 = pile(&mut deck);
		let tableau6 = pile(&mut deck);
		let tableau7 = pile(&mut deck);

		// stock is remaining cards
		let stock = Pile::new_face_down(Stack::from_iter(deck));

		let state = KlondikeState {
			stock,
			foundations: core::array::from_fn(|_| Stack::new()),
			tableau1,
			tableau2,
			tableau3,
			tableau4,
			tableau5,
			tableau6,
			tableau7,
		};
		Self { state }
	}
	pub const fn state(&self) -> &KlondikeState {
		&self.state
	}
	/// Check if the game should be auto-completed
	pub fn is_win_trivial(&self) -> bool {
		// all face down cards empty means win
		self.state.stock.is_empty()
			&& self.state.tableau1.face_down().is_empty()
			&& self.state.tableau2.face_down().is_empty()
			&& self.state.tableau3.face_down().is_empty()
			&& self.state.tableau4.face_down().is_empty()
			&& self.state.tableau5.face_down().is_empty()
			&& self.state.tableau6.face_down().is_empty()
			&& self.state.tableau7.face_down().is_empty()
	}
	fn instruction_priority(&self, instruction: &KlondikeInstruction) -> usize {
		// 1 Move into foundation
		// 2 T->T Move to reveal new card (moving a non-king to reveal empty tableau also counts)
		// 3 Move from stock
		// 4 Rotate stock
		// 5 T->T Move not revealing new card
		// 6 Move from foundation
		match instruction {
			KlondikeInstruction::DstFoundation(_) => 1,
			&KlondikeInstruction::DstTableau(dst_tableau) => match dst_tableau.src {
				KlondikePileStack::Tableau(TableauStack {
					tableau,
					skip_cards: SkipCards::Skip0,
				}) if !self.state().tableau_face_down_cards(tableau).is_empty()
					|| self
						.state()
						.stack_bottom_card(dst_tableau.src)
						.is_some_and(|card| card.rank() != Rank::King) =>
				{
					2
				}
				KlondikePileStack::Stock => 3,
				KlondikePileStack::Tableau(_) => 5,
				KlondikePileStack::Foundation(_) => 6,
			},
			KlondikeInstruction::RotateStock => 4,
		}
	}
	/// A single move that usually makes progress towards a winning game
	pub fn get_auto_move(&self, config: &KlondikeConfig) -> Option<KlondikeInstruction> {
		self.possible_instructions(config)
			.filter(|ins| !ins.is_useless())
			.min_by_key(|ins| self.instruction_priority(ins))
	}
	/// A list of possible moves with useless moves filtered out and sorted by a simple priority function
	pub fn get_sorted_moves(&self, config: &KlondikeConfig) -> Vec<KlondikeInstruction> {
		let mut useful_moves: Vec<_> = self
			.possible_instructions(config)
			.filter(|ins| !ins.is_useless())
			.collect();
		useful_moves.sort_by_key(|ins| self.instruction_priority(ins));
		useful_moves
	}
}

impl Game for Klondike {
	type Score = i32;
	type Stats = KlondikeStats;
	type Config = KlondikeConfig;
	type Instruction = KlondikeInstruction;
	fn score(&self, stats: &Self::Stats, config: &Self::Config) -> Self::Score {
		stats.score(&config.scoring)
	}
	fn possible_instructions(
		&self,
		config: &Self::Config,
	) -> impl Iterator<Item = Self::Instruction> + use<> {
		let state = self.state.clone();
		let config = config.clone();
		KlondikeIter::new()
			.filter(move |&instruction| state.is_instruction_valid(&config, instruction))
	}
	fn is_instruction_valid(&self, config: &Self::Config, instruction: Self::Instruction) -> bool {
		self.state.is_instruction_valid(config, instruction)
	}
	fn process_instruction(
		&mut self,
		stats: &mut Self::Stats,
		config: &Self::Config,
		instruction: Self::Instruction,
	) {
		stats.increment_moves();
		match instruction {
			// Reset the stock if it's empty
			KlondikeInstruction::RotateStock => {
				if self.state.stock.face_down().is_empty() {
					self.state.stock.flip_it_and_reverse_it();
					stats.increment_recycle_count();
				} else {
					for _ in 0..config.draw_stock as usize {
						self.state.stock.flip_up();
					}
				}
			}
			// Move a card from anywhere to a foundation
			KlondikeInstruction::DstFoundation(DstFoundation { src, foundation }) => {
				stats.increment_move_to_foundation();
				let (card, did_flip_up) = self.state.take_top_card(src);
				if did_flip_up {
					stats.increment_flip_up_bonus();
				}
				self.state.extend_foundation(foundation, card);
			}
			// Move a stack of cards from anywhere to a tableau
			KlondikeInstruction::DstTableau(DstTableau { src, tableau }) => {
				match src {
					KlondikePileStack::Stock => stats.increment_move_to_tableau(),
					KlondikePileStack::Foundation(_) => stats.increment_move_from_foundation(),
					_ => {}
				}
				let (cards, did_flip_up) = self.state.take_stack(src);
				if did_flip_up {
					stats.increment_flip_up_bonus();
				}
				self.state.extend_tableau(tableau, cards);
			}
		}
	}
	fn is_win(&self) -> bool {
		// all foundations contain all ranks
		self.state.foundations.iter().all(|foundation| {
			foundation.len() == Rank::RANKS.len()
				&& foundation
					.iter()
					.zip(Rank::RANKS)
					.all(|(card, rank)| card.rank() == rank)
		})
	}
}
