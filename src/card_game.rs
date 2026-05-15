// TODO: pub struct ValidInstruction<I>(I);
pub trait Game {
	type Instruction;
	fn possible_instructions(&self) -> impl Iterator<Item = Self::Instruction> + use<Self>;
	fn is_instruction_valid(&self, instruction: Self::Instruction) -> bool;
	fn process_instruction(&mut self, instruction: Self::Instruction);
	fn is_win(&self) -> bool;
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
	pub fn is_red(self) -> bool {
		self as u8 & 0b01 != 0
	}
	/// Is the suit shape spikey.  (Bouba/kiki)
	pub fn is_kiki(self) -> bool {
		self as u8 & 0b10 != 0
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardValue(u8);
impl CardValue {
	pub const ACE: Self = CardValue(1);
	pub const TWO: Self = CardValue(2);
	pub const THREE: Self = CardValue(3);
	pub const FOUR: Self = CardValue(4);
	pub const FIVE: Self = CardValue(5);
	pub const SIX: Self = CardValue(6);
	pub const SEVEN: Self = CardValue(7);
	pub const EIGHT: Self = CardValue(8);
	pub const NINE: Self = CardValue(9);
	pub const TEN: Self = CardValue(10);
	pub const JACK: Self = CardValue(11);
	pub const QUEEN: Self = CardValue(12);
	pub const KING: Self = CardValue(13);
	pub fn get(self) -> u8 {
		self.0
	}
	pub fn checked_add(self, offset: u8) -> Option<CardValue> {
		let new_value = self.0.checked_add(offset)?;
		if 13 < new_value {
			None
		} else {
			Some(CardValue(new_value))
		}
	}
	pub fn checked_sub(self, offset: u8) -> Option<CardValue> {
		let new_value = self.0.checked_sub(offset)?;
		if new_value < 1 {
			None
		} else {
			Some(CardValue(new_value))
		}
	}
}
/// An identifier which specifies the deck id, suit, and card value.
/// 2 bits for deck ID
/// 2 bits for suit ID
/// 4 bits for card Value
/// TODO: better encoding for slightly more decks
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Card(u8);
impl Card {
	pub fn new(deck: u8, suit: Suit, CardValue(value): CardValue) -> Self {
		Self(deck << 6 | (suit as u8) << 4 | value)
	}
	pub fn value(&self) -> CardValue {
		let masked = self.0 & 0b1111;
		CardValue(masked)
	}
	pub fn suit(&self) -> Suit {
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
	pub fn is_red(&self) -> bool {
		self.0 & 0b010000 != 0
	}
	/// Is the suit shape spikey.  (Bouba/kiki)
	pub fn is_kiki(&self) -> bool {
		self.0 & 0b100000 != 0
	}
	pub fn deck(&self) -> u8 {
		self.0 >> 6
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Stack<const CAP: usize>(arrayvec::ArrayVec<Card, CAP>);
impl<const CAP: usize> Stack<CAP> {
	pub fn new() -> Self {
		Self(arrayvec::ArrayVec::new())
	}
}
impl Stack<52> {
	/// Generate a full deck of cards with the specified deck id.
	pub fn full_deck(deck: u8) -> Self {
		let mut stack = arrayvec::ArrayVec::new();
		for suit in Suit::SUITS {
			for value in 1..=13 {
				stack.push(Card::new(deck, suit, CardValue(value)));
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
pub struct Pile<const CAP: usize> {
	face_down: Stack<CAP>,
	face_up: Stack<CAP>,
}
impl<const CAP: usize> Pile<CAP> {
	pub fn new() -> Self {
		Self {
			face_down: Stack::new(),
			face_up: Stack::new(),
		}
	}
	pub fn new_face_down(stack: Stack<CAP>) -> Self {
		Self {
			face_down: stack,
			face_up: Stack::new(),
		}
	}
	pub fn flip_it_and_reverse_it(&mut self) {
		self.swap_up_down();
		self.face_down.reverse();
	}
	pub fn swap_up_down(&mut self) {
		core::mem::swap(&mut self.face_up, &mut self.face_down);
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
		let card = self.pop()?;
		if self.face_up.is_empty() {
			self.flip_up();
		}
		Some(card)
	}
	pub fn push(&mut self, card: Card) {
		self.face_up.push(card);
	}
	pub fn face_up(&self) -> &[Card] {
		&self.face_up
	}
	pub fn face_down(&self) -> &[Card] {
		&self.face_down
	}
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Session<G: Game> {
	seed: G,
	state: G,
	history: Vec<G::Instruction>,
}
impl<G: Game + Clone + Eq + core::hash::Hash> Session<G>
where
	G::Instruction: Clone + Eq + core::hash::Hash,
{
	pub fn new(state: G) -> Self {
		Self {
			seed: state.clone(),
			state,
			history: Vec::new(),
		}
	}
	pub fn state(&self) -> &G {
		&self.state
	}
	pub fn history(&self) -> &[G::Instruction] {
		&self.history
	}
	pub fn is_winnable(&self) -> Option<Vec<G::Instruction>> {
		let mut observed = std::collections::HashSet::new();
		struct StateMachine<G, P, I> {
			state: G,
			possible_instructions_iter: P,
			instruction: I,
		}
		let state = self.state.clone();
		let mut state = StateMachine {
			possible_instructions_iter: state.possible_instructions(),
			state,
			instruction: None,
		};
		let mut history = Vec::new();
		'outer: while !state.state.is_win() {
			observed.insert(state.state.clone());
			for instruction in &mut state.possible_instructions_iter {
				let mut next_state = state.state.clone();
				next_state.process_instruction(instruction.clone());
				if !observed.contains(&next_state) {
					let it = next_state.possible_instructions();
					history.push(core::mem::replace(
						&mut state,
						StateMachine {
							state: next_state,
							possible_instructions_iter: it,
							instruction: Some(instruction),
						},
					));
					continue 'outer;
				}
			}
			let Some(last_state) = history.pop() else {
				return None;
			};
			state = last_state;
		}
		Some(
			history
				.into_iter()
				.filter_map(|state| state.instruction)
				.collect(),
		)
	}
	pub fn undo(&mut self) {
		// replay the entire history of the game except one move
		self.history.pop();
		let mut state = self.seed.clone();
		for instruction in self.history() {
			state.process_instruction(instruction.clone());
		}
		self.state = state;
	}
}
impl<G: Game> Game for Session<G>
where
	G::Instruction: Clone,
{
	type Instruction = G::Instruction;
	fn possible_instructions(&self) -> impl Iterator<Item = Self::Instruction> + use<G> {
		self.state.possible_instructions()
	}
	fn is_instruction_valid(&self, instruction: Self::Instruction) -> bool {
		self.state.is_instruction_valid(instruction)
	}
	fn process_instruction(&mut self, instruction: Self::Instruction) {
		self.history.push(instruction.clone());
		self.state.process_instruction(instruction);
	}
	fn is_win(&self) -> bool {
		self.state.is_win()
	}
}
