const move_mapper = require('./mapper.js');
import { ChessBrowser } from '../pkg/openchessbrowser';

let moves = [];
let fen = '';
let chessbrowser = new ChessBrowser();

const dropPiece = (src, dest) => {
    // check move valid
    console.log(moves, src, dest);
    const valid = moves.includes(src + dest) ? true : 'snapback';
    if (valid != 'snapback') {
        // make the move
        updateAvailableMoves(chessbrowser.make_move(move_mapper.move_map[src], move_mapper.move_map[dest]));
        fen = chessbrowser.get_fen();
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
    const gameContainer = document.getElementById('game-list-grid');
    gameContainer.innerText = "";
    // add new games to table
    gameData.forEach((game) => {
        let item = document.createElement('li');
        item.innerText = game.event;
        gameContainer.appendChild(item);
    });
}

// Initial Setup
updateAvailableMoves(chessbrowser.get_inital_moves());
fen = chessbrowser.get_fen();
console.log(fen);
getGamesWhereFENOccured(fen).then(gameData => {
    updateList(gameData);
});
