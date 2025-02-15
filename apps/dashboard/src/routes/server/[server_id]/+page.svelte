<script lang="ts">
	import { create_server_query } from '$lib/client/query/server';
	
	import { page } from '$app/state';
	const server = create_server_query(page.params.server_id);
</script>

{#if $server.isPending}
	is loading!!!
{:else if $server.error}
	{$server.error}
{:else}
	<h1>{$server.data.display_name}</h1>
	<p>server id: {$server.data.id}</p>
{/if}