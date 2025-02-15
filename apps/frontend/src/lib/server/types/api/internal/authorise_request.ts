export interface AuthoriseRequest {
	id: string
	kind: AuthoriseRequestKind
}

export type AuthoriseRequestKind =
	'add_roblox_account' |
	'add_roblox_communities'