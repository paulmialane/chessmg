use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::magic::{
    generate_bishop_attack_mask, generate_rook_attack_mask, BISHOP_MAGICS, ROOK_MAGICS,
};
use crate::utils::{square_mask, Color, Kind, Square, CLEAR_FILE, CLEAR_RANK, MASK_RANK};

pub struct Move {
    pub piece_kind: Kind,
    pub piece_color: Color,
    pub from: Square,
    pub to: Square,
    pub casteling: bool,
    pub promoting_piece: Option<Kind>,
    pub double_push: bool,
    pub en_passant: bool,
    pub captured_piece: Option<Kind>,
}

impl Move {
    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn display(&self) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let s = Square::from_u8(u8::try_from(rank * 8 + file).unwrap());
                if s == self.to {
                    print!("# ");
                } else if s == self.from {
                    print!("o ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
        println!("  a b c d e f g h");
        print!("");
    }
}

pub struct MoveGen<'a> {
    pub board: &'a Board,
    pub pseudo_move_list: Vec<Move>,
    pub legal_move_list: Vec<Move>,
}

impl<'a> MoveGen<'a> {
    // The only pub function should be the following
    // For debuggind purposes, all functions are now pub

    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            pseudo_move_list: Vec::with_capacity(500),
            legal_move_list: Vec::with_capacity(500),
        }
    }

    pub fn get_legal_moves(&self) -> &Vec<Move> {
        &self.legal_move_list
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_legal_moves(&mut self) {
        self.gen_pseudo_moves();
        let old_items = std::mem::take(&mut self.pseudo_move_list);
        for m in old_items {
            let eat_king = m.captured_piece.is_some_and(|p| p == Kind::King);
            if m.casteling {
                let can_castle: bool = !match m.to {
                    Square::G1 => {
                        self.is_square_under_black_attack(Square::E1)
                            || self.is_square_under_black_attack(Square::F1)
                            || self.is_square_under_black_attack(Square::G1)
                    }
                    Square::C1 => {
                        self.is_square_under_black_attack(Square::E1)
                            || self.is_square_under_black_attack(Square::D1)
                            || self.is_square_under_black_attack(Square::C1)
                    }
                    Square::G8 => {
                        self.is_square_under_white_attack(Square::E8)
                            || self.is_square_under_white_attack(Square::F8)
                            || self.is_square_under_white_attack(Square::G8)
                    }
                    Square::C8 => {
                        self.is_square_under_white_attack(Square::E8)
                            || self.is_square_under_white_attack(Square::D8)
                            || self.is_square_under_white_attack(Square::C8)
                    }
                    _ => panic!(),
                };
                if !can_castle {
                    continue;
                }
            }
            let mut tmp_board: Board = self.board.clone();
            tmp_board.do_move(&m);
            // Skip adding this move if it results in moving into check
            if !tmp_board.is_in_check(self.board.to_move) && !eat_king {
                self.legal_move_list.push(m);
            }
        }
    }

    pub fn get_pseudo_moves(&self) -> &Vec<Move> {
        &self.pseudo_move_list
    }

    pub fn gen_pseudo_moves(&mut self) {
        match self.board.to_move {
            Color::White => self.gen_white_moves(),
            Color::Black => self.gen_black_moves(),
        }
    }

    pub fn gen_white_moves(&mut self) {
        self.gen_white_pawns_moves();
        self.gen_white_knight_moves();
        self.gen_white_rook_moves();
        self.gen_white_bishop_moves();
        self.gen_white_queen_moves();
        self.gen_white_king_moves();
    }

    pub fn gen_black_moves(&mut self) {
        self.gen_black_pawns_moves();
        self.gen_black_knight_moves();
        self.gen_black_rook_moves();
        self.gen_black_bishop_moves();
        self.gen_black_queen_moves();
        self.gen_black_king_moves();
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_pawn_single_move(&mut self) {
        let mut moved_pawns = self.board.white_pawn.bitboard << 8;
        let free_squares = !self.board.all_pieces();
        moved_pawns = moved_pawns & free_squares;

        let mut promotions: Bitboard = moved_pawns & MASK_RANK[7];
        moved_pawns = moved_pawns & !MASK_RANK[7];

        // Generate single non promotion moves
        while moved_pawns != 0 {
            // Safe to unwrap thanks to previous check
            let to = moved_pawns.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
        }

        // Generate promotions
        while promotions != 0 {
            // Safe to unwrap thanks to previous check
            let to = promotions.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Queen),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Rook),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Bishop),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Knight),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_pawn_double_move(&mut self) {
        let free_squares: Bitboard = !self.board.all_pieces();
        let single_pushes: Bitboard = (self.board.white_pawn.bitboard << 8) & free_squares;
        let mut double_pushes: Bitboard = (single_pushes << 8) & free_squares & MASK_RANK[3];

        while double_pushes != 0 {
            let to = double_pushes.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 16),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: true,
                en_passant: false,
                captured_piece: None,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_pawn_left_attack(&mut self) {
        let mut left_regular_attacks =
            (self.board.white_pawn.bitboard << 7) & self.board.all_black_pieces() & CLEAR_FILE[7];
        let mut left_attack_promotions = left_regular_attacks & MASK_RANK[7];
        left_regular_attacks = left_regular_attacks & CLEAR_RANK[7];

        let mut left_en_passant =
            (self.board.white_pawn.bitboard << 7) & self.board.get_en_passant() & CLEAR_FILE[7];

        while left_regular_attacks != 0 {
            let to = left_regular_attacks.pop_lsb().unwrap();

            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

            let m = Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece,
            };
            self.pseudo_move_list.push(m);
        }

        while left_attack_promotions != 0 {
            let to = left_attack_promotions.pop_lsb().unwrap();
            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Queen),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Rook),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Bishop),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Knight),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
        }

        if left_en_passant != 0 {
            let to = left_en_passant.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                captured_piece: Some(Kind::Pawn),
                en_passant: true,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_pawn_right_attack(&mut self) {
        let mut right_regular_attacks =
            (self.board.white_pawn.bitboard << 9) & self.board.all_black_pieces() & CLEAR_FILE[0];
        let mut right_attack_promotions = right_regular_attacks & MASK_RANK[7];
        right_regular_attacks = right_regular_attacks & CLEAR_RANK[7];

        let mut right_en_passant =
            (self.board.white_pawn.bitboard << 9) & self.board.get_en_passant() & CLEAR_FILE[0];

        while right_regular_attacks != 0 {
            let to = right_regular_attacks.pop_lsb().unwrap();

            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

            let m = Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece,
            };
            self.pseudo_move_list.push(m);
        }

        while right_attack_promotions != 0 {
            let to = right_attack_promotions.pop_lsb().unwrap();
            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Queen),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Rook),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Bishop),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Knight),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
        }

        if right_en_passant != 0 {
            let to = right_en_passant.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::White,
                from: Square::from_usize(to - 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                captured_piece: Some(Kind::Pawn),
                en_passant: true,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_pawn_single_move(&mut self) {
        let mut moved_pawns = self.board.black_pawn.bitboard >> 8;
        let free_squares = !self.board.all_pieces();
        moved_pawns = moved_pawns & free_squares;

        let mut promotions: Bitboard = moved_pawns & MASK_RANK[0];
        moved_pawns = moved_pawns & CLEAR_RANK[0];

        // Generate single non promotion moves
        while moved_pawns != 0 {
            // Safe to unwrap thanks to previous check
            let to = moved_pawns.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
        }

        // Generate promotions
        while promotions != 0 {
            // Safe to unwrap thanks to previous check
            let to = promotions.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Queen),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Rook),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Bishop),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 8),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Knight),
                double_push: false,
                en_passant: false,
                captured_piece: None,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_pawn_double_move(&mut self) {
        let free_squares: Bitboard = !self.board.all_pieces();
        let single_pushes: Bitboard = (self.board.black_pawn.bitboard >> 8) & free_squares;
        let mut double_pushes: Bitboard = (single_pushes >> 8) & free_squares & MASK_RANK[4];

        while double_pushes != 0 {
            let to = double_pushes.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 16),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: true,
                en_passant: false,
                captured_piece: None,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_pawn_left_attack(&mut self) {
        let mut left_regular_attacks =
            (self.board.black_pawn.bitboard >> 7) & self.board.all_white_pieces() & CLEAR_FILE[0];
        let mut left_attack_promotions = left_regular_attacks & MASK_RANK[0];
        left_regular_attacks = left_regular_attacks & CLEAR_RANK[0];

        let mut left_en_passant =
            (self.board.black_pawn.bitboard >> 7) & self.board.get_en_passant() & CLEAR_FILE[0];

        while left_regular_attacks != 0 {
            let to = left_regular_attacks.pop_lsb().unwrap();

            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

            let m = Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece,
            };
            self.pseudo_move_list.push(m);
        }

        while left_attack_promotions != 0 {
            let to = left_attack_promotions.pop_lsb().unwrap();
            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Queen),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Rook),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Bishop),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Knight),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
        }

        if left_en_passant != 0 {
            let to = left_en_passant.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 7),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                captured_piece: Some(Kind::Pawn),
                en_passant: true,
            });
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_pawn_right_attack(&mut self) {
        let mut left_regular_attacks =
            (self.board.black_pawn.bitboard >> 9) & self.board.all_white_pieces() & CLEAR_FILE[7];
        let mut left_attack_promotions = left_regular_attacks & MASK_RANK[0];
        left_regular_attacks = left_regular_attacks & CLEAR_RANK[0];

        let mut left_en_passant =
            (self.board.black_pawn.bitboard >> 9) & self.board.get_en_passant() & CLEAR_FILE[7];

        while left_regular_attacks != 0 {
            let to = left_regular_attacks.pop_lsb().unwrap();

            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

            let m = Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece,
            };
            self.pseudo_move_list.push(m);
        }

        while left_attack_promotions != 0 {
            let to = left_attack_promotions.pop_lsb().unwrap();
            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Queen),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Rook),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Bishop),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: Some(Kind::Knight),
                double_push: false,
                en_passant: false,
                captured_piece,
            });
        }

        if left_en_passant != 0 {
            let to = left_en_passant.pop_lsb().unwrap();
            self.pseudo_move_list.push(Move {
                piece_kind: Kind::Pawn,
                piece_color: Color::Black,
                from: Square::from_usize(to + 9),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                captured_piece: Some(Kind::Pawn),
                en_passant: true,
            });
        }
    }

    pub fn gen_white_pawns_moves(&mut self) {
        self.gen_white_pawn_single_move();
        self.gen_white_pawn_double_move();
        self.gen_white_pawn_left_attack();
        self.gen_white_pawn_right_attack();
    }

    pub fn gen_black_pawns_moves(&mut self) {
        self.gen_black_pawn_single_move();
        self.gen_black_pawn_double_move();
        self.gen_black_pawn_left_attack();
        self.gen_black_pawn_right_attack();
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_king_moves(&mut self) {
        // Square nums
        //     . . . . .
        //     . 1 2 3 .
        //     . 8 K 4 .
        //     . 7 6 5 .
        //     . . . . .

        let king_bitboard = self.board.white_king.bitboard;

        // We need to clip the h and a file of the king to calculate the sport 1, 3, 4, 5, 7 and 8
        // to avoid king teleportation to the other side of the board
        let king_clip_file_h = king_bitboard & CLEAR_FILE[7];
        let king_clip_file_a = king_bitboard & CLEAR_FILE[0];

        let spot1 = king_clip_file_a << 7;
        let spot2 = king_bitboard << 8;
        let spot3 = king_clip_file_h << 9;
        let spot4 = king_clip_file_h << 1;
        let spot5 = king_clip_file_h >> 7;
        let spot6 = king_bitboard >> 8;
        let spot7 = king_clip_file_a >> 9;
        let spot8 = king_clip_file_a >> 1;

        let moved_king = spot1 | spot2 | spot3 | spot4 | spot5 | spot6 | spot7 | spot8;

        let free_squares = !self.board.all_pieces();
        let mut no_attack = moved_king & free_squares;
        let mut attacks = moved_king & self.board.all_black_pieces();

        while no_attack != 0 {
            let to = no_attack.pop_lsb().unwrap();

            let m = Move {
                piece_kind: Kind::King,
                piece_color: Color::White,
                from: Square::from_usize(king_bitboard.clone().pop_lsb().unwrap()),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece: None,
            };
            self.pseudo_move_list.push(m);
        }

        while attacks != 0 {
            let to = attacks.pop_lsb().unwrap();

            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

            let m = Move {
                piece_kind: Kind::King,
                piece_color: Color::White,
                from: Square::from_usize(king_bitboard.clone().pop_lsb().unwrap()),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece,
            };
            self.pseudo_move_list.push(m);
        }

        if self.board.casteling_rights.white_kingside {
            let no_piece_on_f1 = self.board.get_piece(Square::F1).is_none();
            let no_piece_on_g1 = self.board.get_piece(Square::G1).is_none();
            let piece_on_h1 = self.board.get_piece(Square::H1);
            if no_piece_on_g1
                && no_piece_on_f1
                && piece_on_h1.is_some_and(|p| p.color == Color::White && p.kind == Kind::Rook)
            {
                let m = Move {
                    piece_kind: Kind::King,
                    piece_color: Color::White,
                    from: Square::E1,
                    to: Square::G1,
                    casteling: true,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece: None,
                };
                self.pseudo_move_list.push(m);
            }
        }
        if self.board.casteling_rights.white_queenside {
            let no_piece_on_b1 = self.board.get_piece(Square::B1).is_none();
            let no_piece_on_c1 = self.board.get_piece(Square::C1).is_none();
            let no_piece_on_d1 = self.board.get_piece(Square::D1).is_none();
            let piece_on_a1 = self.board.get_piece(Square::A1);
            if no_piece_on_b1
                && no_piece_on_c1
                && no_piece_on_d1
                && piece_on_a1.is_some_and(|p| p.color == Color::White && p.kind == Kind::Rook)
            {
                let m = Move {
                    piece_kind: Kind::King,
                    piece_color: Color::White,
                    from: Square::E1,
                    to: Square::C1,
                    casteling: true,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece: None,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_king_moves(&mut self) {
        // Square nums
        //     . . . . .
        //     . 1 2 3 .
        //     . 8 K 4 .
        //     . 7 6 5 .
        //     . . . . .

        let king_bitboard = self.board.black_king.bitboard;

        // We need to clip the h and a file of the king to calculate the sport 1, 3, 4, 5, 7 and 8
        // to avoid king teleportation to the other side of the board
        let king_clip_file_h = king_bitboard & CLEAR_FILE[7];
        let king_clip_file_a = king_bitboard & CLEAR_FILE[0];

        let spot1 = king_clip_file_a << 7;
        let spot2 = king_bitboard << 8;
        let spot3 = king_clip_file_h << 9;
        let spot4 = king_clip_file_h << 1;
        let spot5 = king_clip_file_h >> 7;
        let spot6 = king_bitboard >> 8;
        let spot7 = king_clip_file_a >> 9;
        let spot8 = king_clip_file_a >> 1;

        let moved_king = spot1 | spot2 | spot3 | spot4 | spot5 | spot6 | spot7 | spot8;

        let free_squares = !self.board.all_pieces();
        let mut no_attack = moved_king & free_squares;
        let mut attacks = moved_king & self.board.all_white_pieces();

        while no_attack != 0 {
            let to = no_attack.pop_lsb().unwrap();

            let m = Move {
                piece_kind: Kind::King,
                piece_color: Color::Black,
                from: Square::from_usize(king_bitboard.clone().pop_lsb().unwrap()),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece: None,
            };
            self.pseudo_move_list.push(m);
        }

        while attacks != 0 {
            let to = attacks.pop_lsb().unwrap();

            let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

            let m = Move {
                piece_kind: Kind::King,
                piece_color: Color::Black,
                from: Square::from_usize(king_bitboard.clone().pop_lsb().unwrap()),
                to: Square::from_usize(to),
                casteling: false,
                promoting_piece: None,
                double_push: false,
                en_passant: false,
                captured_piece,
            };
            self.pseudo_move_list.push(m);
        }

        if self.board.casteling_rights.black_kingside {
            let no_piece_on_f8 = self.board.get_piece(Square::F8).is_none();
            let no_piece_on_g8 = self.board.get_piece(Square::G8).is_none();
            let piece_on_h8 = self.board.get_piece(Square::H8);
            if no_piece_on_g8
                && no_piece_on_f8
                && piece_on_h8.is_some_and(|p| p.color == Color::Black && p.kind == Kind::Rook)
            {
                let m = Move {
                    piece_kind: Kind::King,
                    piece_color: Color::Black,
                    from: Square::E8,
                    to: Square::G8,
                    casteling: true,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece: None,
                };
                self.pseudo_move_list.push(m);
            }
        }
        if self.board.casteling_rights.black_queenside {
            let no_piece_on_b8 = self.board.get_piece(Square::B8).is_none();
            let no_piece_on_c8 = self.board.get_piece(Square::C8).is_none();
            let no_piece_on_d8 = self.board.get_piece(Square::D8).is_none();
            let piece_on_a8 = self.board.get_piece(Square::A8);
            if no_piece_on_b8
                && no_piece_on_c8
                && no_piece_on_d8
                && piece_on_a8.is_some_and(|p| p.color == Color::Black && p.kind == Kind::Rook)
            {
                let m = Move {
                    piece_kind: Kind::King,
                    piece_color: Color::Black,
                    from: Square::E8,
                    to: Square::C8,
                    casteling: true,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece: None,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    pub fn gen_knight_moves(&self, knight_loc: Bitboard) -> Bitboard {
        // Square nums
        //     . 8 . 1 .
        //     7 . . . 2
        //     . . K . .
        //     6 . . . 3
        //     . 5 . 4 .
        let knight_clip_file_h = knight_loc & CLEAR_FILE[7];
        let knight_clip_file_gh = knight_loc & CLEAR_FILE[6] & CLEAR_FILE[7];

        let knight_clip_file_a = knight_loc & CLEAR_FILE[0];
        let knight_clip_file_ab = knight_loc & CLEAR_FILE[1] & CLEAR_FILE[0];

        // The knight can move in 8 directions: 2 squares in one direction and 1 square in the other
        let spot1 = knight_clip_file_h << 17;
        let spot2 = knight_clip_file_gh << 10;
        let spot3 = knight_clip_file_gh >> 6;
        let spot4 = knight_clip_file_h >> 15;
        let spot5 = knight_clip_file_a >> 17;
        let spot6 = knight_clip_file_ab >> 10;
        let spot7 = knight_clip_file_ab << 6;
        let spot8 = knight_clip_file_a << 15;

        spot1 | spot2 | spot3 | spot4 | spot5 | spot6 | spot7 | spot8
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_knight_moves(&mut self) {
        let mut knights_bitboard = self.board.white_knight.bitboard;
        while knights_bitboard != 0 {
            let knight_pos = knights_bitboard.pop_lsb().unwrap();
            let knight_bitboard = square_mask(Square::from_usize(knight_pos));

            let moved_knight = self.gen_knight_moves(knight_bitboard);

            let free_squares = !self.board.all_pieces();
            let mut no_attack = moved_knight & free_squares;
            let mut attacks = moved_knight & self.board.all_black_pieces();

            while no_attack != 0 {
                let to = no_attack.pop_lsb().unwrap();

                let m = Move {
                    piece_kind: Kind::Knight,
                    piece_color: Color::White,
                    from: Square::from_usize(knight_bitboard.clone().pop_lsb().unwrap()),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece: None,
                };
                self.pseudo_move_list.push(m);
            }

            while attacks != 0 {
                let to = attacks.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Knight,
                    piece_color: Color::White,
                    from: Square::from_usize(knight_bitboard.clone().pop_lsb().unwrap()),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_knight_moves(&mut self) {
        let mut knights_bitboard = self.board.black_knight.bitboard;
        while knights_bitboard != 0 {
            let knight_pos = knights_bitboard.pop_lsb().unwrap();
            let knight_bitboard = square_mask(Square::from_usize(knight_pos));

            let moved_knight = self.gen_knight_moves(knight_bitboard);

            let free_squares = !self.board.all_pieces();
            let mut no_attack = moved_knight & free_squares;
            let mut attacks = moved_knight & self.board.all_white_pieces();

            while no_attack != 0 {
                let to = no_attack.pop_lsb().unwrap();

                let m = Move {
                    piece_kind: Kind::Knight,
                    piece_color: Color::Black,
                    from: Square::from_usize(knight_bitboard.clone().pop_lsb().unwrap()),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece: None,
                };
                self.pseudo_move_list.push(m);
            }

            while attacks != 0 {
                let to = attacks.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Knight,
                    piece_color: Color::Black,
                    from: Square::from_usize(knight_bitboard.clone().pop_lsb().unwrap()),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_bishop_moves(&mut self) {
        let mut bishops = self.board.white_bishop.bitboard;
        while bishops != 0 {
            let bishop_pos = bishops.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(bishop_pos))
                & !Bitboard(1 << bishop_pos);
            let mut moves =
                BISHOP_MAGICS[bishop_pos].find_attack(blockers) & !self.board.all_white_pieces();
            while moves != 0 {
                let to = moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Bishop,
                    piece_color: Color::White,
                    from: Square::from_usize(bishop_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_bishop_moves(&mut self) {
        let mut bishops = self.board.black_bishop.bitboard;
        while bishops != 0 {
            let bishop_pos = bishops.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(bishop_pos))
                & !Bitboard(1 << bishop_pos);
            let mut moves =
                BISHOP_MAGICS[bishop_pos].find_attack(blockers) & !self.board.all_black_pieces();
            while moves != 0 {
                let to = moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Bishop,
                    piece_color: Color::Black,
                    from: Square::from_usize(bishop_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_rook_moves(&mut self) {
        let mut rooks = self.board.white_rook.bitboard;
        while rooks != 0 {
            let rook_pos = rooks.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(rook_pos))
                & !Bitboard(1 << rook_pos);
            let mut moves =
                ROOK_MAGICS[rook_pos].find_attack(blockers) & !self.board.all_white_pieces();
            while moves != 0 {
                let to = moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Rook,
                    piece_color: Color::White,
                    from: Square::from_usize(rook_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_rook_moves(&mut self) {
        let mut rooks = self.board.black_rook.bitboard;
        while rooks != 0 {
            let rook_pos = rooks.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(rook_pos))
                & !Bitboard(1 << rook_pos);
            let mut moves =
                ROOK_MAGICS[rook_pos].find_attack(blockers) & !self.board.all_black_pieces();
            while moves != 0 {
                let to = moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Rook,
                    piece_color: Color::Black,
                    from: Square::from_usize(rook_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_white_queen_moves(&mut self) {
        let mut queens = self.board.white_queen.bitboard;
        while queens != 0 {
            let queen_pos = queens.pop_lsb().unwrap();
            let rook_blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let bishop_blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let mut bishop_moves = BISHOP_MAGICS[queen_pos].find_attack(bishop_blockers)
                & !self.board.all_white_pieces();
            let mut rook_moves =
                ROOK_MAGICS[queen_pos].find_attack(rook_blockers) & !self.board.all_white_pieces();
            while rook_moves != 0 {
                let to = rook_moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Queen,
                    piece_color: Color::White,
                    from: Square::from_usize(queen_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
            while bishop_moves != 0 {
                let to = bishop_moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Queen,
                    piece_color: Color::White,
                    from: Square::from_usize(queen_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    pub fn gen_black_queen_moves(&mut self) {
        let mut queens = self.board.black_queen.bitboard;
        while queens != 0 {
            let queen_pos = queens.pop_lsb().unwrap();
            let rook_blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let bishop_blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let mut bishop_moves = BISHOP_MAGICS[queen_pos].find_attack(bishop_blockers)
                & !self.board.all_black_pieces();
            let mut rook_moves =
                ROOK_MAGICS[queen_pos].find_attack(rook_blockers) & !self.board.all_black_pieces();
            while rook_moves != 0 {
                let to = rook_moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Queen,
                    piece_color: Color::Black,
                    from: Square::from_usize(queen_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
            while bishop_moves != 0 {
                let to = bishop_moves.pop_lsb().unwrap();

                let captured_piece = self.board.get_piece_kind(Square::from_usize(to));

                let m = Move {
                    piece_kind: Kind::Queen,
                    piece_color: Color::Black,
                    from: Square::from_usize(queen_pos),
                    to: Square::from_usize(to),
                    casteling: false,
                    promoting_piece: None,
                    double_push: false,
                    en_passant: false,
                    captured_piece,
                };
                self.pseudo_move_list.push(m);
            }
        }
    }

    fn is_square_under_white_attack(&self, square: Square) -> bool {
        let position = square_mask(square);

        // A bitboard representing all pawn left attack
        let pawn_left_attacks = (self.board.white_pawn.bitboard << 7) & CLEAR_FILE[7];
        let pawn_right_attacks = (self.board.white_pawn.bitboard << 9) & CLEAR_FILE[0];

        let king_bitboard = self.board.white_king.bitboard;

        let king_clip_file_h = king_bitboard & CLEAR_FILE[7];
        let king_clip_file_a = king_bitboard & CLEAR_FILE[0];

        let spot1 = king_clip_file_a << 7;
        let spot2 = king_bitboard << 8;
        let spot3 = king_clip_file_h << 9;
        let spot4 = king_clip_file_h << 1;
        let spot5 = king_clip_file_h >> 7;
        let spot6 = king_bitboard >> 8;
        let spot7 = king_clip_file_a >> 9;
        let spot8 = king_clip_file_a >> 1;

        let king_attacks = spot1 | spot2 | spot3 | spot4 | spot5 | spot6 | spot7 | spot8;
        let mut knight_attacks = Bitboard(0);
        let mut knights = self.board.white_knight.bitboard;
        while knights != 0 {
            let knight_pos = knights.pop_lsb().unwrap();
            let moves = self.gen_knight_moves(square_mask(Square::from_usize(knight_pos)));
            knight_attacks = knight_attacks | moves;
        }

        let mut bishop_attacks = Bitboard(0);

        let mut bishops = self.board.white_bishop.bitboard;
        while bishops != 0 {
            let bishop_pos = bishops.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(bishop_pos))
                & !Bitboard(1 << bishop_pos);
            let moves = BISHOP_MAGICS[bishop_pos].find_attack(blockers);
            bishop_attacks = bishop_attacks | moves;
        }

        let mut rook_attacks = Bitboard(0);
        let mut rooks = self.board.white_rook.bitboard;
        while rooks != 0 {
            let rook_pos = rooks.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(rook_pos))
                & !Bitboard(1 << rook_pos);
            let moves = ROOK_MAGICS[rook_pos].find_attack(blockers);
            rook_attacks = rook_attacks | moves;
        }

        let mut queen_attacks = Bitboard(0);
        let mut queens = self.board.white_queen.bitboard;
        while queens != 0 {
            let queen_pos = queens.pop_lsb().unwrap();
            let rook_blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let bishop_blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let bishop_moves = BISHOP_MAGICS[queen_pos].find_attack(bishop_blockers);
            let rook_moves = ROOK_MAGICS[queen_pos].find_attack(rook_blockers);
            queen_attacks = queen_attacks | rook_moves | bishop_moves;
        }

        position
            & (pawn_left_attacks
                | pawn_right_attacks
                | king_attacks
                | bishop_attacks
                | knight_attacks
                | rook_attacks
                | queen_attacks)
            != 0
    }

    fn is_square_under_black_attack(&self, square: Square) -> bool {
        let position = square_mask(square);

        // A bitboard representing all pawn left attack
        let pawn_left_attacks = (self.board.black_pawn.bitboard >> 7) & CLEAR_FILE[0];
        let pawn_right_attacks = (self.board.black_pawn.bitboard >> 9) & CLEAR_FILE[7];

        let king_bitboard = self.board.black_king.bitboard;

        let king_clip_file_h = king_bitboard & CLEAR_FILE[7];
        let king_clip_file_a = king_bitboard & CLEAR_FILE[0];

        let spot1 = king_clip_file_a << 7;
        let spot2 = king_bitboard << 8;
        let spot3 = king_clip_file_h << 9;
        let spot4 = king_clip_file_h << 1;
        let spot5 = king_clip_file_h >> 7;
        let spot6 = king_bitboard >> 8;
        let spot7 = king_clip_file_a >> 9;
        let spot8 = king_clip_file_a >> 1;

        let king_attacks = spot1 | spot2 | spot3 | spot4 | spot5 | spot6 | spot7 | spot8;
        let mut knight_attacks = Bitboard(0);
        let mut knights = self.board.black_knight.bitboard;
        while knights != 0 {
            let knight_pos = knights.pop_lsb().unwrap();
            let moves = self.gen_knight_moves(square_mask(Square::from_usize(knight_pos)));
            knight_attacks = knight_attacks | moves;
        }

        let mut bishop_attacks = Bitboard(0);

        let mut bishops = self.board.black_bishop.bitboard;
        while bishops != 0 {
            let bishop_pos = bishops.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(bishop_pos))
                & !Bitboard(1 << bishop_pos);
            let moves = BISHOP_MAGICS[bishop_pos].find_attack(blockers);
            bishop_attacks = bishop_attacks | moves;
        }

        let mut rook_attacks = Bitboard(0);
        let mut rooks = self.board.black_rook.bitboard;
        while rooks != 0 {
            let rook_pos = rooks.pop_lsb().unwrap();
            let blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(rook_pos))
                & !Bitboard(1 << rook_pos);
            let moves = ROOK_MAGICS[rook_pos].find_attack(blockers);
            rook_attacks = rook_attacks | moves;
        }

        let mut queen_attacks = Bitboard(0);
        let mut queens = self.board.black_queen.bitboard;
        while queens != 0 {
            let queen_pos = queens.pop_lsb().unwrap();
            let rook_blockers = self.board.all_pieces()
                & generate_rook_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let bishop_blockers = self.board.all_pieces()
                & generate_bishop_attack_mask(Square::from_usize(queen_pos))
                & !Bitboard(1 << queen_pos);
            let bishop_moves = BISHOP_MAGICS[queen_pos].find_attack(bishop_blockers);
            let rook_moves = ROOK_MAGICS[queen_pos].find_attack(rook_blockers);
            queen_attacks = queen_attacks | rook_moves | bishop_moves;
        }

        position
            & (pawn_left_attacks
                | pawn_right_attacks
                | king_attacks
                | bishop_attacks
                | knight_attacks
                | rook_attacks
                | queen_attacks)
            != 0
    }

    pub fn is_square_under_attack(&self, square: Square, by: Color) -> bool {
        match by {
            Color::White => self.is_square_under_white_attack(square),
            Color::Black => self.is_square_under_black_attack(square),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn wrapper(fen: &str, n_move: usize) {
        let board = Board::from_fen(fen).unwrap();
        let mut mg = MoveGen::new(&board);
        mg.gen_legal_moves();
        let v = mg.get_legal_moves();
        assert_eq!(v.len(), n_move);
    }

    fn perft(board: &Board, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let mut movegen = MoveGen::new(board);

        movegen.gen_legal_moves();
        for mv in movegen.get_legal_moves() {
            let mut new_board = board.clone();
            new_board.do_move(mv);
            nodes += perft(&new_board, depth - 1);
        }

        nodes
    }

    #[test]
    fn test_king_center() {
        wrapper("k7/8/8/8/3K4/8/8/8 w - - 0 1", 8);
    }

    #[test]
    fn test_king_a1() {
        wrapper("k7/8/8/8/8/8/8/K7 w - - 0 1", 3);
    }
    #[test]
    fn test_king_h1() {
        wrapper("k7/8/8/8/8/8/8/7K w - - 0 1", 3);
    }
    #[test]
    fn test_king_a8() {
        wrapper("K7/8/8/3k4/8/8/8/8 w - - 0 1", 3);
    }
    #[test]
    fn test_king_h8() {
        wrapper("7K/8/8/3k4/8/8/8/8 w - - 0 1", 3);
    }

    #[test]
    fn test_king_face_to_face() {
        wrapper("8/8/8/3k4/8/3K4/8/8 w - - 0 1", 5);
    }

    #[test]
    fn test_king_capture() {
        wrapper("k7/8/8/8/2b1r3/3K4/8/8 w - - 0 1", 4);
    }

    #[test]
    fn test_white_pawn_double_push() {
        wrapper("k7/8/8/8/8/8/4P3/K7 w - - 0 1", 5);
    }

    #[test]
    fn test_white_pawn_single_push() {
        wrapper("k7/8/8/8/4P3/8/8/K7 w - - 0 1", 4);
    }

    #[test]
    fn test_white_pawn_double_push_opposition() {
        wrapper("k7/8/8/8/4p3/8/4P3/K7 w - - 0 1", 4);
        wrapper("k7/8/8/4p3/8/8/4P3/K7 w - - 0 1", 5);
    }

    #[test]
    fn test_white_pawn_single_push_opposition() {
        wrapper("k7/8/8/4p3/4P3/8/8/K7 w - - 0 1", 3);
    }

    #[test]
    fn test_white_pawn_left_attack() {
        wrapper("k7/8/8/8/3p4/4P3/8/K7 w - - 0 1", 5);
    }

    #[test]
    fn test_white_pawn_right_attack() {
        wrapper("k7/8/8/8/5p2/4P3/8/K7 w - - 0 1", 5);
    }

    #[test]
    fn test_white_pawn_enpassant_1() {
        wrapper("k7/8/8/4Pp2/8/8/8/K7 w - f6 0 1", 5);
    }

    #[test]
    fn test_white_pawn_enpassant_2() {
        wrapper("k7/8/8/3pP3/8/8/8/K7 w - d6 0 1", 5);
    }

    #[test]
    fn test_white_pawn_promotion() {
        wrapper("k7/4P3/8/8/8/8/8/K7 w - - 0 1", 7);
    }

    #[test]
    fn test_white_pawn_promotion_blocked() {
        wrapper("k3p3/4P3/8/8/8/8/8/K7 w HAha - 0 1", 3);
    }

    #[test]
    fn test_white_pawn_promotion_attack() {
        wrapper("k4p2/4P3/8/8/8/8/8/K7 w HAha - 0 1", 11);
    }

    #[test]
    fn test_black_pawn_single_push() {
        wrapper("k7/8/4p3/8/8/8/8/K7 b - - 0 1", 4);
    }

    #[test]
    fn test_black_pawn_double_push() {
        wrapper("k7/4p3/8/8/8/8/8/K7 b - - 0 1", 5);
    }

    #[test]
    fn test_black_pawn_double_push_opposition() {
        wrapper("k7/4p3/8/4P3/8/8/8/K7 b HAha - 0 1", 4);
    }

    #[test]
    fn test_black_pawn_single_push_opposition() {
        wrapper("k7/8/4p3/4P3/8/8/8/K7 b - - 0 1", 3);
    }

    #[test]
    fn test_black_pawn_left_attack() {
        wrapper("k7/4p3/5P2/8/8/8/8/K7 b - - 0 1", 6);
    }

    #[test]
    fn test_black_pawn_right_attack() {
        wrapper("k7/8/4p3/3P4/8/8/8/K7 b HAha - 0 1", 5);
    }

    #[test]
    fn test_black_pawn_enpassant_1() {
        wrapper("k7/8/8/8/3Pp3/8/8/K7 b - d3 0 1", 5);
    }

    #[test]
    fn test_black_pawn_enpassant_2() {
        wrapper("k7/8/8/8/4pP2/8/8/K7 b - f3 0 1", 5);
    }

    #[test]
    fn test_black_pawn_promotion() {
        wrapper("k7/8/8/8/8/8/5p2/K7 b - - 0 1", 7);
    }

    #[test]
    fn test_black_pawn_promotion_blocked() {
        wrapper("k7/8/8/8/8/8/5p2/K4R2 b - - 0 1", 3);
    }

    #[test]
    fn test_black_pawn_promotion_attack() {
        wrapper("k7/8/8/8/8/8/5p2/K5R1 b HAha - 0 1", 11);
    }

    #[test]
    fn test_knight() {
        wrapper("k7/8/8/4N3/8/8/8/K7 w HAha - 0 1", 11);
    }

    #[test]
    fn test_knight_corner() {
        wrapper("k6N/8/8/8/8/8/8/K7 w HAha - 0 1", 5);
    }

    #[test]
    fn test_knight_friendly_piece() {
        wrapper("k7/8/8/8/8/8/2N5/K7 w HAha - 0 1", 8);
    }

    #[test]
    fn test_knight_captures() {
        wrapper("k7/8/3p4/8/4N3/8/8/K7 w HAha - 0 1", 11);
    }

    #[test]
    fn test_knight_captures2() {
        wrapper("k7/3r4/1N6/8/8/8/8/K7 w HAha - 0 1", 8);
    }

    #[test]
    fn test_rook() {
        wrapper("k7/8/8/4r3/8/8/8/K7 b - - 0 1", 17);
    }

    #[test]
    fn test_rook_friendly_piece() {
        wrapper("kr6/8/8/8/8/8/8/K7 b - - 0 1", 15);
    }

    #[test]
    fn test_rook_capture() {
        wrapper("k7/4r3/8/8/4P3/8/8/K7 b - - 0 1", 14);
    }

    #[test]
    fn test_bishop() {
        wrapper("k7/8/3b4/8/8/8/8/K7 b - - 0 1", 14);
    }

    #[test]
    fn test_bishop_friendly_piece() {
        wrapper("k7/8/8/8/4b3/8/8/K7 b - - 0 1", 15);
    }

    #[test]
    fn test_bishop_capture() {
        wrapper("k7/8/8/8/4b3/8/2R5/K7 b - - 0 1", 14);
    }

    #[test]
    fn test_queen() {
        wrapper("k7/8/8/8/8/3Q4/8/K7 w - - 0 1", 28);
    }

    #[test]
    fn test_queen_friendly_piece() {
        wrapper("k7/8/8/8/8/2Q5/8/K7 w - - 0 1", 27);
    }

    #[test]
    fn test_queen_captures() {
        wrapper("k7/8/2p5/8/8/2Q5/8/K7 w - - 0 1", 25);
    }

    #[test]
    fn test_castle_kingside() {
        wrapper("k7/8/8/8/8/8/8/4K2R w K - 0 1", 15);
    }

    #[test]
    fn test_castle_queenside() {
        wrapper("k7/8/8/8/8/8/8/R3K3 w HQ - 0 1", 15);
    }

    #[test]
    fn test_cant_castle_queenside() {
        wrapper("krr5/8/8/8/8/8/8/R3K3 w HQ - 0 1", 14);
    }

    #[test]
    fn test_king_not_into_check() {
        wrapper("k7/8/8/8/8/8/4p3/4K3 w - - 0 1", 3);
    }

    #[test]
    fn test_check_mate() {
        wrapper("k6b/Q7/8/8/8/8/8/R3K3 b Q - 0 1", 0);
    }

    #[test]
    fn test_perft1() {
        let b = Board::default();
        let p = perft(&b, 6);
        assert_eq!(p, 119_060_324);
    }
    #[test]
    fn test_perft2() {
        let b =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ")
                .unwrap();
        let p = perft(&b, 5);
        assert_eq!(p, 193_690_690);
    }
    #[test]
    fn test_perft3() {
        let b = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        let p = perft(&b, 6);
        assert_eq!(p, 11_030_083);
    }
    #[test]
    fn test_perft4() {
        let b = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
            .unwrap();
        let p = perft(&b, 6);
        assert_eq!(p, 706_045_033);
    }
    #[test]
    fn test_perft5() {
        let b =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
        let p = perft(&b, 5);
        assert_eq!(p, 89_941_194);
    }
    #[test]
    fn test_perft6() {
        let b = Board::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        )
        .unwrap();
        let p = perft(&b, 5);
        assert_eq!(p, 164_075_551);
    }
}
