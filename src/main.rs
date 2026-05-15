mod card_game;
mod klondike;

pub type Rng = rand::rngs::ThreadRng;

use card_game::{Card, Game, Session, Suit};
use klondike::{Klondike, KlondikeInstruction, KlondikePileId};

use std::fmt::Display;

impl Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.value().get() {
			1 => write!(f, "A"),
			11 => write!(f, "J"),
			12 => write!(f, "Q"),
			13 => write!(f, "K"),
			other => write!(f, "{other}"),
		}?;
		match self.suit() {
			Suit::Spades => write!(f, "♠"),
			Suit::Hearts => write!(f, "♡"),
			Suit::Clubs => write!(f, "♣"),
			Suit::Diamonds => write!(f, "♢"),
		}
	}
}

struct OptionalCard<'a>(Option<&'a Card>);
impl Display for OptionalCard<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			OptionalCard(Some(card)) => write!(f, "{card}"),
			OptionalCard(None) => write!(f, "None"),
		}
	}
}

impl Display for Klondike {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Stock
		let stock_count = self.pile(KlondikePileId::Stock).face_down().len();
		writeln!(f, "Stock: {stock_count}")?;

		// Hand
		let hand = self.pile(KlondikePileId::Stock).face_up().last();
		writeln!(f, "Hand: {}", OptionalCard(hand))?;

		// Foundations
		write!(
			f,
			"Foundations: {} {} {} {}",
			OptionalCard(self.pile(KlondikePileId::Foundation1).face_up().last()),
			OptionalCard(self.pile(KlondikePileId::Foundation2).face_up().last()),
			OptionalCard(self.pile(KlondikePileId::Foundation3).face_up().last()),
			OptionalCard(self.pile(KlondikePileId::Foundation4).face_up().last()),
		)?;
		writeln!(f)?;

		for (i, tableau) in [
			KlondikePileId::Tableau1,
			KlondikePileId::Tableau2,
			KlondikePileId::Tableau3,
			KlondikePileId::Tableau4,
			KlondikePileId::Tableau5,
			KlondikePileId::Tableau6,
			KlondikePileId::Tableau7,
			KlondikePileId::Tableau8,
		]
		.into_iter()
		.enumerate()
		{
			write!(f, "T{} ", i + 1)?;
			let pile = self.pile(tableau);
			for _ in pile.face_down() {
				write!(f, "]")?;
			}
			for card in pile.face_up() {
				write!(f, "{card}")?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

#[derive(Debug)]
struct Invalid;
struct Parsed<T>(T);
impl core::str::FromStr for Parsed<KlondikeInstruction> {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let Parsed(src) = s.get(0..2).ok_or(Invalid)?.parse()?;
		let Parsed(dst) = s.get(3..5).ok_or(Invalid)?.parse()?;
		Ok(Parsed(KlondikeInstruction { src, dst }))
	}
}
impl core::str::FromStr for Parsed<KlondikePileId> {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Parsed(match s {
			"ST" | "st" => KlondikePileId::Stock,
			"T1" | "t1" => KlondikePileId::Tableau1,
			"T2" | "t2" => KlondikePileId::Tableau2,
			"T3" | "t3" => KlondikePileId::Tableau3,
			"T4" | "t4" => KlondikePileId::Tableau4,
			"T5" | "t5" => KlondikePileId::Tableau5,
			"T6" | "t6" => KlondikePileId::Tableau6,
			"T7" | "t7" => KlondikePileId::Tableau7,
			"T8" | "t8" => KlondikePileId::Tableau8,
			"F1" | "f1" => KlondikePileId::Foundation1,
			"F2" | "f2" => KlondikePileId::Foundation2,
			"F3" | "f3" => KlondikePileId::Foundation3,
			"F4" | "f4" => KlondikePileId::Foundation4,
			_ => return Err(Invalid),
		}))
	}
}

enum SessionInstruction {
	Undo,
	Hint,
	Klondike(KlondikeInstruction),
}
impl core::str::FromStr for SessionInstruction {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"UNDO" | "undo" => Self::Undo,
			"HINT" | "hint" => Self::Hint,
			other => {
				let Parsed(ki) = other.parse()?;
				Self::Klondike(ki)
			}
		})
	}
}

fn main() -> Result<(), std::io::Error> {
	let mut session = Session::new(Klondike::new_random_default());
	loop {
		// display game
		println!("{}", session.state());

		// parse input
		let mut input = String::new();
		std::io::stdin().read_line(&mut input)?;
		let Ok(instruction) = input.trim().parse() else {
			println!("Invalid instruction.");
			continue;
		};

		// run game
		match instruction {
			SessionInstruction::Undo => session.undo(),
			SessionInstruction::Hint => {
				for instruction in session.possible_instructions() {
					println!("{instruction:?}");
				}
			}
			SessionInstruction::Klondike(instruction) => {
				if session.is_instruction_valid(instruction) {
					session.process_instruction(instruction);
				} else {
					println!("Invalid move!");
				}
			}
		}
	}
}
