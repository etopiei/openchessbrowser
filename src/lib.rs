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

    fn makeBitMoveFromString(moveString: &str) -> BitMove {
        let srcFile = File::A;
        let srcRank = Rank::R2;
        let srcSquare = SQ::make(srcFile, srcRank);

        let destFile = File::A;
        let destRank = Rank::R3;
        let destSquare = SQ::make(destFile, destRank);

        BitMove::make(0, srcSquare, destSquare)
    }

    pub fn makeMove(&mut self, moveString: &str) -> String {
        // Create the move from string
        let newMove: BitMove = ChessBrowser::makeBitMoveFromString(moveString);
        // Make move on the board
        self.playMove(&newMove);
        // Add move to move history
        self.addToHistory(newMove);
        // Update moves
        self.moves = self.current_board.generate_moves();
        // Return the moves
        self.movesToStr()
    }

    pub fn getInitalMoves(&mut self) -> String {
        // Update moves to represent all possible moves from the current board state
        self.moves = self.current_board.generate_moves();
        self.movesToStr()
    }

    pub fn getFEN(self) -> String {
        self.current_board.fen()
    }
}
