import { ChessBrowser } from "openchessbrowser";

let moves = [];
let chessbrowser = ChessBrowser.new();

function updateAvailableMoves(s) {
    moves = s.split(",");
    moves.pop(); // account for superfluous comma at end of string
}

console.log(moves);
updateAvailableMoves(chessbrowser.getInitalMoves());
console.log(moves);
