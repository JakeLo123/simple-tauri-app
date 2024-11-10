import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let incBtn: HTMLButtonElement | null;
let decBtn: HTMLButtonElement | null;
let countEl: HTMLElement | null;

type Tick = {
	count: number;
};

listen<Tick>("tick", (event) => {
	if (!countEl) return;
	countEl.textContent = String(event.payload.count);
});

window.addEventListener("DOMContentLoaded", () => {
	countEl = document.querySelector("#inc-btn");
	incBtn = document.querySelector("#inc-btn");
	decBtn = document.querySelector("#dec-btn");

	incBtn?.addEventListener("click", async () => {
		await invoke("inc_count");
	});
	decBtn?.addEventListener("click", async () => {
		await invoke("dec_count");
	});
});
