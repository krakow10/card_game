use crate::Klondike;
use card_game::Session;
#[test]
fn test_is_winnable() {
	// is winnable
	let is_winnable = Session::new_default(Klondike::new_random()).is_winnable();
	println!("is_winnable = {is_winnable:?}");
}
#[test]
fn test_klondike() {
	// create game session
	let game = Klondike::new_random();
	let mut session = Session::new_default(game);

	// is winnable
	let is_winnable = session.is_winnable();
	println!("is_winnable = {is_winnable:?}");

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

	println!("is_win = {is_win}");
}
