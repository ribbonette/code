import { createQuery } from '@tanstack/svelte-query';

import { get_server, get_server_member_links } from '../api/server';
export function create_server_query(server_id: string) {
	return createQuery({
		queryKey: ['server', server_id],
		queryFn: () => get_server(server_id)
	});
}

export function create_server_member_links_query(server_id: string) {
	return createQuery({
		queryKey: ['server', server_id, 'member_links'],
		queryFn: () => get_server_member_links(server_id)
	});
}