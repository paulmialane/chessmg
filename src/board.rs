use crate::bitboard::Bitboard;
use crate::errors::ChessMgError;
use crate::errors::ChessMgError::InvalidFEN;
use crate::move_gen::{Move, MoveGen};
use crate::piece::Piece;
use crate::utils::{square_mask, Casteling, Color, Kind, Square};
use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
pub struct Board {
    // Who is it to move (White/Black)
    pub to_move: Color,

    // The placement of the White pieces
    pub white_pawn: Piece,
    pub white_knight: Piece,
    pub white_bishop: Piece,
    pub white_rook: Piece,
    pub white_queen: Piece,
    pub white_king: Piece,

    // The placement of the Black pieces
    pub black_pawn: Piece,
    pub black_knight: Piece,
    pub black_bishop: Piece,
    pub black_rook: Piece,
    pub black_queen: Piece,
    pub black_king: Piece,

    // Who can castle
    pub casteling_rights: Casteling,

    // Is there a `En Passant` square
    pub en_passant: Option<Square>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            to_move: Color::White,
            white_pawn: Piece::create_initial(Kind::Pawn, Color::White),
            white_knight: Piece::create_initial(Kind::Knight, Color::White),
            white_bishop: Piece::create_initial(Kind::Bishop, Color::White),
            white_rook: Piece::create_initial(Kind::Rook, Color::White),
            white_queen: Piece::create_initial(Kind::Queen, Color::White),
            white_king: Piece::create_initial(Kind::King, Color::White),
            black_pawn: Piece::create_initial(Kind::Pawn, Color::Black),
            black_knight: Piece::create_initial(Kind::Knight, Color::Black),
            black_bishop: Piece::create_initial(Kind::Bishop, Color::Black),
            black_rook: Piece::create_initial(Kind::Rook, Color::Black),
            black_queen: Piece::create_initial(Kind::Queen, Color::Black),
            black_king: Piece::create_initial(Kind::King, Color::Black),
            casteling_rights: Casteling::default(),
            en_passant: None,
        }
    }
}

impl fmt::Display for Board {
    // Used to display a board in a formatter
    // Very useful to debug
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let piece_ref: Option<&Piece> =
                    self.get_piece(Square::from_u8(u8::try_from(rank * 8 + file).unwrap()));
                let symbol = match piece_ref {
                    Some(p) => p.get_char(),
                    None => '.',
                };
                write!(f, "{symbol} ")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        write!(f, "")
    }
}

impl Board {
    pub fn get_piece(&self, square: Square) -> Option<&Piece> {
        let square_mask: Bitboard = square_mask(square);
        if (self.white_pawn.bitboard & square_mask) != 0 {
            Some(&self.white_pawn)
        } else if (self.white_knight.bitboard & square_mask) != 0 {
            Some(&self.white_knight)
        } else if (self.white_bishop.bitboard & square_mask) != 0 {
            Some(&self.white_bishop)
        } else if (self.white_rook.bitboard & square_mask) != 0 {
            Some(&self.white_rook)
        } else if (self.white_queen.bitboard & square_mask) != 0 {
            Some(&self.white_queen)
        } else if (self.white_king.bitboard & square_mask) != 0 {
            Some(&self.white_king)
        } else if (self.black_pawn.bitboard & square_mask) != 0 {
            Some(&self.black_pawn)
        } else if (self.black_knight.bitboard & square_mask) != 0 {
            Some(&self.black_knight)
        } else if (self.black_bishop.bitboard & square_mask) != 0 {
            Some(&self.black_bishop)
        } else if (self.black_rook.bitboard & square_mask) != 0 {
            Some(&self.black_rook)
        } else if (self.black_queen.bitboard & square_mask) != 0 {
            Some(&self.black_queen)
        } else if (self.black_king.bitboard & square_mask) != 0 {
            Some(&self.black_king)
        } else {
            None
        }
    }

