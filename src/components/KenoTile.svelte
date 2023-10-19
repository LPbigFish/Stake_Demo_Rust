<script>
	import { TILE, locked, selected, wining } from "$lib/keno";

    let visual = TILE.NORMAL;

    export let number = 0;

    const flip = () => {
        visual = visual === TILE.NORMAL ? TILE.SELECTED : TILE.NORMAL;
        visual === TILE.SELECTED ? $selected = [...$selected, number] : $selected.splice($selected.indexOf(number), 1);
        $selected = $selected;
    }

    $: if ($locked && !$selected.includes(number)) {
        visual = TILE.LOCKED;
    } else if ($selected.includes(number)) {
        visual = TILE.SELECTED;
    } else {
        visual = TILE.NORMAL;
    }

    wining.subscribe(value => {
        if (value.includes(number)) {
            visual = TILE.WINING;
        }
    });
</script>

<button class="{ visual === TILE.SELECTED ? "bg-violet-800 scale-[0.9] -translate-y-1 shadow-[0px_4px_0px_7px_#000000]" : visual === TILE.LOCKED ? "opacity-50 bg-transparent border-4" : visual === TILE.WINING ? "bg-lime-500" : "bg-gray-950 border" }" on:click={ () => { (visual === TILE.LOCKED ? () => {} : flip()) }}>
    {number}
</button>

<style lang="postcss">

    button {
        @apply w-24 m-1 aspect-square text-3xl font-bold rounded-2xl text-white ease-out transition-all duration-150 border-gray-900;
    }
</style>
