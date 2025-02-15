import type { User } from './user';
export interface UseQuickLinkResponse {
	quick_link_server_id: string | null,
	user: User
}