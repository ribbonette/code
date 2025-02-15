import { AUTH_JWT_KEY as _AUTH_JWT_KEY } from '$env/static/private';
export const AUTH_JWT_KEY = new TextEncoder().encode(_AUTH_JWT_KEY);