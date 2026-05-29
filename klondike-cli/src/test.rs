use card_game::Session;
use klondike::Klondike;
#[test]
fn test_is_winnable() {
	// is winnable
	let solution_result = Session::new_default(Klondike::with_seed(124)).solve();
	if let Ok(Some(win_moves)) = solution_result {
		// for (i, ins) in win_moves.into_iter().enumerate() {
		// 	println!("{i} = {:?}", ins.instruction());
		// }
		println!("Game is winnable with {} moves", win_moves.len());
	} else {
		println!("Game is not winnable");
	}
}
