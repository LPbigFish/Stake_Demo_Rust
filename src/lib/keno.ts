import { parse } from 'svelte/compiler';
import { writable, type Writable } from 'svelte/store';



export const locked = writable(false);

export const selected: Writable<number[]> = writable([]);

export const wining: Writable<number[]> = writable([]);

export const enum TILE {
    NORMAL,
    SELECTED,
    LOCKED,
    WINING,
};

export const run_check = async (lock: boolean, selected: number[]) => {
    if (!lock) return;
    const wining_tiles = await fetch("http://127.0.0.1:8080/keno").then(res => res.json());
    wining_tiles.keno.forEach((value: number) => {
        wining.update(w => [...w, value]);
    });
    console.log(wining_tiles);
}