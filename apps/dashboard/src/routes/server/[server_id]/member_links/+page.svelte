<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	import { create_server_member_link } from '$lib/client/api/server';
	import { create_server_member_links_query } from '$lib/client/query/server';
	import { create_toast } from '$lib/client/store/interface/toasts.svelte';

	const server_id = page.params.server_id;
	const member_links = create_server_member_links_query(server_id);

	let dialog_element: HTMLDialogElement;
	let new_display_name = '';
	let new_creating = false;
	const create_new = async () => {
		new_creating = true;
		const new_member_link = await create_server_member_link(server_id, new_display_name);
		if (!new_member_link.id)
			return;

		create_toast({
			actions: [{
				callback: () => alert('not implemented'),
				content_id: 'Undo'
			}],
			content_id: 'action.server.member_link.created',
			metadata: new_member_link
		});

		dialog_element.close();
		goto(`/server/${server_id}/member_links/${new_member_link.id}`);

		new_creating = false;
	};
</script>

{#if $member_links.isPending}
	we're loading them links!!!
{:else if $member_links.isError}
	{$member_links.error}
{:else}
	<h1>Member Links</h1>
	<table>
		<thead>
			<tr>
				<th>display name</th>
				<td>connectors</td>
				<td>criteria items</td>
				<td></td>
			</tr>
		</thead>
		<tbody>
			{#each $member_links.data as member_link}
				<tr>
					<th>{member_link.display_name}</th>
					<td>{member_link.connectors.items.length}</td>
					<td>{member_link.criteria.items.length}</td>
					<td>
						<a href={`/server/${server_id}/member_links/${member_link.id}`}>
							edit
						</a>
						<button type="button">
							delete
						</button>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<button type="button" on:click={() => dialog_element.showModal()}>
		create new member link
	</button>
{/if}

<dialog bind:this={dialog_element}>
	<label for="display_name">display name</label>
	<input id="display_name" type="text" bind:value={new_display_name}/>
	<div>
		<button type="button" disabled={new_creating || !new_display_name} on:click={create_new}>
			create
		</button>
		<form method="dialog">
			<button disabled={new_creating}>
				cancel
			</button>
		</form>
	</div>
</dialog>

<style lang="scss">
	a, button {
		all: unset;
		background: #fff;
		border: 1px solid #000;
		color: #000;
		cursor: pointer;
		padding: 4px 8px;
		&:not(:disabled):hover {
			color: blue;
		}
		&:disabled {
			cursor: not-allowed;
			opacity: .5;
		}
	}
	dialog {
		div {
			display: flex;
			gap: 8px;
			margin-top: 16px;
		}
	}
	// temporary css i copied from mdn web docsl ol
	table {
		background: white;
		border-collapse: collapse;
		border: 2px solid rgb(140 140 140);
		color: #000;
		font-family: sans-serif;
		font-size: 0.8rem;
		letter-spacing: 1px;
		margin-bottom: 64px;
	}

	thead {
	background-color: rgb(228 240 245);
	}

	th,
	td {
	border: 1px solid rgb(160 160 160);
	padding: 8px 10px;
	}

	td:last-of-type {
	text-align: center;
	}

	tbody > tr:nth-of-type(even) {
	background-color: rgb(237 238 242);
	}
</style>