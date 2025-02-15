import { jwtVerify } from 'jose';

import { PUBLIC_AUTH_SESSION_COOKIE_NAME } from '$env/static/public';

import { AUTH_JWT_KEY } from '$lib/server/constants';
import type { Session } from '$lib/shared/types/auth';
export async function handle({ event, resolve }) {
	const user_session_jwt = event.cookies.get(PUBLIC_AUTH_SESSION_COOKIE_NAME);
	if (user_session_jwt) {
		const { payload } = await jwtVerify<Session>(user_session_jwt, AUTH_JWT_KEY);
		event.locals.session = payload;
	}

	return resolve(event, {
		filterSerializedResponseHeaders(name) {
			return name === 'content-range';
		}
	});
}