const wasm_chess = require('../pkg/openchessbrowser');
const ChessBrowser = wasm_chess.ChessBrowser;

let moves = [];
let fen = '';
let chessbrowser = ChessBrowser.new();

const dropPiece = (src, dest) => {
    // check move valid
    const valid = moves.includes(src + dest) ? true : 'snapback';
    if (valid != 'snapback') {
        // make the move
        updateAvailableMoves(chessbrowser.makeMove(src+dest));
        fen = chessbrowser.getFEN();
    }
    return valid;
};

const boardConfig = {
    draggable: true,
    dropOffBoard: 'snapback',
    onDrop: dropPiece,
    position: 'start'
};

const board = ChessBoard('chess-board', boardConfig);

function updateAvailableMoves(s) {
    moves = [];
    moves = s.split(",");
    moves.pop(); // account for superfluous comma at end of string
}

// Initial Setup
updateAvailableMoves(chessbrowser.getInitalMoves());
fen = chessbrowser.getFEN();
