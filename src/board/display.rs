use crate::board::Board;
use crate::board::piece::{Color, PieceType};
impl Board {
    pub fn show(&self) {
        let mut piece_names: [Option<(Color, PieceType)>; 64] = [None; 64];
        for color in [Color::White, Color::Black] {
            for piece in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                let bb = &self.bitboards[color][piece];
                for square in 0..64 {
                    if (bb.0 & (1u64 << square)) != 0 {
                        piece_names[square] = Some((color, piece))
                    }
                }
            }
        }
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);

            for file in 0..8 {
                let square = rank * 8 + file;

                match piece_names[square] {
                    Some((color, piece)) => print!("{}{} ", color, piece),
                    None => print!(".. "),
                }
            }

            println!(); // <- this must execute once per rank
        }

        println!("  A  B  C  D  E  F  G  H");
    }

    pub fn print_log(&self) {
        self.move_log.print_entries();
    }
}
