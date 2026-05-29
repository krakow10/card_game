// test readme
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

use core::ops::RangeBounds;

// TODO: pub struct ValidInstruction<I>(I);
pub trait Game: Clone {
	type Score: Clone + core::fmt::Debug;
	type Stats: Clone + core::fmt::Debug;
	type Config: Clone + core::fmt::Debug;
	type Instruction: Clone + core::fmt::Debug;
	fn score(&self, stats: &Self::Stats, config: &Self::Config) -> Self::Score;
	fn possible_instructions(
		&self,
		config: &Self::Config,
	) -> impl Iterator<Item = Self::Instruction> + use<Self>;
	fn is_instruction_valid(&self, config: &Self::Config, instruction: Self::Instruction) -> bool;
	fn process_instruction(
		&mut self,
		stats: &mut Self::Stats,
		config: &Self::Config,
		instruction: Self::Instruction,
	);
	fn is_win(&self) -> bool;
}

/// card_game supports up to 4 identifiably separate decks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Deck {
	Deck1 = 0b00,
	Deck2 = 0b01,
	Deck3 = 0b10,
	Deck4 = 0b11,
}
impl Deck {
	pub const fn new(deck: u8) -> Option<Self> {
		use Deck::*;
		Some(match deck {
			0b00 => Deck1,
			0b01 => Deck2,
			0b10 => Deck3,
			0b11 => Deck4,
			_ => return None,
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Suit {
	Spades = 0b00,
	Hearts = 0b01,
	Clubs = 0b10,
	Diamonds = 0b11,
}
impl Suit {
	pub const SUITS: [Self; 4] = [Self::Spades, Self::Hearts, Self::Clubs, Self::Diamonds];
	pub const fn new(suit: u8) -> Option<Self> {
		use Suit::*;
		Some(match suit {
			0b00 => Spades,
			0b01 => Hearts,
			0b10 => Clubs,
			0b11 => Diamonds,
			_ => return None,
		})
	}
	/// Is the suit red.
	pub const fn is_red(self) -> bool {
		self as u8 & 0b01 != 0
	}
	/// Suit value is 2 bits, is_red is the low bit.
	pub const fn suit_high_bit(self) -> bool {
		self as u8 & 0b10 != 0
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rank {
	Ace = 1,
	Two = 2,
	Three = 3,
	Four = 4,
	Five = 5,
	Six = 6,
	Seven = 7,
	Eight = 8,
	Nine = 9,
	Ten = 10,
	Jack = 11,
	Queen = 12,
	King = 13,
}
impl Rank {
	pub const RANKS: [Self; 13] = [
		Self::Ace,
		Self::Two,
		Self::Three,
		Self::Four,
		Self::Five,
		Self::Six,
		Self::Seven,
		Self::Eight,
		Self::Nine,
		Self::Ten,
		Self::Jack,
		Self::Queen,
		Self::King,
	];
	pub const fn new(rank: u8) -> Option<Self> {
		use Rank::*;
		Some(match rank {
			1 => Ace,
			2 => Two,
			3 => Three,
			4 => Four,
			5 => Five,
			6 => Six,
			7 => Seven,
			8 => Eight,
			9 => Nine,
			10 => Ten,
			11 => Jack,
			12 => Queen,
			13 => King,
			_ => return None,
		})
	}
	pub const fn checked_add(self, offset: u8) -> Option<Rank> {
		match (self as u8).checked_add(offset) {
			Some(rank) => Self::new(rank),
			None => None,
		}
	}
	pub const fn checked_sub(self, offset: u8) -> Option<Rank> {
		match (self as u8).checked_sub(offset) {
			Some(rank) => Self::new(rank),
			None => None,
		}
	}
}

/// A card which specifies the deck id, suit, and card value.
/// 2 bits for deck ID
/// 2 bits for suit ID
/// 4 bits for card Value
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Card(core::num::NonZeroU8);
impl Card {
	pub const fn new(deck: Deck, suit: Suit, rank: Rank) -> Self {
		let packed = (deck as u8) << 6 | (suit as u8) << 4 | (rank as u8);
		Self(core::num::NonZeroU8::new(packed).unwrap())
	}
	pub const fn rank(&self) -> Rank {
		let masked = self.0.get() & 0b1111;
		Rank::new(masked).unwrap()
	}
	pub const fn suit(&self) -> Suit {
		let low_bit = self.is_red();
		let high_bit = self.suit_high_bit();
		match (high_bit, low_bit) {
			(false, false) => Suit::Spades,
			(false, true) => Suit::Hearts,
			(true, false) => Suit::Clubs,
			(true, true) => Suit::Diamonds,
		}
	}
	/// Is the suit red.
	pub const fn is_red(&self) -> bool {
		self.0.get() & 0b010000 != 0
	}
	/// Suit value is 2 bits, is_red is the low bit.
	pub const fn suit_high_bit(&self) -> bool {
		self.0.get() & 0b100000 != 0
	}
	pub const fn deck(&self) -> Deck {
		Deck::new(self.0.get() >> 6).unwrap()
	}
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Stack<const CAP: usize>(arrayvec::ArrayVec<Card, CAP>);
impl<const CAP: usize> Stack<CAP> {
	pub const fn new() -> Self {
		Self(arrayvec::ArrayVec::new_const())
	}
	pub fn take_range<R: RangeBounds<usize>>(&mut self, range: R) -> Self {
		Stack::from_iter(self.drain(range))
	}
}
impl Stack<52> {
	/// Generate a full deck of cards with the specified deck id.
	pub fn full_deck(deck: Deck) -> Self {
		let mut stack = arrayvec::ArrayVec::new();
		for suit in Suit::SUITS {
			for rank in Rank::RANKS {
				stack.push(Card::new(deck, suit, rank));
			}
		}
		Stack(stack)
	}
}
impl<const CAP: usize> From<arrayvec::ArrayVec<Card, CAP>> for Stack<CAP> {
	fn from(value: arrayvec::ArrayVec<Card, CAP>) -> Self {
		Self(value)
	}
}
impl<const CAP: usize> FromIterator<Card> for Stack<CAP> {
	fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
		Self(arrayvec::ArrayVec::from_iter(iter))
	}
}
impl<const CAP: usize> core::ops::Deref for Stack<CAP> {
	type Target = arrayvec::ArrayVec<Card, CAP>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<const CAP: usize> core::ops::DerefMut for Stack<CAP> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl<const CAP: usize> IntoIterator for Stack<CAP> {
	type Item = Card;
	type IntoIter = arrayvec::IntoIter<Card, CAP>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

/// A pile is a stack of face down cards and a stack of face up cards.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Pile<const DN: usize, const UP: usize> {
	face_down: Stack<DN>,
	face_up: Stack<UP>,
}
impl<const DN: usize, const UP: usize> Pile<DN, UP> {
	pub const fn new() -> Self {
		Self {
			face_down: Stack::new(),
			face_up: Stack::new(),
		}
	}
	pub const fn new_face_down(stack: Stack<DN>) -> Self {
		Self {
			face_down: stack,
			face_up: Stack::new(),
		}
	}
	/// Returns whether a card was flipped up.
	pub fn flip_up(&mut self) -> bool {
		if let Some(card) = self.face_down.pop() {
			self.face_up.push(card);
			return true;
		}
		false
	}
	pub fn is_empty(&self) -> bool {
		self.face_down.is_empty() && self.face_up.is_empty()
	}
	pub fn pop(&mut self) -> Option<Card> {
		self.face_up.pop()
	}
	/// Returns the popped card and whether a card was flipped up.
	pub fn pop_flip_up(&mut self) -> (Option<Card>, bool) {
		let card = match self.face_up.pop() {
			Some(card) => card,
			None => return (None, false),
		};
		let did_flip_up = if self.face_up.is_empty() {
			self.flip_up()
		} else {
			false
		};
		(Some(card), did_flip_up)
	}
	pub fn take_range<R: RangeBounds<usize>>(&mut self, range: R) -> Stack<UP> {
		self.face_up.take_range(range)
	}
	/// Returns the card range and whether a card was flipped up.
	pub fn take_range_flip_up<R: RangeBounds<usize>>(&mut self, range: R) -> (Stack<UP>, bool) {
		let cards = self.take_range(range);
		let did_flip_up = if self.face_up.is_empty() {
			self.flip_up()
		} else {
			false
		};
		(cards, did_flip_up)
	}
	pub fn push(&mut self, card: Card) {
		self.face_up.push(card);
	}
	pub fn extend<I: IntoIterator<Item = Card>>(&mut self, cards: I) {
		self.face_up.extend(cards);
	}
	pub fn face_up(&self) -> &[Card] {
		&self.face_up
	}
	pub fn face_down(&self) -> &[Card] {
		&self.face_down
	}
}
impl<const CAP: usize> Pile<CAP, CAP> {
	pub fn flip_it_and_reverse_it(&mut self) {
		self.swap_up_down();
		self.face_down.reverse();
	}
	pub const fn swap_up_down(&mut self) {
		core::mem::swap(&mut self.face_up, &mut self.face_down);
	}
}

#[derive(Clone, Debug)]
pub enum SolveError {
	MovesBudgetExceeded,
	StatesBudgetExceeded,
}
impl std::fmt::Display for SolveError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for SolveError {}

#[derive(Clone, Debug)]
pub enum SessionInstruction<I> {
	Undo,
	InnerInstruction(I),
}

#[derive(Clone, Debug, Default)]
pub struct SessionStats<S> {
	inner: S,
	undos: u32,
}
impl<S> SessionStats<S> {
	pub const fn stats(&self) -> &S {
		&self.inner
	}
	const fn increment_undos(&mut self) {
		self.undos += 1;
	}
	pub const fn undos(&self) -> u32 {
		self.undos
	}
}
#[derive(Clone, Debug)]
pub struct SessionConfig<C> {
	pub inner: C,
	pub undo_penalty: i32,
	pub solve_moves_budget: u64,
	pub solve_states_budget: u64,
}
impl<C> SessionConfig<C> {
	fn new_default(inner: C) -> Self {
		Self {
			inner,
			undo_penalty: -15,
			solve_moves_budget: 100_000,
			solve_states_budget: 100_000,
		}
	}
}
impl<C: Default> Default for SessionConfig<C> {
	fn default() -> Self {
		Self::new_default(C::default())
	}
}

#[derive(Clone, Debug)]
pub struct Session<G: Game> {
	stats: SessionStats<G::Stats>,
	config: SessionConfig<G::Config>,
	state: SessionState<G>,
}
#[derive(Clone, Debug)]
pub struct StateSnapshot<G: Game> {
	state: G,
	instruction: G::Instruction,
}
impl<G: Game> StateSnapshot<G> {
	pub const fn state(&self) -> &G {
		&self.state
	}
	pub const fn instruction(&self) -> &G::Instruction {
		&self.instruction
	}
}
#[derive(Clone, Debug)]
pub struct SessionState<G: Game> {
	state: G,
	history: Vec<StateSnapshot<G>>,
}
impl<G: Game + Clone> SessionState<G> {
	fn new(state: G) -> Self {
		Self {
			state,
			history: Vec::new(),
		}
	}
}
impl<G: Game> SessionState<G> {
	pub const fn state(&self) -> &G {
		&self.state
	}
}
impl<G: Game<Score = i32>> Session<G>
where
	G: Eq + core::hash::Hash,
	G::Stats: Default,
	G::Instruction: Eq + core::hash::Hash,
{
	pub fn new(state: G, config: SessionConfig<G::Config>) -> Self {
		Self {
			stats: SessionStats::default(),
			config,
			state: SessionState::new(state),
		}
	}
	pub fn new_default(state: G) -> Self
	where
		G::Config: Default,
	{
		Self::new(state, Default::default())
	}
	pub const fn stats(&self) -> &SessionStats<G::Stats> {
		&self.stats
	}
	pub const fn state(&self) -> &SessionState<G> {
		&self.state
	}
	pub const fn config(&self) -> &SessionConfig<G::Config> {
		&self.config
	}
	pub fn history(&self) -> &[StateSnapshot<G>] {
		&self.state.history
	}
	pub fn undo(&mut self) {
		self.state
			.process_instruction(&mut self.stats, &self.config, SessionInstruction::Undo)
	}
	pub fn possible_instructions(&self) -> impl Iterator<Item = G::Instruction> + use<G> {
		self.state.state.possible_instructions(&self.config.inner)
	}
	pub fn process_instruction(&mut self, instruction: G::Instruction) {
		self.state.process_instruction(
			&mut self.stats,
			&self.config,
			SessionInstruction::InnerInstruction(instruction),
		)
	}
	pub fn is_win(&self) -> bool {
		self.state.is_win()
	}
	pub fn solve(&self) -> Result<Option<Vec<StateSnapshot<G>>>, SolveError> {
		let mut state_moves = std::collections::HashMap::new();
		let mut state = self.clone();
		let mut moves = 0;
		while !state.is_win() {
			moves += 1;
			if self.config.solve_moves_budget < moves {
				return Err(SolveError::MovesBudgetExceeded);
			}
			if self.config.solve_states_budget < state_moves.len() as u64 {
				return Err(SolveError::StatesBudgetExceeded);
			}
			// Continue existing iterator if it exists
			let it = state_moves
				.entry(state.state().state().clone())
				.or_insert_with(|| {
					state
						.state()
						.state()
						.possible_instructions(&self.config().inner)
				});

			// Run one possible move
			if let Some(instruction) = it.next() {
				state.process_instruction(instruction);
				continue;
			}

			// No more moves. If we can't undo we're done
			if state.history().is_empty() {
				return Ok(None);
			} else {
				state.undo();
			}
		}
		Ok(Some(state.state.history))
	}
}
impl<G: Game<Score = i32>> Game for SessionState<G>
where
	G::Stats: Default,
{
	type Score = i32;
	type Stats = SessionStats<G::Stats>;
	type Config = SessionConfig<G::Config>;
	type Instruction = SessionInstruction<G::Instruction>;
	fn score(&self, stats: &Self::Stats, config: &Self::Config) -> Self::Score {
		self.state.score(&stats.inner, &config.inner) + stats.undos as i32 * config.undo_penalty
	}
	fn possible_instructions(
		&self,
		config: &Self::Config,
	) -> impl Iterator<Item = Self::Instruction> + use<G> {
		self.state
			.possible_instructions(&config.inner)
			.map(SessionInstruction::InnerInstruction)
	}
	fn is_instruction_valid(&self, config: &Self::Config, instruction: Self::Instruction) -> bool {
		match instruction {
			SessionInstruction::Undo => !self.history.is_empty(),
			SessionInstruction::InnerInstruction(instruction) => {
				self.state.is_instruction_valid(&config.inner, instruction)
			}
		}
	}
	fn process_instruction(
		&mut self,
		stats: &mut Self::Stats,
		config: &Self::Config,
		instruction: Self::Instruction,
	) {
		match instruction {
			SessionInstruction::Undo => {
				if let Some(snapshot) = self.history.pop() {
					self.state = snapshot.state;
					stats.increment_undos();
				}
			}
			SessionInstruction::InnerInstruction(instruction) => {
				self.history.push(StateSnapshot {
					state: self.state.clone(),
					instruction: instruction.clone(),
				});
				self.state
					.process_instruction(&mut stats.inner, &config.inner, instruction);
			}
		}
	}
	fn is_win(&self) -> bool {
		self.state.is_win()
	}
}
