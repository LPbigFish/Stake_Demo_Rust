<script lang="ts">
    export const ssr = true;

	import { locked, run_check, selected } from "$lib/keno";
	import KenoTile from "../components/KenoTile.svelte";

    const numbers = Array.from(Array(40).keys()).map(x => x + 1);

    selected.subscribe(value => {
        if (value.length === 10) {
            $locked = true;
        } else {
            $locked = false;
        }
    });
</script>

<div class="w-screen mx-auto">
    <div class="holder">
        <aside class="side_buttons">
            <button class="bet_button" on:click={ () => { run_check($locked, $selected) } }>RUN</button>
        </aside>
        <div class="table_container">
            {#each numbers as tile}
                <KenoTile number={tile} />                 
            {/each}
        </div>
    </div>
</div>

<style lang="postcss">
    .holder {
        @apply bg-indigo-950 max-w-6xl mx-auto h-[70vh] p-3 flex flex-row gap-4;
    }

    .side_buttons {
        @apply flex flex-col w-1/5;
    }

    .bet_button {
        @apply btn btn-primary font-mono btn-lg w-48;
    }

    .table_container {
        @apply flex-grow h-full;
    }
</style>