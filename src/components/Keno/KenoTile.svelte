<script lang="ts">
	import { TILE, locked, selected, wining } from "$lib/keno";

    let visual = TILE.NORMAL;

    export let number = 0;

    const update = () => {
        if ((($locked && !$selected.includes(number)) || (!$selected.includes(number) && $selected.length >= 10)) && ($locked && !$wining.includes(number))) {
            visual = TILE.LOCKED;
        } else if ($wining.includes(number) && $selected.includes(number)) {
            visual = TILE.SELWIN;
        } else if ($wining.includes(number)) {
            visual = TILE.WINING;
        } else if ($selected.includes(number)) {
            visual = TILE.SELECTED;
        } else {
            visual = TILE.NORMAL;
        }
    };

    const flip = () => {
        visual = visual === TILE.NORMAL ? TILE.SELECTED : TILE.NORMAL;
        visual === TILE.SELECTED ? $selected = [...$selected, number] : $selected.splice($selected.indexOf(number), 1);
        $selected = $selected;
    };

    wining.subscribe(() => {
        update();
    });
    
    locked.subscribe(() => {
        update();
    });

    selected.subscribe(() => {
        update();
    });
</script>
<button class="w-full aspect-square" on:click={ () => { (visual === TILE.LOCKED || ($locked && !$selected.includes(number) || ($wining.length > 0)) ? () => {} : flip()) }}>
    <div class="{ visual === TILE.SELECTED ? "bg-violet-800 scale-90 -translate-y-1 shadow-[0px_4px_0px_7px_#000000]" : 
        visual === TILE.LOCKED && $locked ? "delay-150 opacity-50 bg-transparent shadow-[0px_0px_0px_7px_#000000] shadow-gray-900 scale-90 -translate-y-1" : 
        visual === TILE.SELWIN && $locked ? "bg-violet-500 scale-90 -translate-y-1 shadow-[0px_4px_0px_7px_rgb(132_204_22)]" :
        visual === TILE.WINING ? "bg-lime-500 shadow-[0px_3px_0px_5px_#FFFFFF] scale-90 -translate-y-1 border-transparent border-0" : "bg-gray-950" }">
    <p class=" { visual === TILE.WINING ? "drop-shadow-[0_1.2px_1.5px_rgba(0,0,0,0.8)]" : "" }">{number}</p>
</div>
</button>


<style lang="postcss">
    div {
        @apply w-full aspect-square text-3xl font-bold flex flex-col justify-around rounded-2xl text-white ease-linear transition-all duration-200 border-gray-900;
    }
</style>
