//! This module provides the `Bitboard` struct, which is a 64-bit integer
//! representing the position of a kind of piece on a chessboard.
//! Bitboards provide an efficient way to represent and manipulate chess positions
//! through bitwise operations.
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Mul, Not, Shl, Shr};

#[derive(Copy, Clone)]
/// A `Bitboard` is a 64-bit integer where each bit represents the presence or absence
/// of a piece on a chessboard square.
///
/// The choosen layout is such that the least significant bit (LSB) corresponds to the
/// a1 square, and the most significant bit (MSB) corresponds to the h8 square, like so:
/// ```text
///  
/// 8 | 56 57 58 59 60 61 62 63
/// 7 | 48 49 50 51 52 53 54 55
/// 6 | 40 41 42 43 44 45 46 47
/// 5 | 32 33 34 35 36 37 38 39
/// 4 | 24 25 26 27 28 29 30 31
/// 3 | 16 17 18 19 20 21 22 23
/// 2 | 08 09 10 11 12 13 14 15
/// 1 | 00 01 02 03 04 05 06 07
///     a  b  c  d  e  f  g  h
/// ```
pub struct Bitboard(pub u64);

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in (0..8).rev() {
            for j in 0..8 {
                if self.0 & (1 << (8 * i + j)) != 0 {
                    s.push_str("1 ");
                } else {
                    s.push_str(". ");
                }
            }
            s.push('\n');
        }
        write!(f, "\n{s}")
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Bitboard) -> bool {
        *self == other.0
    }
}

impl Shl<usize> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Bitboard> for u64 {
    fn eq(&self, other: &Bitboard) -> bool {
        *self == other.0
    }
}

impl Mul<u64> for Bitboard {
    type Output = u64;

    fn mul(self, rhs: u64) -> Self::Output {
        self.0 * rhs
    }
}

impl BitAnd<u8> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 & u64::from(rhs))
    }
}

impl Bitboard {
    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    pub fn wrapping_mul(self, n: u64) -> u64 {
        self.0.wrapping_mul(n)
    }

    /// Finds the first set bit (least significant bit) in the bitboard,
    /// removing it from the bitboard, and returning its index.
    pub fn pop_lsb(&mut self) -> Option<usize> {
        if self.0 == 0 {
            return None;
        }

        let lsb_index = self.0.trailing_zeros() as usize;
        self.0 &= self.0 - 1;
        Some(lsb_index)
    }
}
