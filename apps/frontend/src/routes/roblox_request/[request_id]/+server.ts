import { redirect } from '@sveltejs/kit';

import { get_authorise_request } from '$lib/server/api/internal';
import { resolve_authorise_request_url } from '$lib/server/util';
export async function GET({ params: { request_id }}) {
	const request = await get_authorise_request(request_id);
	const redirect_url = resolve_authorise_request_url(request);
	return redirect(302, redirect_url);
}