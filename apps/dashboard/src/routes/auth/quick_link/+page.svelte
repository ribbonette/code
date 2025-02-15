<script lang="ts">
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { use_quick_link } from '$lib/client/api/auth';
	
	let error = false;
	onMount(async () => {
		const { quick_link_server_id } = await use_quick_link($page.url.hash.slice(1));
		if (quick_link_server_id)
			goto(`/server/${quick_link_server_id}`);
		else
			error = true;
	});
</script>

{#if error}
	something went wrong!!!
{:else}
	one moment...
{/if}