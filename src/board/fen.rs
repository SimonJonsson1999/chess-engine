use crate::board::piece::{Color, Piece};
use crate::board::square::Square;
use crate::board::{Board, CastlingRights};
impl Board {
    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::empty();

        // extract each part of fen string
        let parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(parts.len(), 6);

        // Go over part with all pieces, and palce where applicable
        let piece_placement = parts[0];
        let ranks: Vec<&str> = piece_placement.split("/").collect();
        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file = 0;

            for ch in rank_str.chars() {
                if let Some(n) = ch.to_digit(10) {
                    file += n as u8;
                } else {
                    let rank = 7 - rank_idx as u8;
                    let square = Square::from_rank_file(rank as u8, file as u8);
                    let piece = Piece::from_fen(ch);
                    board.add_piece(piece, square);
                    file += 1;
                }
            }

            debug_assert_eq!(file, 8);
        }
        // Set whos turn it is
        let side_to_move = parts[1];
        board.turn = match side_to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Incorrect fen string"),
        };
        // Set the castling rights
        let castling = parts[2];
        let mut rights = CastlingRights::none();

        if castling != "-" {
            for c in castling.chars() {
                match c {
                    'K' => rights.white_kingside = true,
                    'Q' => rights.white_queenside = true,
                    'k' => rights.black_kingside = true,
                    'q' => rights.black_queenside = true,
                    _ => panic!("Invalid castling char"),
                }
            }
        }

        board.castling_rights = rights;

        // Set enpassant square
        let en_passant = parts[3];
        let mut enpassant = None;
        if en_passant != "-" {
            enpassant = Some(Square::from_algebraic(en_passant).expect("Wrong FEN"));
        }
        board.enpassant = enpassant;

        let halfmove_string = parts[4];
        board.half_move = halfmove_string.parse().unwrap();
        let fullmove_string = parts[5];
        board.full_move = fullmove_string.parse().unwrap();
        board
    }

    // pub fn to_fen(&self) -> &str {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fen() {
        let board = Board::from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let board_2 = Board::default();
        assert_eq!(board, board_2);
    }
}
