use card_game::{Card, Game, Pile, Rank, Session, Suit};
use klondike::{
	DstFoundation, DstTableau, Foundation, Klondike, KlondikeConfig, KlondikeInstruction,
	KlondikePile, KlondikePileStack, SkipCards, Tableau, TableauStack,
};

use std::fmt::Display;
struct Displayed<T>(T);

impl Display for Displayed<&Card> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.0.rank() {
			Rank::Ace => write!(f, " A"),
			Rank::Jack => write!(f, " J"),
			Rank::Queen => write!(f, " Q"),
			Rank::King => write!(f, " K"),
			other => write!(f, "{:>2}", other as u8),
		}?;
		match self.0.suit() {
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
			&OptionalCard(Some(card)) => write!(f, "{}", Displayed(card)),
			OptionalCard(None) => write!(f, " []"),
		}
	}
}

impl Display for Displayed<&Klondike> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Stock
		let stock_count = self.0.state().stock().face_down().len();

		// Hand
		let hand = self.0.state().stock().face_up().last();

		// Foundations
		writeln!(f, " STOCK       F1  F2  F3  F4")?;
		write!(
			f,
			" {:>2} {}     {} {} {} {}",
			stock_count,
			OptionalCard(hand),
			OptionalCard(self.0.state().foundation1().last()),
			OptionalCard(self.0.state().foundation2().last()),
			OptionalCard(self.0.state().foundation3().last()),
			OptionalCard(self.0.state().foundation4().last()),
		)?;
		writeln!(f)?;

		writeln!(f, " T1  T2  T3  T4  T5  T6  T7")?;

		fn write_pile_card<const DN: usize, const UP: usize>(
			f: &mut std::fmt::Formatter<'_>,
			pile: &Pile<DN, UP>,
			row: usize,
		) -> std::fmt::Result {
			if let Some(_card) = pile.face_down().get(row) {
				return write!(f, " ⎾⏋"); // └┘ ⨽⨼ ⫭⫬
			}
			let Some(row) = row.checked_sub(pile.face_down().len()) else {
				return write!(f, "   ");
			};
			if let Some(card) = pile.face_up().get(row) {
				return write!(f, "{}", Displayed(card));
			}
			write!(f, "   ")
		}

		fn write_row(
			f: &mut std::fmt::Formatter<'_>,
			game: &Klondike,
			row: usize,
		) -> std::fmt::Result {
			write_pile_card(f, game.state().tableau1(), row)?;
			write!(f, " ")?;
			write_pile_card(f, game.state().tableau2(), row)?;
			write!(f, " ")?;
			write_pile_card(f, game.state().tableau3(), row)?;
			write!(f, " ")?;
			write_pile_card(f, game.state().tableau4(), row)?;
			write!(f, " ")?;
			write_pile_card(f, game.state().tableau5(), row)?;
			write!(f, " ")?;
			write_pile_card(f, game.state().tableau6(), row)?;
			write!(f, " ")?;
			write_pile_card(f, game.state().tableau7(), row)?;
			writeln!(f)
		}

		for row in 0..7 + 13 {
			write_row(f, self.0, row)?;
		}

		Ok(())
	}
}

struct DisplayStats<'a>(&'a Session<Klondike>);
impl Display for DisplayStats<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"recycles: {} moves: {} undos: {} score:{}",
			self.0.stats().stats().recycle_count(),
			self.0.stats().stats().moves(),
			self.0.stats().undos(),
			self.0.state().score(self.0.stats(), self.0.config()),
		)
	}
}

#[derive(Debug)]
struct Invalid;
struct Parsed<T>(T);
struct NaiveInstruction {
	src: KlondikePile,
	dst: KlondikePile,
}
impl core::str::FromStr for NaiveInstruction {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let Parsed(src) = s.get(0..2).ok_or(Invalid)?.parse()?;
		let Parsed(dst) = s.get(3..5).ok_or(Invalid)?.parse()?;
		Ok(NaiveInstruction { src, dst })
	}
}
impl core::str::FromStr for Parsed<KlondikePile> {
	type Err = Invalid;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Parsed(match s {
			"st" => KlondikePile::Stock,
			"t1" => KlondikePile::Tableau(Tableau::Tableau1),
			"t2" => KlondikePile::Tableau(Tableau::Tableau2),
			"t3" => KlondikePile::Tableau(Tableau::Tableau3),
			"t4" => KlondikePile::Tableau(Tableau::Tableau4),
			"t5" => KlondikePile::Tableau(Tableau::Tableau5),
			"t6" => KlondikePile::Tableau(Tableau::Tableau6),
			"t7" => KlondikePile::Tableau(Tableau::Tableau7),
			"f1" => KlondikePile::Foundation(Foundation::Foundation1),
			"f2" => KlondikePile::Foundation(Foundation::Foundation2),
			"f3" => KlondikePile::Foundation(Foundation::Foundation3),
			"f4" => KlondikePile::Foundation(Foundation::Foundation4),
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
			"auto" | "a" | "" => Self::Auto,
			"exit" => Self::Exit,
			"s" => Self::Stock,
			other => Self::Klondike(other.parse()?),
		})
	}
}

