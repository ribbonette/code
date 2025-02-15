<script lang="ts">
	import './toast_overlay_item.scss';
	import { toast_items } from '$lib/client/store/interface/toasts.svelte';
	import type { Toast } from '$lib/client/types/store/interface/toasts';

	import IconInfoCircle from 'virtual:icons/bi/info-circle';
	export let toast: Toast;

	let fade = false;
</script>

<div class="toast_overlay_item" class:fade>
	<div class="icon">
		<IconInfoCircle font-size={18}/>
	</div>
	<p>
		<!-- this is a temporary thing -->
		{#if toast.content_id === 'action.server.member_link.created'}
			<w>{toast.metadata.display_name}</w> member link has been created
		{:else if toast.content_id === 'action.server.member_link.saved'}
			<w>{toast.metadata.display_name}</w> member link has been saved
		{:else}
			{toast.content_id}
		 {/if}
	</p>
	{#each toast.actions as action_item}
		<button type="button" on:click={() => {
			fade = true;
			clearTimeout(toast.timeout);
			setTimeout(() => toast_items.splice(toast_items.indexOf(toast), 1), 500);

			action_item.callback();
		}}>
			{action_item.content_id}
		</button>
	{/each}
</div>