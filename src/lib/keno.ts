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

const myHeaders = new Headers();
myHeaders.append("Content-Type", "application/json");

export const run_check = async (selected: number[], uuid: string) => {
    if (selected.length < 4 || selected.length > 10) return;
    locked.update(_ => true);
    wining.update(_ => []);

    let raw = JSON.stringify({
        "uuid": uuid
    });

    const requestOptions: RequestInit = {
        method: 'GET',
        headers: myHeaders,
        body: raw,
        redirect: 'follow'
    };

    const wining_tiles = await fetch("127.0.0.1:8080/keno", requestOptions)
        .then(res => res.json())
        .catch(error => console.log('error', error));
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

