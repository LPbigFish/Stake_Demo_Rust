<script lang="ts">
    export let error: Error
</script>
{#if error}
     <strong class="text-red-700">{ error.message }</strong>
{/if}