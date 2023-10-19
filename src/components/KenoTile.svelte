<script>
	import { TILE } from "$lib";

    let visual = TILE.NORMAL;

    export let number = 0;

    

    const flip = () => {
        visual = visual === TILE.NORMAL ? TILE.SELECTED : TILE.NORMAL;
    }
</script>

<button class="{ visual === TILE.NORMAL ? "bg-slate-900" : "bg-violet-800 scale-[0.9] -translate-y-1 shadow-[0px_4px_0px_7px_#000000]" }" on:click={ () => { flip() }}>
    {number}
</button>

<style lang="postcss">

    button {
        @apply w-24 m-1 aspect-square text-3xl font-bold rounded-2xl text-white ease-out transition-all duration-150;
    }
</style>
