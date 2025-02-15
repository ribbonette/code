export interface MemberLink {
	id: number,
	display_name: string,

	connectors: Connectors,
	criteria: Criteria
}

export interface Connectors {
	items: Connector[]
}

export type Connector =
	ConnectorNickname |
	ConnectorRoles

export interface ConnectorNickname {
	kind: 'nickname'
}

export interface ConnectorRoles {
	kind: 'roles',
	target_role_ids: string[]
}

export interface Criteria {
	items: CriteriaItem[]
}

export type CriteriaItem =
	CriteriaItemGroupMembership |
	CriteriaItemGroupMembershipRole |
	CriteriaItemValidAccount

export interface CriteriaItemGroupMembership {
	kind: 'group_membership',
	group_id: string
}
export interface CriteriaItemGroupMembershipRole {
	kind: 'group_membership_role',
	group_id: string,
	role_id: string
}
export interface CriteriaItemValidAccount {
	kind: 'valid_account'
}