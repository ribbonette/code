<script lang="ts">
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	import { get_server_member_links, update_server_member_link } from '$lib/client/api/server';
	import { create_toast } from '$lib/client/store/interface/toasts.svelte';
	import type { Connector, Connectors, Criteria, CriteriaItem, MemberLink } from '$lib/client/types/api/server';

	$: member_link_id = parseInt($page.params.member_link_id);
	$: server_id = $page.params.server_id;

	let member_link: MemberLink | null = null;
	onMount(async () => {
		const member_links = await get_server_member_links(server_id);
		const current_member_link = member_links
			.find(item => item.id == member_link_id);
		if (!current_member_link)
			return goto(`/server/${server_id}`);

		connectors = JSON.parse(JSON.stringify(current_member_link.connectors));
		criteria = JSON.parse(JSON.stringify(current_member_link.criteria));
		display_name = current_member_link.display_name;
		member_link = current_member_link
	});

	let connectors: Connectors = { items: [] };
	let criteria: Criteria = { items: [] };
	let display_name = '';

	let dialog_element: HTMLDialogElement;
	let dialog_element2: HTMLDialogElement;

	const connector_kinds = ['nickname', 'roles'] as const;
	const criteria_kinds = ['group_membership', 'group_membership_role', 'valid_account'] as const;
	function create_connector(connector_kind: typeof connector_kinds[number]): Connector {
		if (connector_kind == 'nickname')
			return { kind: connector_kind };
		return { kind: connector_kind, target_role_ids: [] };
	}

	function create_criteria_item(criteria_kind: typeof criteria_kinds[number]): CriteriaItem {
		if (criteria_kind == 'group_membership')
			return { kind: criteria_kind, group_id: '' };
		if (criteria_kind == 'group_membership_role')
			return { kind: criteria_kind, group_id: '', role_id: '' };
		return { kind: criteria_kind };
	}

	let is_saving = false;
	async function save_changes() {
		is_saving = true;

		await update_server_member_link(server_id, member_link_id, {
			connectors,
			criteria,
			display_name
		});

		create_toast({
			content_id: 'action.server.member_link.saved',
			metadata: { display_name }
		});

		member_link = {
			id: member_link_id,
			display_name,

			connectors,
			criteria
		};

		is_saving = false;
	}
</script>

<div class="geist">
	{#if member_link}
		<h1>{member_link.display_name}</h1>

		<label for="display_name">display name</label>
		<input id="display_name" type="text" bind:value={display_name}/>

		<table>
			<thead>
				<tr>
					<th>kind</th>
					<td></td>
					<td></td>
				</tr>
			</thead>
			<tbody>
				{#each criteria.items as criteria_item}
					<tr>
						<th>{criteria_item.kind}</th>
						<th>
							{#if criteria_item.kind === 'group_membership' || criteria_item.kind === 'group_membership_role'}
								<input placeholder="group id" type="text" bind:value={criteria_item.group_id}/>
							{/if}
							{#if criteria_item.kind === 'group_membership_role'}
								<input placeholder="role id" type="text" bind:value={criteria_item.role_id}/>
							{/if}
						</th>
						<td>
							<button type="button" on:click={() => criteria.items = criteria.items.filter(item => item !== criteria_item)}>
								delete
							</button>
						</td>
					</tr>
				{/each}
				<tr>
					<th>
						<button type="button" on:click={() => dialog_element.show()}>
							add new criteria item
						</button>
					</th>
					<td></td>
					<td></td>
				</tr>
			</tbody>
		</table>

		<table>
			<thead>
				<tr>
					<th>kind</th>
					<td></td>
					<td></td>
				</tr>
			</thead>
			<tbody>
				{#each connectors.items as connector_item}
					<tr>
						<th>{connector_item.kind}</th>
						<th>
							{#if connector_item.kind === 'roles'}
								<div class="roles">
									{#each connector_item.target_role_ids as role_id}
										<div class="role">
											<input type="text" bind:value={role_id}/>
											<button type="button" on:click={() => connector_item.target_role_ids = connector_item.target_role_ids.filter(item => item != role_id)}>
												X
											</button>
										</div>
									{/each}
								</div>
								<button type="button" on:click={() => connector_item.target_role_ids = [...connector_item.target_role_ids, '']}>
									add role
								</button>
							{/if}
						</th>
						<td>
							<button type="button" on:click={() => connectors.items = connectors.items.filter(item => item !== connector_item)}>
								delete
							</button>
						</td>
					</tr>
				{/each}
				<tr>
					<th>
						<button type="button" on:click={() => dialog_element2.show()}>
							add new connector
						</button>
					</th>
					<td></td>
					<td></td>
				</tr>
			</tbody>
		</table>

		<div>
			<button type="button" disabled={is_saving} on:click={save_changes}>
				save changes
			</button>
			<a href={`/server/${server_id}/member_links`}>
				cancel
			</a>
		</div>
	{:else}
		loading!
	{/if}
</div>

<dialog bind:this={dialog_element}>
	<p>select kind</p>
	{#each criteria_kinds as kind}
		<button type="button" on:click={() => {
			criteria.items = [...criteria.items, create_criteria_item(kind)];
			dialog_element.close();
		}}>
			{kind}
		</button>
	{/each}
</dialog>

<dialog bind:this={dialog_element2}>
	<p>select kind</p>
	{#each connector_kinds as kind}
		<button type="button" on:click={() => {
			connectors.items = [...connectors.items, create_connector(kind)];
			dialog_element2.close();
		}}>
			{kind}
		</button>
	{/each}
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