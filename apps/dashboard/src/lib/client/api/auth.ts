import { PUBLIC_AUTH_SESSION_COOKIE_NAME, PUBLIC_BACKEND_URL } from '$env/static/public';

import { get_public_key_raw } from '../database';
import type { UseQuickLinkResponse } from '../types/api/auth';
export async function use_quick_link(quick_link_token: string): Promise<UseQuickLinkResponse> {
	const public_key_raw = await get_public_key_raw();

	const payload = new FormData();
		payload.append('public_key', new Blob([public_key_raw]));
		payload.append('token', quick_link_token);
	
	const response = await fetch(`${PUBLIC_BACKEND_URL}/v1/auth/quick_link`, {
		body: payload,
		credentials: 'include',
		method: 'POST'
	});
	
	if (!document.cookie.includes(PUBLIC_AUTH_SESSION_COOKIE_NAME))
		throw new Error('uh oh! session cookie is missing!');

	return response.json();
}