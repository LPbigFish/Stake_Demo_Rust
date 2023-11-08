<script lang="ts">
    export const ssr = true;

	import { locked, run_check, selected, unlock, wining } from "$lib/keno";
	import Error from "../Error.svelte";
	import KenoTile from "./KenoTile.svelte";
    import { onMount } from "svelte";
    
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
onMount(() => {
    setInterval(() => {
        fetch("http://127.0.0.1:8080/").then(_ => {
            error = false;
        }).catch(_ => {
            error = true;
        });
    }, 2000);

    fetch("http://127.0.0.1:8080/").then(_ => {
            error = false;
        }).catch(_ => {
            error = true;
    });
    });
</script>

{#if error}
    <div></div>
{:else}
    <div class="w-screen mx-auto">
        <div class="holder">
            <aside class="side_buttons">
                <button class="bet_button { $wining.length > 0 ? "btn-disabled" : "" }" on:click={ () => { $wining.length > 0 ? "" : run_check($selected, "JINFENINEFIIFE") } }>RUN</button>
                <button class="bet_button { ($locked && ($wining.length == 10)) ? "" : "btn-disabled" }" on:click={ unlock }>UNLOCK</button>
            </aside>
            <div class="game">
                <div class="table_container">
                    {#each numbers as tile}
                        <KenoTile number={tile} />                 
                    {/each}
                </div>
                <ul class="multies">
                    {#each $selected as _, i}
                        <li class="step transition-all duration-200 ease-linear { $selected.filter(value => $wining.includes(value)).length >= i + 1 && $selected.filter(value => $wining.includes(value)).length > 0 ? "step-secondary" : "" }">
                            {i + 1}
                        </li>
                    {/each}
                </ul>
            </div>
            
        </div>
    </div>
{/if}


<style lang="postcss">
    
    @screen lg {
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
            @apply flex-grow grid grid-cols-8 gap-2.5 h-2/3 p-4 bg-blue-950 rounded-md border-4 border-black;
        }
    
        .game {
            @apply flex flex-col max-w-3xl flex-grow gap-4 h-fit;
        }
    
        .multies {
            @apply w-full h-16 steps;
        }
    
        .multi {
            @apply step;
        }
    
        .error {
            @apply top-0 right-0 flex flex-col justify-around w-screen h-screen text-4xl text-center text-white bg-slate-700;
        }
    }
</style>