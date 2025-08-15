#![warn(clippy::pedantic)]
#![allow(clippy::similar_names, clippy::must_use_candidate)]
mod bitboard;
mod errors;
mod magic;
mod piece;
mod utils;

pub mod board;
pub use board::Board;

mod move_gen;
pub use move_gen::MoveGen;

pub use magic::load_magics;
