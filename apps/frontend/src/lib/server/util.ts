import { BACKEND_URL_URI_ENCODED } from '$lib/shared/constants';

import type { AuthoriseRequest } from './types/api/internal';
export function resolve_authorise_request_url({ id, kind }: AuthoriseRequest): string {
	if (kind === 'add_roblox_account')
		return `https://apis.roblox.com/oauth/v1/authorize?client_id=4271638591494552131&redirect_uri=${BACKEND_URL_URI_ENCODED}%2Fv1%2Froblox_callback&scope=openid%20profile&response_type=code&state=r_${id}`;
	else if (kind === 'add_roblox_communities')
		return '';
	throw new TypeError(`unknown authorisation request kind: ${kind}`);
}