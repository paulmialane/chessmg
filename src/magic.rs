use crate::bitboard::Bitboard;
use crate::utils::{
    Kind, Square, EAST_RAY, NORTH_EAST_RAY, NORTH_RAY, NORTH_WEST_RAY, SOUTH_EAST_RAY, SOUTH_RAY,
    SOUTH_WEST_RAY, WEST_RAY,
};
use rand::random;
use std::array::from_fn;
use std::collections::HashMap;
use std::sync::LazyLock;

type MagicIndex = u16;

#[derive(Clone)]
pub struct MagicEntry {
    /// Maps magic indices to precomputed attack bitboards.
    pub attack_set: HashMap<MagicIndex, Bitboard>,

    /// The default attack set when there are no blockers.
    pub default_attack: Bitboard,

    /// The magic number used for hashing blocker configurations.
    pub magic: u64,

    /// The number of bits to shift after multiplying by magic.
    pub shift: u8,
}

// TODO: figure out where this function belongs
// TODO: Test the function
pub fn generate_rook_attack_mask(square: Square) -> Bitboard {
    let square: usize = square as usize;
    NORTH_RAY[square] | EAST_RAY[square] | SOUTH_RAY[square] | WEST_RAY[square]
}

// TODO: figure out where this function belongs
// TODO: Test the function
pub fn generate_bishop_attack_mask(square: Square) -> Bitboard {
    let square: usize = square as usize;
    NORTH_EAST_RAY[square]
        | NORTH_WEST_RAY[square]
        | SOUTH_EAST_RAY[square]
        | SOUTH_WEST_RAY[square]
}

// TODO: Test the function
pub fn enumerate_blockers(mask: Bitboard) -> Vec<Bitboard> {
    let mut bits = Vec::new();
    for i in 0..64 {
        if (mask >> i) & 1u8 != 0 {
            bits.push(i);
        }
    }

    let n = bits.len();
    let mut blockers = Vec::with_capacity(1 << n);

    for i in 0..(1 << n) {
        let mut b = 0;
        for (j, bit) in bits.iter().enumerate().take(n) {
            if (i >> j) & 1 != 0 {
                b |= 1u64 << bit;
            }
        }
        blockers.push(Bitboard(b));
    }

    blockers
}

pub fn compute_attack(square: Square, blockers: Bitboard, kind: Kind) -> Bitboard {
    let mut attacks = Bitboard(0);
    let (rank, file) = (square as u8 / 8, square as u8 % 8);

    let directions: &[(i32, i32)] = match kind {
        Kind::Rook => &[(-1, 0), (1, 0), (0, -1), (0, 1)], // vertical, horizontal
        Kind::Bishop => &[(-1, -1), (-1, 1), (1, -1), (1, 1)], // diagonals
        _ => todo!("Should return an error"),
    };

    for &(dr, df) in directions {
        let mut r = i32::from(rank) + dr;
        let mut f = i32::from(file) + df;

        while (0..8).contains(&r) && (0..8).contains(&f) {
            let sq = usize::try_from(r * 8 + f).unwrap();
            attacks = attacks | Bitboard(1u64 << sq);
            if (blockers >> sq) & 1 != 0 {
                break; // ray blocked
            }
            r += dr;
            f += df;
        }
    }

    attacks
}

impl MagicEntry {
    // TODO: impl mul on &Bitbloard to avoid Copying
    // TODO: Test function
    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn find_attack(&self, blockers: Bitboard) -> Bitboard {
        let magic_index = u16::try_from((blockers.wrapping_mul(self.magic)) >> self.shift).unwrap();
        *self
            .attack_set
            .get(&magic_index)
            .unwrap_or(&self.default_attack)
    }

    // TODO: Test function
    fn generate(square: Square, kind: Kind) -> MagicEntry {
        let mask = match kind {
            Kind::Rook => generate_rook_attack_mask(square),
            Kind::Bishop => generate_bishop_attack_mask(square),
            _ => todo!("Should output an error"),
        };
        let permutations = enumerate_blockers(mask);
        let shift = 64 - mask.count_ones();

        loop {
            // Can be replaced by loop to be sure
            // Here it is just to win time
            let magic = random::<u64>() & random::<u64>() & random::<u64>();
            let mut attack_set = HashMap::new();
            let mut success = true;

            for &blockers in &permutations {
                // Here, we use wrapping_mul because we're not sure the number can be represented
                // as a u16 otherwise
                let magic_index = u16::try_from((blockers.wrapping_mul(magic)) >> shift).unwrap();
                let attack = compute_attack(square, blockers, kind);

                if let Some(existing) = attack_set.get(&magic_index) {
                    if *existing != attack {
                        success = false;
                        break;
                    }
                } else {
                    attack_set.insert(magic_index, attack);
                }
            }

            if success {
                let default_attack = compute_attack(square, Bitboard(0), kind);
                return MagicEntry {
                    attack_set,
                    default_attack,
                    magic,
                    shift: u8::try_from(shift).unwrap(),
                };
            }
        }
    }
}

pub static ROOK_MAGICS: LazyLock<[MagicEntry; 64]> = LazyLock::new(|| {
    from_fn(|square| MagicEntry::generate(Square::from_usize(square), Kind::Rook))
});
pub static BISHOP_MAGICS: LazyLock<[MagicEntry; 64]> = LazyLock::new(|| {
    from_fn(|square| MagicEntry::generate(Square::from_usize(square), Kind::Bishop))
});
