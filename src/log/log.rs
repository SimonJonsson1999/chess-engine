use crate::piece::Piece;
use crate::square::Square;
use crate::board::CastlingRights;
use crate::piece::MoveKind;


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LogEntry {
    pub from: Square,
    pub to: Square,
    pub captured_piece: Option<Piece>,
    pub previous_enpassant: Option<Square>,
    pub previous_castling_rights: CastlingRights,
    pub move_kind: MoveKind
}
impl LogEntry {
    pub fn new(
        from: Square,
        to: Square,
        captured_piece: Option<Piece>,
        previous_enpassant: Option<Square>,
        previous_castling_rights: CastlingRights,
        move_kind: MoveKind,

    ) -> Self {
        LogEntry {
            from,
            to,
            captured_piece,
            previous_enpassant,
            previous_castling_rights,
            move_kind
        }
    }
}

pub struct MoveLog {
    pub moves: Vec<LogEntry>,
}

impl MoveLog {
    pub fn new() -> Self {
        let moves = Vec::new();
        MoveLog { moves }
    }
    pub fn add(&mut self, move_entry: LogEntry) {
        self.moves.push(move_entry);
    }
    pub fn remove(&mut self) -> Option<LogEntry> {
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
