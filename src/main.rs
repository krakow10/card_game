mod card_game;
mod klondike;

pub type Rng = rand::rngs::ThreadRng;

use card_game::{Card, Game, Pile, Session, Suit};
use klondike::{
	InstructionSrc, Klondike, KlondikeInstruction, KlondikePileId, KlondikePileStack, SkipCards,
};

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
		let stock_count = self.state().stock().face_down().len();
		writeln!(f, "Stock: {stock_count}")?;

		// Hand
		let hand = self.state().stock().face_up().last();
		writeln!(f, "Hand: {}", OptionalCard(hand))?;

		// Foundations
		write!(
			f,
			"Foundations: {} {} {} {}",
			OptionalCard(self.state().foundation1().last()),
			OptionalCard(self.state().foundation2().last()),
			OptionalCard(self.state().foundation3().last()),
			OptionalCard(self.state().foundation4().last()),
		)?;
		writeln!(f)?;

		fn write_pile<const DN: usize, const UP: usize>(
			f: &mut std::fmt::Formatter<'_>,
			pile: &Pile<DN, UP>,
			pile_id: usize,
		) -> std::fmt::Result {
			write!(f, "T{} ", pile_id)?;
			for _ in pile.face_down() {
				write!(f, "]")?;
			}
			for card in pile.face_up() {
				write!(f, "{card}")?;
			}
			writeln!(f)?;
			Ok(())
		}
		write_pile(f, self.state().tableau1(), 1)?;
		write_pile(f, self.state().tableau2(), 2)?;
		write_pile(f, self.state().tableau3(), 3)?;
		write_pile(f, self.state().tableau4(), 4)?;
		write_pile(f, self.state().tableau5(), 5)?;
		write_pile(f, self.state().tableau6(), 6)?;
		write_pile(f, self.state().tableau7(), 7)?;

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
impl core::str::FromStr for Parsed<InstructionSrc> {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Parsed(match s {
			"ST" | "st" => InstructionSrc::new(KlondikePileStack::Stock),
			"T1" | "t1" => InstructionSrc::new(KlondikePileStack::Tableau1(SkipCards::Zero)),
			"T2" | "t2" => InstructionSrc::new(KlondikePileStack::Tableau2(SkipCards::Zero)),
			"T3" | "t3" => InstructionSrc::new(KlondikePileStack::Tableau3(SkipCards::Zero)),
			"T4" | "t4" => InstructionSrc::new(KlondikePileStack::Tableau4(SkipCards::Zero)),
			"T5" | "t5" => InstructionSrc::new(KlondikePileStack::Tableau5(SkipCards::Zero)),
			"T6" | "t6" => InstructionSrc::new(KlondikePileStack::Tableau6(SkipCards::Zero)),
			"T7" | "t7" => InstructionSrc::new(KlondikePileStack::Tableau7(SkipCards::Zero)),
			"F1" | "f1" => InstructionSrc::new(KlondikePileStack::Foundation1),
			"F2" | "f2" => InstructionSrc::new(KlondikePileStack::Foundation2),
			"F3" | "f3" => InstructionSrc::new(KlondikePileStack::Foundation3),
			"F4" | "f4" => InstructionSrc::new(KlondikePileStack::Foundation4),
			_ => return Err(Invalid),
		}))
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
			"F1" | "f1" => KlondikePileId::Foundation1,
			"F2" | "f2" => KlondikePileId::Foundation2,
			"F3" | "f3" => KlondikePileId::Foundation3,
			"F4" | "f4" => KlondikePileId::Foundation4,
			_ => return Err(Invalid),
		}))
	}
}

enum SessionInstruction {
	New,
	Undo,
	Hint,
	Auto,
	Stock,
	Exit,
	Klondike(KlondikeInstruction),
}
impl core::str::FromStr for SessionInstruction {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"NEW" | "new" | "n" => Self::New,
			"UNDO" | "undo" | "u" => Self::Undo,
			"HINT" | "hint" | "h" => Self::Hint,
			"AUTO" | "auto" | "a" => Self::Auto,
			"exit" => Self::Exit,
			"s" => Self::Stock,
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
			SessionInstruction::New => session = Session::new(Klondike::new_random_default()),
			SessionInstruction::Undo => session.undo(),
			SessionInstruction::Exit => break Ok(()),
			SessionInstruction::Hint => {
				for instruction in session.possible_instructions() {
					println!("{instruction:?}");
				}
			}
			SessionInstruction::Auto => {
				if let Some(instruction) = session.possible_instructions().next() {
					session.process_instruction(instruction);
				} else {
					println!("No valid moves!");
				}
			}
			SessionInstruction::Stock => session.process_instruction(KlondikeInstruction::stock()),
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
