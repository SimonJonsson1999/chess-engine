use crate::piece::Piece;
use crate::square::Square;

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub captured_piece: Option<Piece>,
    pub previous_enpassant: Option<Square>,
}
impl Move {
    pub fn new(
        from: Square,
        to: Square,
        captured_piece: Option<Piece>,
        previous_enpassant: Option<Square>,
    ) -> Self {
        Move {
            from,
            to,
            captured_piece,
            previous_enpassant,
        }
    }
}

pub struct MoveLog {
    pub moves: Vec<Move>,
}

impl MoveLog {
    pub fn new() -> Self {
        let moves = Vec::new();
        MoveLog { moves }
    }
    pub fn add(&mut self, move_entry: Move) {
        self.moves.push(move_entry);
    }
    pub fn remove(&mut self) -> Option<Move> {
        self.moves.pop()
    }
    pub fn print_entries(&self) {
        for entry in &self.moves {
            match entry.captured_piece {
                Some(piece) => {
                    println!(
                        "{}{}, captured {}{}",
                        entry.from, entry.to, piece.color, piece.piece_type
                    );
                }
                None => {
                    println!("{}{}", entry.from, entry.to);
                }
            }
        }
    }
}
