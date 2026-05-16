mod card_game;
mod klondike;

pub type Rng = rand::rngs::ThreadRng;

use card_game::{Card, Game, Pile, Session, Suit};
use klondike::{
	InstructionSrc, Klondike, KlondikeInstruction, KlondikePileId, KlondikePileStack,
	KlondikeState, SkipCards,
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
struct NaiveInstruction {
	src: KlondikePileId,
	dst: KlondikePileId,
}
impl core::str::FromStr for NaiveInstruction {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let Parsed(src) = s.get(0..2).ok_or(Invalid)?.parse()?;
		let Parsed(dst) = s.get(3..5).ok_or(Invalid)?.parse()?;
		Ok(NaiveInstruction { src, dst })
	}
}
impl core::str::FromStr for Parsed<KlondikePileId> {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Parsed(match s {
			"st" => KlondikePileId::Stock,
			"t1" => KlondikePileId::Tableau1,
			"t2" => KlondikePileId::Tableau2,
			"t3" => KlondikePileId::Tableau3,
			"t4" => KlondikePileId::Tableau4,
			"t5" => KlondikePileId::Tableau5,
			"t6" => KlondikePileId::Tableau6,
			"t7" => KlondikePileId::Tableau7,
			"f1" => KlondikePileId::Foundation1,
			"f2" => KlondikePileId::Foundation2,
			"f3" => KlondikePileId::Foundation3,
			"f4" => KlondikePileId::Foundation4,
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
	Klondike(NaiveInstruction),
}
impl core::str::FromStr for SessionInstruction {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"new" | "n" => Self::New,
			"undo" | "u" => Self::Undo,
			"hint" | "h" => Self::Hint,
			"auto" | "a" => Self::Auto,
			"exit" => Self::Exit,
			"s" => Self::Stock,
			other => Self::Klondike(other.parse()?),
		})
	}
}

fn find_valid_instruction(
	state: &Klondike,
	naive_instruction: NaiveInstruction,
) -> Option<KlondikeInstruction> {
	const SKIP_LIST: [SkipCards; 13] = [
		SkipCards::Zero,
		SkipCards::One,
		SkipCards::Two,
		SkipCards::Three,
		SkipCards::Four,
		SkipCards::Five,
		SkipCards::Six,
		SkipCards::Seven,
		SkipCards::Eight,
		SkipCards::Nine,
		SkipCards::Ten,
		SkipCards::Eleven,
		SkipCards::Twelve,
	];
	let dst = naive_instruction.dst;
	let src = match naive_instruction.src {
		KlondikePileId::Tableau1 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau1(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Tableau2 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau2(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Tableau3 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau3(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Tableau4 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau4(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Tableau5 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau5(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Tableau6 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau6(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Tableau7 => {
			for skip in SKIP_LIST {
				let src = InstructionSrc::new(KlondikePileStack::Tableau7(skip));
				let instruction = KlondikeInstruction { src, dst };
				if state.is_instruction_valid(instruction) {
					return Some(instruction);
				}
			}
			return None;
		}
		KlondikePileId::Foundation1 => InstructionSrc::new(KlondikePileStack::Foundation1),
		KlondikePileId::Foundation2 => InstructionSrc::new(KlondikePileStack::Foundation2),
		KlondikePileId::Foundation3 => InstructionSrc::new(KlondikePileStack::Foundation3),
		KlondikePileId::Foundation4 => InstructionSrc::new(KlondikePileStack::Foundation4),
		KlondikePileId::Stock => InstructionSrc::new(KlondikePileStack::Stock),
	};
	let instruction = KlondikeInstruction { src, dst };
	state
		.is_instruction_valid(instruction)
		.then_some(instruction)
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
			SessionInstruction::Klondike(naive_instruction) => {
				if let Some(instruction) =
					find_valid_instruction(session.state(), naive_instruction)
				{
					session.process_instruction(instruction);
				} else {
					println!("Invalid move!");
				}
			}
		}
	}
}
