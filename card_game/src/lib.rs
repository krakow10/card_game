use core::ops::RangeBounds;

// TODO: pub struct ValidInstruction<I>(I);
pub trait Game {
	type Stats;
	type Config;
	type Instruction;
	fn possible_instructions(&self) -> impl Iterator<Item = Self::Instruction> + use<Self>;
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
	/// Is the suit red.
	pub const fn is_red(self) -> bool {
		self as u8 & 0b01 != 0
	}
	/// Is the suit shape spikey.  (Bouba/kiki)
	pub const fn is_kiki(self) -> bool {
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
/// An identifier which specifies the deck id, suit, and card value.
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
	pub const fn is_red(&self) -> bool {
		self.0.get() & 0b010000 != 0
	}
	/// Is the suit shape spikey.  (Bouba/kiki)
	pub const fn is_kiki(&self) -> bool {
		self.0.get() & 0b100000 != 0
	}
	pub const fn deck(&self) -> Deck {
		Deck::new(self.0.get() >> 6).unwrap()
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
	pub fn flip_up(&mut self) {
		if let Some(card) = self.face_down.pop() {
			self.face_up.push(card);
		}
	}
	pub fn is_empty(&self) -> bool {
		self.face_down.is_empty() && self.face_up.is_empty()
	}
	pub fn pop(&mut self) -> Option<Card> {
		self.face_up.pop()
	}
	pub fn pop_flip_up(&mut self) -> Option<Card> {
		let card = self.face_up.pop()?;
		if self.face_up.is_empty() {
			self.flip_up();
		}
		Some(card)
	}
	pub fn take_range<R: RangeBounds<usize>>(&mut self, range: R) -> Stack<UP> {
		// if self.face_up.get(range).is_none() {
		// 	return None;
		// }
		self.face_up.take_range(range)
	}
	pub fn take_range_flip_up<R: RangeBounds<usize>>(&mut self, range: R) -> Stack<UP> {
		let cards = self.take_range(range);
		if self.face_up.is_empty() {
			self.flip_up();
		}
		cards
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
pub enum SessionInstruction<I> {
	Undo,
	InnerInstruction(I),
}

#[derive(Clone, Debug, Default)]
pub struct SessionStats<S> {
	inner_stats: S,
	undos: usize,
}
impl<S> SessionStats<S> {
	pub const fn stats(&self) -> &S {
		&self.inner_stats
	}
	const fn increment_undos(&mut self) {
		self.undos += 1;
	}
	pub const fn undos(&self) -> usize {
		self.undos
	}
}

pub struct Session<G: Game> {
	stats: SessionStats<G::Stats>,
	config: G::Config,
	state: SessionState<G>,
}
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct SessionState<G: Game> {
	seed: G,
	state: G,
	history: Vec<G::Instruction>,
}
impl<G: Game + Clone> SessionState<G> {
	fn new(state: G) -> Self {
		Self {
			seed: state.clone(),
			state,
			history: Vec::new(),
		}
	}
}
impl<G: Game> Session<G>
where
	G: Clone + Eq + core::hash::Hash,
	G::Stats: Clone + Default,
	G::Instruction: Clone + Eq + core::hash::Hash,
{
	pub fn new(state: G, config: G::Config) -> Self {
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
	pub const fn state(&self) -> &G {
		&self.state.state
	}
	pub const fn config(&self) -> &G::Config {
		&self.config
	}
	pub fn history(&self) -> &[G::Instruction] {
		&self.state.history
	}
	pub fn undo(&mut self) {
		self.state
			.process_instruction(&mut self.stats, &self.config, SessionInstruction::Undo)
	}
	pub fn possible_instructions(&self) -> impl Iterator<Item = G::Instruction> + use<G> {
		self.state.state.possible_instructions()
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
	pub fn is_winnable(&self) -> Option<Vec<G::Instruction>> {
		let mut observed = std::collections::HashSet::new();
		struct StateMachine<G, P, I> {
			state: G,
			possible_instructions_iter: P,
			instruction: I,
		}
		let mut dummy_stats = self.stats.inner_stats.clone();
		let mut state = self.state.state.clone();
		let mut it = state.possible_instructions();
		let mut path = Vec::new();
		'outer: while !state.is_win() {
			observed.insert(state.clone());
			for instruction in &mut it {
				let mut next_state = state.clone();
				next_state.process_instruction(&mut dummy_stats, &self.config, instruction.clone());
				if !observed.contains(&next_state) {
					let possible_instructions_iter =
						core::mem::replace(&mut it, next_state.possible_instructions());
					let state = core::mem::replace(&mut state, next_state);
					path.push(StateMachine {
						state,
						possible_instructions_iter,
						instruction,
					});
					continue 'outer;
				}
			}
			let Some(last_state) = path.pop() else {
				return None;
			};
			state = last_state.state;
			it = last_state.possible_instructions_iter;
		}
		Some(path.into_iter().map(|state| state.instruction).collect())
	}
}
impl<G: Game> Game for SessionState<G>
where
	G: Clone,
	G::Stats: Default,
	G::Instruction: Clone,
{
	type Stats = SessionStats<G::Stats>;
	type Config = G::Config;
	type Instruction = SessionInstruction<G::Instruction>;
	fn possible_instructions(&self) -> impl Iterator<Item = Self::Instruction> + use<G> {
		self.state
			.possible_instructions()
			.map(SessionInstruction::InnerInstruction)
	}
	fn is_instruction_valid(&self, config: &Self::Config, instruction: Self::Instruction) -> bool {
		match instruction {
			SessionInstruction::Undo => !self.history.is_empty(),
			SessionInstruction::InnerInstruction(instruction) => {
				self.state.is_instruction_valid(config, instruction)
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
				// replay the entire history of the game except one move
				self.history.pop();
				let mut inner_stats = G::Stats::default();
				let mut state = self.seed.clone();
				for instruction in &self.history {
					state.process_instruction(&mut inner_stats, config, instruction.clone());
				}
				self.state = state;
				stats.inner_stats = inner_stats;
				stats.increment_undos();
			}
			SessionInstruction::InnerInstruction(instruction) => {
				self.history.push(instruction.clone());
				self.state
					.process_instruction(&mut stats.inner_stats, config, instruction);
			}
		}
	}
	fn is_win(&self) -> bool {
		self.state.is_win()
	}
}
