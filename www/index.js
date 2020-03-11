import { ChessBrowser } from '../pkg/openchessbrowser';

let moves = [];
let madeMoves = [];
let fen = '';
let chessbrowser = ChessBrowser.new();

const RESULT = {
    "1-0": "whiteWin",
    "0-1": "blackWin",
    "1/2-1/2": "draw"
};

const dropPiece = (src, dest) => {
    // check move valid
    console.log(moves, src, dest);
    let valid = moves.includes(src + dest) ? true : 'snapback';
    if (valid != 'snapback') {
        // make the move
        const move = src + dest;
        updateAvailableMoves(chessbrowser.make_move(move));
        const newFen = chessbrowser.get_fen();
        if (fen === newFen) {
            valid = 'snapback'
        } else {
            // update the move log with the last move played.
            // In future need better logic here to show which piece moved, but what can you do, y'know
            madeMoves.push(`${chessbrowser.get_latest_move()}`);
            document.getElementById('moves').children[1].innerHTML = madeMoves.join(" , ");
            // re-fetch games list
            getGamesWhereFENOccured(fen).then(gameData => {
                updateList(gameData);
            });
        }
        fen = newFen;
    }
    return valid;
};

const boardConfig = {
    draggable: true,
    dropOffBoard: 'snapback',
    onDrop: dropPiece,
    position: 'start'
};

ChessBoard('chess-board', boardConfig);

function updateAvailableMoves(s) {
    moves = [];
    moves = s.split(",");
    moves.pop(); // account for superfluous comma at end of string
    if (moves.length === 0) {
        // Find out if it's check mate or stale-mate
        if (chessbrowser.is_check_mate()) {
            alert("CheckMate!");
        } else {
            alert("StaleMate!");
        }
    }
}

function getGamesWhereFENOccured(fen) {
    return fetch('http://localhost:3000/games/fen', {method: "POST", body: fen, headers: {'Content-Type': 'application/json'}}).then(res => res.json());
}

function updateList(gameData) {
    const gameContainer = document.getElementById('game-list-grid');
    gameContainer.innerText = "";
    // add new games to table
    gameData.forEach((game) => {
        let item = document.createElement('li');
        item.innerText = game.event;
        item.className = RESULT[game.result];
        item.addEventListener('click', () => {
            // When a game is clicked, load the moves
            fetch(`http://localhost:3000/games/moves`, {method: "POST", body: game.game_id, headers: {'Content-Type': 'application/json'}}).then(r => r.json()).then(res => {
                console.log(res);
            });
        });
        gameContainer.appendChild(item);
    });
}

// Initial Setup
updateAvailableMoves(chessbrowser.get_initial_moves());
fen = chessbrowser.get_fen();
console.log(fen);
getGamesWhereFENOccured(fen).then(gameData => {
    updateList(gameData);
});
