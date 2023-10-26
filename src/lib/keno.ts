import { parse } from 'svelte/compiler';
import { writable, type Writable } from 'svelte/store';



export const locked = writable(false);

export const selected: Writable<number[]> = writable([]);

export const wining: Writable<number[]> = writable([]);

export const hard_mulltipliers = Array.from(Array(10).keys()).map((value, index) => {
    value = 104.748 * index - 337.367
});

export const enum TILE {
    NORMAL,
    SELECTED,
    LOCKED,
    WINING,
    SELWIN,
};

export let error = false;

export const run_check = async (selected: number[]) => {
    if (selected.length < 4 || selected.length > 10) return;
    locked.update(_ => true);
    wining.update(_ => []);
    const wining_tiles = await fetch("http://127.0.0.1:8080/keno").then(res => res.json());
    wining_tiles.keno.forEach((value: number, index: number) => {
        setTimeout(() => {
            wining.update(w => [...w, value]);
        }, 150 * index);
    });
    setTimeout(() => {
        console.log(wining_tiles);
    }, 150 * wining_tiles.keno.length);
}

export const unlock = () => {
    wining.update(_ => []);
    locked.update(_ => false);
    selected.update(s => s);
}