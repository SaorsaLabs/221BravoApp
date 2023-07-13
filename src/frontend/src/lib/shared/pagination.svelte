<script>
    import { createEventDispatcher } from 'svelte';
    import Button from './button.svelte';
    export let page = 1;
    export let max = 2;

    const dispatch = createEventDispatcher();
    function setPage(change){
        page += change;
        if (page<1) page = 1;
        if (page>max) page = max;
        dispatch('click', {
			page
		});
    }
</script>
<nav class="alignRight">
        <Button type="grey" slim={true} on:click={() => setPage(-1)}>PREV</Button>
        {#if page >= 3}
        <Button type="grey" slim={true} on:click={() => setPage(-2)}>{page-2}</Button>
        {/if}
        {#if page >= 2}
        <Button type="grey" slim={true} on:click={() => setPage(-1)}>{page-1}</Button>
        {/if}
        <Button type="white" slim={true} on:click={() => setPage(0)}>{page}</Button>
        {#if page <= max-1}
        <Button type="grey" slim={true} on:click={() => setPage(+1)}>{page+1}</Button>
        {/if}
        {#if page <= max-2}
        <Button type="grey" slim={true} on:click={() => setPage(+2)}>{page+2}</Button>
        {/if}
        <Button type="grey" slim={true} on:click={() => setPage(+1)}>NEXT</Button>
</nav>
<style>
    .alignRight{
        text-align: right;
        margin-right: 10px;
    }
</style>