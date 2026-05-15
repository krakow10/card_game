use crate::card_game::{Card, Game, Stack};

struct Pile {
	face_down: Stack,
	face_up: Stack,
}
struct KlondikeConfig {}
struct KlondikeState {
	piles: [Pile; 14],
}
enum KlondikePileId {
	Stock,
	Hand,
	Foundation0,
	Foundation1,
	Foundation2,
	Foundation3,
	Tableau0,
	Tableau1,
	Tableau2,
	Tableau3,
	Tableau4,
	Tableau5,
	Tableau6,
	Tableau7,
}
struct KlondikeMove {
	src: KlondikePileId,
	dst: KlondikePileId,
}
pub struct Klondike {
	config: KlondikeConfig,
	state: KlondikeState,
}