fn find_valid_instruction(
	config: &KlondikeConfig,
	state: &Klondike,
	naive_instruction: NaiveInstruction,
) -> Option<KlondikeInstruction> {
	const SKIP_LIST: [SkipCards; 13] = [
		SkipCards::Skip0,
		SkipCards::Skip1,
		SkipCards::Skip2,
		SkipCards::Skip3,
		SkipCards::Skip4,
		SkipCards::Skip5,
		SkipCards::Skip6,
		SkipCards::Skip7,
		SkipCards::Skip8,
		SkipCards::Skip9,
		SkipCards::Skip10,
		SkipCards::Skip11,
		SkipCards::Skip12,
	];
	let instruction = match (naive_instruction.dst, naive_instruction.src) {
		(KlondikePile::Tableau(tableau), src) => {
			let src = match src {
				KlondikePile::Tableau(src_tableau) => {
					for skip_cards in SKIP_LIST {
						let src = KlondikePileStack::Tableau(TableauStack {
							tableau: src_tableau,
							skip_cards,
						});
						let instruction =
							KlondikeInstruction::DstTableau(DstTableau { tableau, src });
						if state.is_instruction_valid(config, instruction) {
							return Some(instruction);
						}
					}
					return None;
				}
				KlondikePile::Stock => KlondikePileStack::Stock,
				KlondikePile::Foundation(foundation) => KlondikePileStack::Foundation(foundation),
			};
			KlondikeInstruction::DstTableau(DstTableau { tableau, src })
		}
		(KlondikePile::Stock, KlondikePile::Stock) => KlondikeInstruction::RotateStock,
		(KlondikePile::Foundation(foundation), src) => {
			KlondikeInstruction::DstFoundation(DstFoundation { foundation, src })
		}
		_ => return None,
	};
	state
		.is_instruction_valid(config, instruction)
		.then_some(instruction)
}

fn main() -> Result<(), std::io::Error> {
	use rand::RngExt;
	let mut rng = rand::rng();
	// seed from cli argument
	let mut seed = if let Some(seed) = std::env::args().nth(1) {
		seed.parse().expect("Invalid u64 seed")
	} else {
		rng.random()
	};
	let mut session = Session::new_default(Klondike::with_seed(seed));
	let mut input = String::new();
	loop {
		// display stats
		println!("seed: {seed} ");
		println!("{}", DisplayStats(&session));
		// display game
		println!("{}", Displayed(session.state().state()));

		// parse input
		input.clear();
		std::io::stdin().read_line(&mut input)?;
		let Ok(instruction) = input.trim().parse() else {
			println!("Invalid instruction.");
			continue;
		};

		// run game
		match instruction {
			SessionInstruction::New => {
				seed = rng.random();
				session = Session::new_default(Klondike::with_seed(seed))
			}
			SessionInstruction::Undo => session.undo(),
			SessionInstruction::Exit => break Ok(()),
			SessionInstruction::Hint => {
				for instruction in session.possible_instructions() {
					println!("{instruction:?}");
				}
			}
			SessionInstruction::Auto => {
				if let Some(instruction) = session
					.state()
					.state()
					.get_auto_move(&session.config().inner)
				{
					session.process_instruction(instruction);
				} else {
					println!("No valid moves!");
				}
			}
			SessionInstruction::Stock => {
				session.process_instruction(KlondikeInstruction::RotateStock)
			}
			SessionInstruction::Klondike(naive_instruction) => {
				if let Some(instruction) = find_valid_instruction(
					&session.config().inner,
					session.state().state(),
					naive_instruction,
				) {
					session.process_instruction(instruction);
				} else {
					println!("Invalid move!");
				}
			}
		}
	}
}
