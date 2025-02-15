import { PUBLIC_BACKEND_URL } from '$env/static/public';
import { BACKEND_INTERNAL_KEY } from '$env/static/private';

import type { AuthoriseRequest } from '$lib/server/types/api/internal';
export async function get_authorise_request(link_id: string): Promise<AuthoriseRequest> {
	const response = await fetch(`${PUBLIC_BACKEND_URL}/v1/internal/authorise_request/${link_id}`, {
		headers: {
			'x-api-key': BACKEND_INTERNAL_KEY
		}
	})
		.then(response => response.json());

	return response;
}