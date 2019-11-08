mod utils;

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
pub struct ChessBrowser {
    current_board: Board,
    moves: MoveList,
    moveHistory: Vec<BitMove>,
}

#[wasm_bindgen]
impl ChessBrowser {

    pub fn new () -> ChessBrowser {
        // Create a global board and moves object for game state
        let current_board = Board::start_pos();
        let moves = MoveList::default();
        let moveHistory = Vec::new();

        // Initalise object with state and return it
        ChessBrowser {
            current_board,
            moves,
            moveHistory,
        }
    }

    fn movesToStr(&self) -> String {
        let mut moves_string: String = "".to_string();
        for x in self.moves.iter() {
            moves_string = moves_string + &x.to_string() + ",";
        }
        moves_string
    }

    fn addToHistory(&mut self, newMove: BitMove) {
        self.moveHistory.push(newMove);
    }

    fn playMove(&mut self, newMove: &BitMove) {
        self.current_board.apply_move(*newMove);
    }

    fn fileFromChar(file: char) -> Option<File> {
        return match file {
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

    fn rankFromChar(rank: char) -> Option<Rank> {
        return match rank {
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

    fn makeBitMoveFromString(moveString: &str) -> Option<BitMove> {
        let srcFile = match ChessBrowser::fileFromChar(moveString.chars().nth(0).unwrap()) {
            Some(srcFile) => srcFile,
            None => return None
        };
        let srcRank = match ChessBrowser::rankFromChar(moveString.chars().nth(1).unwrap()) {
            Some(srcRank) => srcRank,
            None => return None
        };

        let srcSquare = SQ::make(srcFile, srcRank);

        let destFile = match ChessBrowser::fileFromChar(moveString.chars().nth(2).unwrap()) {
            Some(destFile) => destFile,
            None => return None
        };
        let destRank = match ChessBrowser::rankFromChar(moveString.chars().nth(3).unwrap()) {
            Some(destRank) => destRank,
            None => return None
        };

        let destSquare = SQ::make(destFile, destRank);
        Some(BitMove::make(0, srcSquare, destSquare))
    }

    pub fn makeMove(&mut self, moveString: &str) -> String {
        // Create the move from string
        if let Some(newMove) = ChessBrowser::makeBitMoveFromString(moveString) {
            // Make move on the board
            self.playMove(&newMove);
            // Add move to move history
            self.addToHistory(newMove);
            // Update moves
            self.moves = self.current_board.generate_moves();
        }
        // Return the moves
        self.movesToStr()
    }

    pub fn getInitalMoves(&mut self) -> String {
        // Update moves to represent all possible moves from the current board state
        self.moves = self.current_board.generate_moves();
        self.movesToStr()
    }

    pub fn getFEN(&self) -> String {
        self.current_board.fen()
    }
}
