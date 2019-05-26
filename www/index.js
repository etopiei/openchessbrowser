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

function getGamesWhereFENOccured(fen) {
    return fetch('http://localhost:3000/', {method: "POST", body: fen, headers: {'Content-Type': 'application/json'}}).then(res => res.json());
}

function updateList(gameData) {
    const gameContainer = document.getElementById('game-list');
    // clear old table
    // add new games to table
    gameData.forEach((game) => {
        let item = document.createElement('li');
        item.innerText = game.event;
        gameContainer.appendChild(item);
    });
}

// Initial Setup
updateAvailableMoves(chessbrowser.getInitalMoves());
fen = chessbrowser.getFEN();
console.log(fen);
getGamesWhereFENOccured(fen).then(gameData => {
    updateList(gameData);
});
