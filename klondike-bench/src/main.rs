use card_game::Game;
use klondike::{Klondike, KlondikeConfig, KlondikeStats, Rng};

fn play_to_win(rng: &mut Rng) -> bool {
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
		game.process_instruction(&mut stats, &CONFIG, instruction);

		// quit after 250 moves
		if 250 < stats.moves() {
			return false;
		}
	}
	game.is_win()
}
fn main() {
	use rand::SeedableRng;
	let mut rng = Rng::seed_from_u64(0);
	const GAMES: u32 = 10000;
	let wins: u32 = (0..GAMES).map(|_| play_to_win(&mut rng) as u32).sum();
	println!("wins = {wins}/{GAMES} win_rate = {}%", wins * 100 / GAMES);
}
