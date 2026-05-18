use card_game::Game;
use klondike::{Klondike, KlondikeConfig, KlondikeStats, Rng};

const MAX_MOVES: usize = 250;

fn play_to_win(rng: &mut Rng) -> Option<KlondikeStats> {
	// create game session
	let mut game = Klondike::with_rng(rng);
	let mut stats = KlondikeStats::new();
	const CONFIG: KlondikeConfig = KlondikeConfig {
		draw_stock: klondike::DrawStockConfig::DrawOne,
	};
	// play game a bit
	while let Some(instruction) = game.get_auto_move()
		&& !game.is_win()
	{
		// quit before 250 moves
		if MAX_MOVES < stats.moves() + 1 {
			return None;
		}

		game.process_instruction(&mut stats, &CONFIG, instruction);
	}
	game.is_win().then_some(stats)
}
fn main() {
	use rand::SeedableRng;
	let mut rng = Rng::seed_from_u64(0);
	const GAMES: u32 = 100000;
	let mut wins = 0;
	let mut score_tally = [0usize; MAX_MOVES * 10 / 5];
	let mut recycle_tally = [0usize; MAX_MOVES];
	let mut moves_tally = [0usize; MAX_MOVES];
	for _ in 0..GAMES {
		if let Some(stats) = play_to_win(&mut rng) {
			wins += 1;
			score_tally[stats.score() / 5] += 1;
			recycle_tally[stats.recycle_count()] += 1;
			moves_tally[stats.moves()] += 1;
		}
	}
	println!("score_tally={score_tally:?}");
	println!("recycle_tally={recycle_tally:?}");
	println!("moves_tally={moves_tally:?}");
	println!("wins = {wins}/{GAMES} win_rate = {}%", wins * 100 / GAMES);
}
