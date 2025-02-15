import { PUBLIC_BACKEND_URL } from '$env/static/public';

import { fetch_json } from '$lib/fetch';
import type { MemberLink, Server } from '../types/api/server';
export async function get_server(server_id: string): Promise<Server> {
	const response = await fetch_json<Server>(`${PUBLIC_BACKEND_URL}/v1/server/${server_id}`, {
		credentials: 'include'
	});

	return response;
}

export async function create_server_member_link(server_id: string, display_name: string): Promise<MemberLink> {
	const response = await fetch_json<MemberLink>(`${PUBLIC_BACKEND_URL}/v1/server/${server_id}/member_links`, {
		body: JSON.stringify({ display_name }),
		credentials: 'include',
		headers: {
			'content-type': 'application/json'
		},
		method: 'POST'
	});

	return response;
}

export async function update_server_member_link(server_id: string, member_link_id: | number, payload: Partial<Omit<MemberLink, 'id'>>) {
	await fetch(`${PUBLIC_BACKEND_URL}/v1/server/${server_id}/member_link/${member_link_id}`, {
		body: JSON.stringify(payload),
		credentials: 'include',
		headers: {
			'content-type': 'application/json'
		},
		method: 'PATCH'
	});
}

export async function get_server_member_links(server_id: string): Promise<MemberLink[]> {
	const response = await fetch_json<MemberLink[]>(`${PUBLIC_BACKEND_URL}/v1/server/${server_id}/member_links`, {
		credentials: 'include'
	});

	return response;
}