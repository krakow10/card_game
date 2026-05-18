Klondike
========

`klondike` is a pure-logic implementation of Klondike using `card_game`.  Graphics not included.

## Example

```rust
use card_game::{Session, Game};
use klondike::Klondike;

// create game session
let game = Klondike::with_seed(123);
let mut session = Session::new_default(game);

// play game a bit
while let Some(instruction) = session.state().get_auto_move() {
	session.process_instruction(instruction);

	// quit after 1000 moves
	if 1000 < session.stats().stats().moves() {
		break;
	}
}

// did win
let is_win = session.is_win();

// print session history
for (i, instruction) in session.history().iter().enumerate() {
	println!("move {i} = {instruction:?}");
}

println!("is_win = {is_win}");
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
