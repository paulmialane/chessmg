#![warn(clippy::pedantic)]
#![allow(clippy::similar_names, clippy::must_use_candidate)]
mod bitboard;
pub mod board;
mod errors;
mod magic;
mod move_gen;
mod piece;
mod utils;

pub use board::Board;
pub use magic::load_magics;
pub use move_gen::{Move, MoveGen};
pub use utils::{Color, Kind, Square};
