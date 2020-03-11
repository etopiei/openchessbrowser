use wasm_bindgen::prelude::*;
use pleco::{Board, BitMove, MoveList};
use pleco::core::{File, Rank};
use pleco::SQ;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Default)]
pub struct ChessBrowser {
    current_board: Board,
    moves: MoveList,
    move_history: Vec<BitMove>,
}

#[wasm_bindgen]
impl ChessBrowser {

    pub fn new () -> ChessBrowser {
        // Create a global board and moves object for game state
        let current_board = Board::start_pos();
        let moves = MoveList::default();
        let move_history = Vec::new();

        // Initalise object with state and return it
        ChessBrowser {
            current_board,
            moves,
            move_history
        }
    }

    fn moves_to_str(&self) -> String {
        let mut moves_string: String = "".to_string();
        for x in self.moves.iter() {
            moves_string = moves_string + &x.to_string() + ",";
        }
        moves_string
    }

    fn add_to_history(&mut self, new_move: BitMove) {
        self.move_history.push(new_move);
    }

    fn play_move(&mut self, new_move: BitMove) {
        self.current_board.apply_move(new_move);
    }

    fn file_from_char(file: char) -> Option<File> {
        match file {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            _ => None
        }
    }

    fn rank_from_char(rank: char) -> Option<Rank> {
        match rank {
            '1' => Some(Rank::R1),
            '2' => Some(Rank::R2),
            '3' => Some(Rank::R3),
            '4' => Some(Rank::R4),
            '5' => Some(Rank::R5),
            '6' => Some(Rank::R6),
            '7' => Some(Rank::R7),
            '8' => Some(Rank::R8),
            _ => None
        }
    }

    fn make_bitmove_from_str(move_string: &str) -> Option<BitMove> {
        let src_file = match ChessBrowser::file_from_char(move_string.chars().nth(0).unwrap()) {
            Some(src_file) => src_file,
            None => return None
        };
        let src_rank = match ChessBrowser::rank_from_char(move_string.chars().nth(1).unwrap()) {
            Some(src_rank) => src_rank,
            None => return None
        };

        let src_square = SQ::make(src_file, src_rank);

        let dest_file = match ChessBrowser::file_from_char(move_string.chars().nth(2).unwrap()) {
            Some(dest_file) => dest_file,
            None => return None
        };
        let dest_rank = match ChessBrowser::rank_from_char(move_string.chars().nth(3).unwrap()) {
            Some(dest_rank) => dest_rank,
            None => return None
        };

        let dest_square = SQ::make(dest_file, dest_rank);
        Some(BitMove::make(0, src_square, dest_square))
    }

    pub fn make_move(&mut self, move_string: &str) -> String {
        // Create the move from string
        if let Some(new_move) = ChessBrowser::make_bitmove_from_str(move_string) {
            // Make move on the board
            self.play_move(new_move);
            // Add move to move history
            self.add_to_history(new_move);
            // Update moves
            self.moves = self.current_board.generate_moves();
        }
        // Return the moves now available
        self.moves_to_str()
    }

    pub fn is_check_mate(&self) -> bool {
        self.current_board.checkmate()
    }

    pub fn get_latest_move(&self) -> String {
        self.move_history.last().unwrap().stringify()
    }

    pub fn get_initial_moves(&mut self) -> String {
        // Update moves to represent all possible moves from the current board state
        self.moves = self.current_board.generate_moves();
        self.moves_to_str()
    }

    pub fn get_fen(&self) -> String {
        self.current_board.fen()
    }
}
