import { Dexie, type EntityTable } from 'dexie';

export interface Auth {
	id: number,
	public_key: CryptoKey,
	private_key: CryptoKey
}

export const database = new Dexie('ribbonette') as Dexie & {
	auth: EntityTable<Auth, 'id'>
};

database
	.version(1)
	.stores({
		auth: 'id, public_key, private_key'
	});

export async function get_public_key(): Promise<CryptoKey> {
	const auth = await database
		.auth
		.get(1);
	if (auth)
		return auth.public_key;

	const { publicKey, privateKey } = await crypto
		.subtle
		.generateKey({
			name: 'ECDSA',
			namedCurve: 'P-384'
		}, false, ['sign', 'verify']);
	await database
		.auth
		.add({
			id: 1,
			public_key: publicKey,
			private_key: privateKey
		});

	return publicKey
}

export async function get_public_key_raw(): Promise<ArrayBuffer> {
	const public_key = await get_public_key();
	const public_key_raw = await crypto
		.subtle
		.exportKey('raw', public_key);
	return public_key_raw;
}