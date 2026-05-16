use card_game::card_game::{Card, Game, Pile, Session, Suit};
use card_game::klondike::{
	DstFoundation, DstTableau, Foundation, Klondike, KlondikeInstruction, KlondikePile,
	KlondikePileStack, SkipCards, Tableau, TableauStack,
};

use std::fmt::Display;
struct Displayed<T>(T);

impl Display for Displayed<&Card> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.0.value().get() {
			1 => write!(f, "A"),
			11 => write!(f, "J"),
			12 => write!(f, "Q"),
			13 => write!(f, "K"),
			other => write!(f, "{other}"),
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
			OptionalCard(None) => write!(f, "None"),
		}
	}
}

impl Display for Displayed<&Klondike> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// Stock
		let stock_count = self.0.state().stock().face_down().len();
		writeln!(f, "Stock: {stock_count}")?;

		// Hand
		let hand = self.0.state().stock().face_up().last();
		writeln!(f, "Hand: {}", OptionalCard(hand))?;

		// Foundations
		write!(
			f,
			"Foundations: {} {} {} {}",
			OptionalCard(self.0.state().foundation1().last()),
			OptionalCard(self.0.state().foundation2().last()),
			OptionalCard(self.0.state().foundation3().last()),
			OptionalCard(self.0.state().foundation4().last()),
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
				write!(f, "{}", Displayed(card))?;
			}
			writeln!(f)?;
			Ok(())
		}
		write_pile(f, self.0.state().tableau1(), 1)?;
		write_pile(f, self.0.state().tableau2(), 2)?;
		write_pile(f, self.0.state().tableau3(), 3)?;
		write_pile(f, self.0.state().tableau4(), 4)?;
		write_pile(f, self.0.state().tableau5(), 5)?;
		write_pile(f, self.0.state().tableau6(), 6)?;
		write_pile(f, self.0.state().tableau7(), 7)?;

		Ok(())
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
						if state.is_instruction_valid(instruction) {
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
		.is_instruction_valid(instruction)
		.then_some(instruction)
}

fn main() -> Result<(), std::io::Error> {
	let mut session = Session::new(Klondike::new_random_default());
	loop {
		// display game
		println!("{}", Displayed(session.state()));

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
				fn useless_moves(instruction: &KlondikeInstruction) -> bool {
					!matches!(
						instruction,
						// foundation -> foundation is a useless move
						KlondikeInstruction::DstFoundation(DstFoundation {
							src: KlondikePile::Foundation(_),
							..
						})
						// Tableau -> Tableau when not revealing a new card is _usually_ a useless move
						| KlondikeInstruction::DstTableau(DstTableau {
							src: KlondikePileStack::Tableau(TableauStack {
								skip_cards: SkipCards::Skip1
									| SkipCards::Skip2 | SkipCards::Skip3
									| SkipCards::Skip4 | SkipCards::Skip5
									| SkipCards::Skip6 | SkipCards::Skip7
									| SkipCards::Skip8 | SkipCards::Skip9
									| SkipCards::Skip10 | SkipCards::Skip11
									| SkipCards::Skip12,
								..
							}),
							..
						})
					)
				}
				if let Some(instruction) =
					session.possible_instructions().filter(useless_moves).next()
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
