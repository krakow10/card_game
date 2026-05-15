#[test]
fn test_klondike() {
	use crate::Rng;
	use crate::card_game::{Game, Session};
	use crate::klondike::Klondike;

	// create game session
	let seed = Rng::default();
	let game = Klondike::new(seed.clone(), Default::default());
	let mut session = Session::new(seed, game);

	// is winnable
	let is_winnable = session.is_winnable().is_some();

	// play game
	while let Some(instruction) = session.possible_instructions().next() {
		session.process_instruction(instruction);
	}

	// did win
	let is_win = session.is_win();

	// print session history
	for (i, instruction) in session.history().iter().enumerate() {
		println!("move {i} = {instruction:?}");
	}

	println!("is_winnable = {is_winnable}");
	println!("is_win = {is_win}");
}
