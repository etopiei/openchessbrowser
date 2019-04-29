mod utils;

use wasm_bindgen::prelude::*;
use chess::{Board, ChessMove};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ChessBrowser {
    current_board: Board,
    new_board: Board,
    moves: [ChessMove; 256],
    moveHistory: Vec<ChessMove>
}

fn createMoveFromString(moveString: &str) -> ChessMove {
    return ChessMove::default();
}

#[wasm_bindgen]
impl ChessBrowser {

    pub fn new () -> ChessBrowser {
        // Create a global board and moves object for game state
        let current_board = Board::default();
        let new_board = Board::default();
        let moves = [ChessMove::default(); 256];
        let moveHistory: Vec<ChessMove> = Vec::new();

        // Initalise object with state and return it
        ChessBrowser {
            current_board,
            new_board,
            moves,
            moveHistory
        }
    }

    fn movesToStr(&self) -> String {
        let mut moves_string: String = "".to_string();
        for x in self.moves.iter() {
            moves_string = moves_string + &x.to_string() + ",";
        }
        moves_string
    }

    pub fn makeMove(&mut self, moveString: &str) -> String {
        // Create the move
        let newMove = createMoveFromString(moveString);
        // Make move on the board
        self.current_board.make_move(newMove, &mut self.new_board);
        self.current_board = self.new_board;
        // Add move to move history
        self.moveHistory.push(newMove);
        // Update moves
        self.current_board.enumerate_moves(&mut self.moves);
        self.movesToStr()
    }

    pub fn getInitalMoves(&mut self) -> String {
        // Update moves to represent all possible moves from the current board state
        self.current_board.enumerate_moves(&mut self.moves);
        self.movesToStr()
    }
}
