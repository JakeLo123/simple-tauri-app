import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import { draw } from "./gameFunctions";
import type { Board } from "./gameTypes";
import { initUserControls } from "./userControls";

let canvas: HTMLCanvasElement | null;

type Tick = Board;

listen<Tick>("tick", (event) => {
	draw(event.payload);
});

window.addEventListener("DOMContentLoaded", async () => {
	canvas = document.querySelector("#canvas");
	const ctx = canvas?.getContext("2d");

	if (!canvas || !ctx) return;

	const board: Board = await invoke("get_board");
	draw(board);

	initUserControls();
});
