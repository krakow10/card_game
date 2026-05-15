Card Game
=========

`card_game` is a library to implement card games.  Mainly interesting for the `Game` trait and the `Session` type.  Contains klondike as the reference implementation.

## Example

```rust
use card_game::card_game::{Session, Game};
use card_game::klondike::Klondike;

// create game session
let game = Klondike::new();
let mut session = Session::new(game);

// is winnable
let is_winnable = session.is_winnable();

// play game
while let Some(instruction) = session.enumerate_instructions() {
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
