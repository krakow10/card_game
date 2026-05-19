use card_game::Session;
use klondike::Klondike;
#[test]
fn test_is_winnable() {
	// is winnable
	let is_winnable = Session::new_default(Klondike::with_seed(124)).is_winnable();
	if let Some(win_moves) = is_winnable {
		// for (i, ins) in win_moves.into_iter().enumerate() {
		// 	println!("{i} = {:?}", ins.instruction());
		// }
		println!("Game is winnable with {} moves", win_moves.len());
	} else {
		println!("Game is not winnable");
	}
}
