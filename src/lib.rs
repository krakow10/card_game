pub mod card_game;
pub mod klondike;

#[cfg(test)]
mod test;

pub type Rng = rand::rngs::ThreadRng;

// // test readme
// #[doc = include_str!("../README.md")]
// #[cfg(doctest)]
// struct ReadmeDoctests;
