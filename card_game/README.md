Card Game
=========

`card_game` is a collection of algorithms, structs, and enums which are useful to implement card games.

## Example

```rust
use card_game::{Card, Deck, Rank, Stack, Suit};

// create a full deck (unshuffled)
let mut deck = Stack::full_deck(Deck::Deck1);

// inspect the top card
let card = deck.pop().unwrap();
assert_eq!(card, Card::new(Deck::Deck1, Suit::Diamonds, Rank::King));
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
