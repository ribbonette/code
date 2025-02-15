<script lang="ts">
	import { server_navigation_items } from '$lib/client/resource/navigation';
	
	import { page } from '$app/state';

	import { create_server_query } from '$lib/client/query/server';
	import SideNavigation from '$lib/interface/components/side_navigation.svelte';
	
	const server = create_server_query(page.params.server_id);
</script>

<div class="server_root">
	{#if $server.isPending}
		LOADING!
	{:else if $server.isError}
		{$server.error}
	{:else}
		<SideNavigation items={server_navigation_items}/>
		<div class="server_root_content">
			<slot/>
		</div>
	{/if}
</div>

<style lang="scss">
	.server_root {
		display: flex;
		.server_root_content {
			
		}
	}
</style>