    fn zero() -> Self {
        Board {
            to_move: Color::White,
            white_pawn: Piece {
                kind: Kind::Pawn,
                color: Color::White,
                bitboard: Bitboard(0),
            },
            white_knight: Piece {
                kind: Kind::Knight,
                color: Color::White,
                bitboard: Bitboard(0),
            },
            white_bishop: Piece {
                kind: Kind::Bishop,
                color: Color::White,
                bitboard: Bitboard(0),
            },
            white_rook: Piece {
                kind: Kind::Rook,
                color: Color::White,
                bitboard: Bitboard(0),
            },
            white_queen: Piece {
                kind: Kind::Queen,
                color: Color::White,
                bitboard: Bitboard(0),
            },
            white_king: Piece {
                kind: Kind::King,
                color: Color::White,
                bitboard: Bitboard(0),
            },

            black_pawn: Piece {
                kind: Kind::Pawn,
                color: Color::Black,
                bitboard: Bitboard(0),
            },
            black_knight: Piece {
                kind: Kind::Knight,
                color: Color::Black,
                bitboard: Bitboard(0),
            },
            black_bishop: Piece {
                kind: Kind::Bishop,
                color: Color::Black,
                bitboard: Bitboard(0),
            },
            black_rook: Piece {
                kind: Kind::Rook,
                color: Color::Black,
                bitboard: Bitboard(0),
            },
            black_queen: Piece {
                kind: Kind::Queen,
                color: Color::Black,
                bitboard: Bitboard(0),
            },
            black_king: Piece {
                kind: Kind::King,
                color: Color::Black,
                bitboard: Bitboard(0),
            },

            casteling_rights: Casteling {
                white_kingside: false,
                white_queenside: false,
                black_kingside: false,
                black_queenside: false,
            },

            en_passant: None,
        }
    }

    pub fn all_white_pieces(&self) -> Bitboard {
        self.white_pawn.bitboard
            | self.white_knight.bitboard
            | self.white_bishop.bitboard
            | self.white_rook.bitboard
            | self.white_queen.bitboard
            | self.white_king.bitboard
    }

    pub fn all_black_pieces(&self) -> Bitboard {
        self.black_pawn.bitboard
            | self.black_knight.bitboard
            | self.black_bishop.bitboard
            | self.black_rook.bitboard
            | self.black_queen.bitboard
            | self.black_king.bitboard
    }

    pub fn all_pieces(&self) -> Bitboard {
        self.all_white_pieces() | self.all_black_pieces()
    }

    pub fn get_piece_kind(&self, square: Square) -> Option<Kind> {
        let square_mask: Bitboard = square_mask(square);
        if (self.white_pawn.bitboard & square_mask) != 0 {
            Some(Kind::Pawn)
        } else if (self.white_knight.bitboard & square_mask) != 0 {
            Some(Kind::Knight)
        } else if (self.white_bishop.bitboard & square_mask) != 0 {
            Some(Kind::Bishop)
        } else if (self.white_rook.bitboard & square_mask) != 0 {
            Some(Kind::Rook)
        } else if (self.white_queen.bitboard & square_mask) != 0 {
            Some(Kind::Queen)
        } else if (self.white_king.bitboard & square_mask) != 0 {
            Some(Kind::King)
        } else if (self.black_pawn.bitboard & square_mask) != 0 {
            Some(Kind::Pawn)
        } else if (self.black_knight.bitboard & square_mask) != 0 {
            Some(Kind::Knight)
        } else if (self.black_bishop.bitboard & square_mask) != 0 {
            Some(Kind::Bishop)
        } else if (self.black_rook.bitboard & square_mask) != 0 {
            Some(Kind::Rook)
        } else if (self.black_queen.bitboard & square_mask) != 0 {
            Some(Kind::Queen)
        } else if (self.black_king.bitboard & square_mask) != 0 {
            Some(Kind::King)
        } else {
            None
        }
    }

