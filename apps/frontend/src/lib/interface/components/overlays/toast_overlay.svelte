<script lang="ts">
	import './toast_overlay.scss';

	import { toast_items } from '$lib/client/store/interface/toasts.svelte';

	import ToastOverlayItem from './toast_overlay_item.svelte';
</script>

<div class="overlay_toasts">
	{#each toast_items as toast_item}
		<ToastOverlayItem toast={toast_item}/>
	{/each}
</div>