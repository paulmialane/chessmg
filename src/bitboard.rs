use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Mul, Not, Shl, Shr};

// A bitboard represent the position of a certain kind of piece
// on the board. For instance, all the white pawns, or the blacks rooks.
#[allow(dead_code)]
#[derive(Copy, Clone)]
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

// Handy to compare u64 to bitboard and vice versa
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
        Bitboard(self.0 & (rhs as u64))
    }
}

impl Bitboard {
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn wrapping_mul(&self, n: u64) -> u64 {
        self.0.wrapping_mul(n)
    }

    pub fn pop_lsb(&mut self) -> Option<usize> {
        if self.0 == 0 {
            return None;
        }

        let lsb_index = self.0.trailing_zeros() as usize;
        self.0 &= self.0 - 1;
        Some(lsb_index)
    }
}
