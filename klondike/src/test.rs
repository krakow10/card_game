use crate::Klondike;
use card_game::Session;

#[test]
fn test_is_winnable() {
	// is winnable
	let solution_result = Session::new_default(Klondike::with_seed(124)).solve();
	if let Ok(Some(solution)) = solution_result {
		let win_moves = solution.clean_solution();
		// for (i, ins) in win_moves.into_iter().enumerate() {
		// 	println!("{i} = {:?}", ins.instruction());
		// }
		println!("Game is winnable with {} moves", win_moves.len());
	} else {
		println!("Game is not winnable");
	}
}

#[cfg(feature = "serde")]
#[test]
fn test_json() {
	let mut session = Session::new_default(Klondike::with_seed(124));
	let solution_result = session.solve();
	if let Ok(Some(solution)) = solution_result {
		for snapshot in solution.clean_solution() {
			session.process_instruction(snapshot.instruction().clone());
		}
	}
	let serialized = serde_json::to_string(&session).unwrap();
	println!("serialized = {serialized}");
	let round_trip_session: Session<Klondike> = serde_json::from_str(&serialized).unwrap();
	let serialized2 = serde_json::to_string(&round_trip_session).unwrap();
	assert_eq!(serialized, serialized2);

	insta::assert_snapshot!(serialized);
}

#[cfg(feature = "serde")]
#[test]
fn test_rmp() {
	let mut session = Session::new_default(Klondike::with_seed(124));
	let solution_result = session.solve();
	if let Ok(Some(solution)) = solution_result {
		for snapshot in solution.clean_solution() {
			session.process_instruction(snapshot.instruction().clone());
		}
	}
	let serialized = rmp_serde::to_vec(&session).unwrap();
	println!("serialized.len() = {}", serialized.len());
	let round_trip_session: Session<Klondike> = rmp_serde::from_slice(&serialized).unwrap();
	let serialized2 = rmp_serde::to_vec(&round_trip_session).unwrap();
	assert_eq!(serialized, serialized2);

	insta::assert_binary_snapshot!("save_rmp.bin", serialized);
}
