<script lang="ts">
    export const ssr = true;

	import { locked, run_check, selected, unlock, wining } from "$lib/keno";
	import KenoTile from "./KenoTile.svelte";
    
    export let error = false;

    const numbers = Array.from(Array(40).keys()).map(x => x + 1);

    selected.subscribe(value => {
        value.length >= 10 ? $locked = true : $locked = false;
    });

    wining.subscribe(value => {
        if (value.length <= 10) {
            const count = $selected.filter(value => $wining.includes(value)).length;
            console.log(count);
        }
    });

    setInterval(() => {
        fetch("http://127.0.0.1:8080/keno").then(_ => {
            error = false;
        }).catch(_ => {
            error = true;
        });
    }, 2000);

    fetch("http://127.0.0.1:8080/keno").then(_ => {
            error = false;
        }).catch(_ => {
            error = true;
    });
</script>

{#if error}
     <div class="error">THERE WAS AN ERROR WITH THE SERVER. PLEASE TRY AGAIN LATER.</div>
{:else}
    <div class="w-screen mx-auto">
        <div class="holder">
            <aside class="side_buttons">
                <button class="bet_button { $wining.length > 0 ? "btn-disabled" : "" }" on:click={ () => { $wining.length > 0 ? "" : run_check($selected) } }>RUN</button>
                <button class="bet_button { ($locked && ($wining.length == 10)) ? "" : "btn-disabled" }" on:click={ unlock }>UNLOCK</button>
            </aside>
            <div class="game">
                <div class="table_container">
                    {#each numbers as tile}
                        <KenoTile number={tile} />                 
                    {/each}
                </div>
                <div class="multies">
                    {#each $selected as j, i}
                        <div class="multi { $selected.filter(value => $wining.includes(value)).length >= i + 1 && $selected.filter(value => $wining.includes(value)).length > 0 ? "bg-lime-500" : "bg-gray-900" }">
                            {i + 1}
                        </div>
                    {/each}
                </div>
            </div>
            
        </div>
    </div>
{/if}


<style lang="postcss">
    .holder {
        @apply bg-indigo-950 max-w-6xl mx-auto h-[70vh] p-3 flex flex-row gap-4;
    }

    .side_buttons {
        @apply flex flex-col w-1/5 justify-center gap-4 -translate-y-16;
    }

    .bet_button {
        @apply btn btn-primary font-mono btn-lg w-48;
    }

    .table_container {
        @apply flex-grow h-full grid grid-cols-8;
    }

    .game {
        @apply flex flex-col h-fit w-fit;
    }

    .multies {
        @apply flex flex-row w-full h-16 bg-black;
    }

    .multi {
        @apply flex-grow text-2xl text-white text-center transition-all delay-100 duration-100 ease-out;
    }

    .error {
        @apply top-0 right-0 flex flex-col justify-around w-screen h-screen text-4xl text-center text-white bg-slate-700;
    }
</style>