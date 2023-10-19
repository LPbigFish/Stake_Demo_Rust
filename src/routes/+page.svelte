<script lang="ts">
	import KenoTile from "../components/KenoTile.svelte";

    const numbers = Array.from(Array(40).keys()).map(x => x + 1);
</script>

<div class="w-screen mx-auto">
    <div class="holder">
        <aside class="side_buttons">
            <button class="bet_button">RUN</button>
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