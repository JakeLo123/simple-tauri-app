import { invoke } from "@tauri-apps/api/core";
import type { Board, Cell } from "./gameTypes";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
const cellsize = 30; // px

export function draw(board: Board) {
	canvas.width = board.length * cellsize;
	canvas.height = board[0].length * cellsize;

	for (let y = 0; y < board.length; y += 1) {
		const col = board[y];
		for (let x = 0; x < col.length; x += 1) {
			const cell = col[x];
			_drawCell(x * cellsize, y * cellsize, cell);
		}
	}
}

function _drawCell(xOrigin: number, yOrigin: number, cell: Cell) {
	if (cell.alive) {
		ctx.fillStyle = "black";
		ctx.fillRect(xOrigin, yOrigin, cellsize, cellsize);
	} else {
		ctx.moveTo(xOrigin, yOrigin);
		ctx.lineTo(xOrigin + cellsize, yOrigin);
		ctx.lineTo(xOrigin + cellsize, yOrigin + cellsize);
		ctx.lineTo(xOrigin, yOrigin + cellsize);
		ctx.lineTo(xOrigin, yOrigin);

		ctx.strokeStyle = "black";
		ctx.stroke();
	}
}

export async function toggleCell(e: MouseEvent) {
	const x = Math.floor(e.offsetX / cellsize);
	const y = Math.floor(e.offsetY / cellsize);
	await invoke("toggle_cell", { coordinates: { x, y } });
}

export async function tick() {
	await invoke("tick");
}

export async function resetGame() {
	await invoke("reset_game");
}

export async function startGame() {
	await invoke("start_game");
}
