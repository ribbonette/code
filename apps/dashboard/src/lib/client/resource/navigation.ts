import type { SideNavigationItem } from '../types/interface/side_navigation';

export const server_navigation_items: SideNavigationItem[] = [
	{
		display_label: 'Overview',
		pathname: `/server/[server_id]`
	},
	{
		display_label: 'Member Links',
		pathname: `/server/[server_id]/member_links`
	},
	{
		display_label: 'Integrations',
		pathname: `/server/[server_id]/integrations`
	}
];