    pub fn get_en_passant(&self) -> Bitboard {
        match self.en_passant {
            None => Bitboard(0),
            Some(square) => square_mask(square),
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "It is not suppose to panic")]
    pub fn is_in_check(&self, color: Color) -> bool {
        match color {
            Color::White => {
                let king_square =
                    Square::from_usize(self.white_king.bitboard.clone().pop_lsb().unwrap());
                let mg = MoveGen {
                    board: self,
                    pseudo_move_list: Vec::new(),
                    legal_move_list: Vec::new(),
                };
                mg.is_square_under_attack(king_square, Color::Black)
            }
            Color::Black => {
                let king_square =
                    Square::from_usize(self.black_king.bitboard.clone().pop_lsb().unwrap());
                let mg = MoveGen {
                    board: self,
                    pseudo_move_list: Vec::new(),
                    legal_move_list: Vec::new(),
                };
                mg.is_square_under_attack(king_square, Color::White)
            }
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "It is not suppose to panic")]
    #[allow(clippy::too_many_lines)]
    pub fn do_move(&mut self, m: &Move) {
        // Determine the piece to modify
        let piece = match (m.piece_kind, m.piece_color) {
            (Kind::Pawn, Color::White) => &mut self.white_pawn,
            (Kind::King, Color::White) => &mut self.white_king,
            (Kind::Bishop, Color::White) => &mut self.white_bishop,
            (Kind::Knight, Color::White) => &mut self.white_knight,
            (Kind::Rook, Color::White) => &mut self.white_rook,
            (Kind::Queen, Color::White) => &mut self.white_queen,
            (Kind::Pawn, Color::Black) => &mut self.black_pawn,
            (Kind::King, Color::Black) => &mut self.black_king,
            (Kind::Bishop, Color::Black) => &mut self.black_bishop,
            (Kind::Knight, Color::Black) => &mut self.black_knight,
            (Kind::Rook, Color::Black) => &mut self.black_rook,
            (Kind::Queen, Color::Black) => &mut self.black_queen,
        };
        // Generate the masks
        let from_bitboard = square_mask(m.from);
        let to_bitboard = square_mask(m.to);

        // Execute move
        piece.bitboard = piece.bitboard & !from_bitboard;

        // If the rook move, or the king, remove the casteling rights
        if piece.kind == Kind::Rook && piece.color == Color::White {
            match m.from {
                Square::H1 => self.casteling_rights.white_kingside = false,
                Square::A1 => self.casteling_rights.white_queenside = false,
                _ => (),
            }
        }
        if piece.kind == Kind::Rook && piece.color == Color::Black {
            match m.from {
                Square::H8 => self.casteling_rights.black_kingside = false,
                Square::A8 => self.casteling_rights.black_queenside = false,
                _ => (),
            }
        }
        if piece.kind == Kind::King {
            match piece.color {
                Color::White => {
                    self.casteling_rights.white_kingside = false;
                    self.casteling_rights.white_queenside = false;
                }
                Color::Black => {
                    self.casteling_rights.black_kingside = false;
                    self.casteling_rights.black_queenside = false;
                }
            }
        }

        // If the move is a promotion, it is not useful to make the pawn appear
        // So we only care when there is no promotion
        if m.promoting_piece.is_none() {
            piece.bitboard = piece.bitboard | to_bitboard;
        }

        // Handle the edge cases (promotion, casteling, double_push,
        // captures)

        // Captures
        if m.captured_piece.is_some() {
            let enemy_kind = m.captured_piece.unwrap();
            let enemy_color = match m.piece_color {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
            let enemy_piece = match (enemy_kind, enemy_color) {
                (Kind::Pawn, Color::White) => &mut self.white_pawn,
                (Kind::King, Color::White) => &mut self.white_king,
                (Kind::Bishop, Color::White) => &mut self.white_bishop,
                (Kind::Knight, Color::White) => &mut self.white_knight,
                (Kind::Rook, Color::White) => &mut self.white_rook,
                (Kind::Queen, Color::White) => &mut self.white_queen,
                (Kind::Pawn, Color::Black) => &mut self.black_pawn,
                (Kind::King, Color::Black) => &mut self.black_king,
                (Kind::Bishop, Color::Black) => &mut self.black_bishop,
                (Kind::Knight, Color::Black) => &mut self.black_knight,
                (Kind::Rook, Color::Black) => &mut self.black_rook,
                (Kind::Queen, Color::Black) => &mut self.black_queen,
            };

            // Make it disapear

            if m.en_passant {
                match enemy_color {
                    Color::White => {
                        enemy_piece.bitboard =
                            enemy_piece.bitboard & !(square_mask(self.en_passant.unwrap()) << 8);
                    }
                    Color::Black => {
                        enemy_piece.bitboard =
                            enemy_piece.bitboard & !(square_mask(self.en_passant.unwrap()) >> 8);
                    }
                }
            } else {
                enemy_piece.bitboard = enemy_piece.bitboard & !to_bitboard;
            }

            if enemy_piece.kind == Kind::Rook && enemy_piece.color == Color::White {
                if m.to == Square::H1 {
                    self.casteling_rights.white_kingside = false;
                }
                if m.to == Square::A1 {
                    self.casteling_rights.white_queenside = false;
                }
            }
            if enemy_piece.kind == Kind::Rook && enemy_piece.color == Color::Black {
                if m.to == Square::H8 {
                    self.casteling_rights.black_kingside = false;
                }
                if m.to == Square::A8 {
                    self.casteling_rights.black_queenside = false;
                }
            }
        }

        // Promotion
        if m.promoting_piece.is_some() {
            let piece_kind = m.promoting_piece.unwrap();
            let new_piece = match (piece_kind, m.piece_color) {
                (Kind::Pawn, Color::White) => &mut self.white_pawn,
                (Kind::King, Color::White) => &mut self.white_king,
                (Kind::Bishop, Color::White) => &mut self.white_bishop,
                (Kind::Knight, Color::White) => &mut self.white_knight,
                (Kind::Rook, Color::White) => &mut self.white_rook,
                (Kind::Queen, Color::White) => &mut self.white_queen,

                (Kind::Pawn, Color::Black) => &mut self.black_pawn,
                (Kind::King, Color::Black) => &mut self.black_king,
                (Kind::Bishop, Color::Black) => &mut self.black_bishop,
                (Kind::Knight, Color::Black) => &mut self.black_knight,
                (Kind::Rook, Color::Black) => &mut self.black_rook,
                (Kind::Queen, Color::Black) => &mut self.black_queen,
            };
            // Make the new piece appear
            new_piece.bitboard = new_piece.bitboard | to_bitboard;
        }

        // Double_push
        if m.double_push {
            let s_to = m.to as usize;
            let s_from = m.from as usize;
            self.en_passant = Some(Square::from_usize((s_to + s_from) / 2));
        } else {
            self.en_passant = None;
        }

        // Casteling
        if m.casteling {
            match m.to {
                Square::G1 => {
                    self.white_rook.bitboard = self.white_rook.bitboard & !square_mask(Square::H1);
                    self.white_rook.bitboard = self.white_rook.bitboard | square_mask(Square::F1);
                }
                Square::C1 => {
                    self.white_rook.bitboard = self.white_rook.bitboard & !square_mask(Square::A1);
                    self.white_rook.bitboard = self.white_rook.bitboard | square_mask(Square::D1);
                }
                Square::G8 => {
                    self.black_rook.bitboard = self.black_rook.bitboard & !square_mask(Square::H8);
                    self.black_rook.bitboard = self.black_rook.bitboard | square_mask(Square::F8);
                }
                Square::C8 => {
                    self.black_rook.bitboard = self.black_rook.bitboard & !square_mask(Square::A8);
                    self.black_rook.bitboard = self.black_rook.bitboard | square_mask(Square::D8);
                }
                _ => panic!(),
            }
        }

        self.to_move = match self.to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    #[allow(clippy::missing_panics_doc, reason = "it is not supposed to panic")]
    /// # Errors
    /// TODO
    pub fn from_fen(fen: &str) -> Result<Self, ChessMgError> {
        // start with zeroed bitboards and default values
        let mut board = Board::zero();

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(InvalidFEN("Expected at least 4 fields".to_string()));
        }

        // piece placement (ranks from 8 down to 1)
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err(InvalidFEN("Expected 8 ranks".to_string()));
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file: usize = 0;
            for ch in rank_str.chars() {
                if ch.is_ascii_digit() {
                    file += ch.to_digit(10).unwrap() as usize;
                } else {
                    if file >= 8 {
                        return Err(InvalidFEN("Too many squares in rank".to_string()));
                    }
                    // compute square index for a1 = 0 .. h8 = 63
                    let sq = u32::try_from((7 - rank_idx) * 8 + file).unwrap();
                    let bit = 1u64 << sq;

                    match ch {
                        'P' => board.white_pawn.bitboard.0 |= bit,
                        'N' => board.white_knight.bitboard.0 |= bit,
                        'B' => board.white_bishop.bitboard.0 |= bit,
                        'R' => board.white_rook.bitboard.0 |= bit,
                        'Q' => board.white_queen.bitboard.0 |= bit,
                        'K' => board.white_king.bitboard.0 |= bit,

                        'p' => board.black_pawn.bitboard.0 |= bit,
                        'n' => board.black_knight.bitboard.0 |= bit,
                        'b' => board.black_bishop.bitboard.0 |= bit,
                        'r' => board.black_rook.bitboard.0 |= bit,
                        'q' => board.black_queen.bitboard.0 |= bit,
                        'k' => board.black_king.bitboard.0 |= bit,

                        _ => return Err(InvalidFEN(format!("Invalid piece char {ch}"))),
                    }

                    file += 1;
                }
            }
            if file != 8 {
                return Err(InvalidFEN("A rank did not fill 8 files".to_string()));
            }
        }

        // side to move
        board.to_move = match parts.get(1) {
            Some(&"w") => Color::White,
            Some(&"b") => Color::Black,
            _ => return Err(InvalidFEN("Active color is invalid".to_string())),
        };

        // castling rights
        let rights = parts[2];
        board.casteling_rights.white_kingside = rights.contains('K');
        board.casteling_rights.white_queenside = rights.contains('Q');
        board.casteling_rights.black_kingside = rights.contains('k');
        board.casteling_rights.black_queenside = rights.contains('q');

        // en passant target
        let ep = parts[3];
        if ep == "-" {
            board.en_passant = None;
        } else {
            // TODO: return custom error
            board.en_passant = Some(Square::from_str(ep)?);
        }

        Ok(board)
    }
}
