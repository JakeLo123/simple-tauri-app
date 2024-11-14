import { tick, resetGame, startGame, toggleCell } from "./gameFunctions";

export function initUserControls() {
	const tickBtn = document.getElementById("tick-btn") as HTMLButtonElement;
	const playBtn = document.getElementById("play-btn") as HTMLButtonElement;
	const resetBtn = document.getElementById("reset-btn") as HTMLButtonElement;
	const canvas = document.getElementById("canvas") as HTMLCanvasElement;

	canvas.addEventListener("click", toggleCell);
	playBtn.addEventListener("click", startGame);
	tickBtn.addEventListener("click", tick);
	resetBtn.addEventListener("click", resetGame);
}
