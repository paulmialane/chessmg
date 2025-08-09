use crate::bitboard::Bitboard;
use crate::utils::{Color, Kind};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
    pub bitboard: Bitboard,
}

impl Piece {
    #[must_use]
    pub fn create_initial(kind: Kind, color: Color) -> Self {
        let bitboard = match (kind, color) {
            (Kind::Pawn, Color::White) => Bitboard(0xFF00),
            (Kind::Knight, Color::White) => Bitboard(0x42),
            (Kind::Bishop, Color::White) => Bitboard(0x24),
            (Kind::Rook, Color::White) => Bitboard(0x81),
            (Kind::Queen, Color::White) => Bitboard(0x8),
            (Kind::King, Color::White) => Bitboard(0x10),

            (Kind::Pawn, Color::Black) => Bitboard(0x00FF_0000_0000_0000),
            (Kind::Knight, Color::Black) => Bitboard(0x4200_0000_0000_0000),
            (Kind::Bishop, Color::Black) => Bitboard(0x2400_0000_0000_0000),
            (Kind::Rook, Color::Black) => Bitboard(0x8100_0000_0000_0000),
            (Kind::Queen, Color::Black) => Bitboard(0x0800_0000_0000_0000),
            (Kind::King, Color::Black) => Bitboard(0x1000_0000_0000_0000),
        };
        Piece {
            kind,
            color,
            bitboard,
        }
    }

    #[must_use]
    pub fn get_char(&self) -> char {
        match (self.kind, self.color) {
            (Kind::King, Color::White) => '♔',
            (Kind::Queen, Color::White) => '♕',
            (Kind::Rook, Color::White) => '♖',
            (Kind::Bishop, Color::White) => '♗',
            (Kind::Knight, Color::White) => '♘',
            (Kind::Pawn, Color::White) => '♙',

            (Kind::King, Color::Black) => '♚',
            (Kind::Queen, Color::Black) => '♛',
            (Kind::Knight, Color::Black) => '♞',
            (Kind::Bishop, Color::Black) => '♝',
            (Kind::Rook, Color::Black) => '♜',
            (Kind::Pawn, Color::Black) => '♟',
        }
    }
}
