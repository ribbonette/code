import type { CreateToastOptions, Toast } from '$lib/client/types/store/interface/toasts';
export let toast_items: Toast[] = $state([]);

export function create_toast(options: CreateToastOptions) {
	const new_toast_item: Toast = {
		kind: 'default',

		actions: options.actions ?? [],
		content_id: options.content_id,
		metadata: options.metadata ?? {},

		timeout: setTimeout(() => toast_items.shift(), 6000)
	};
	toast_items.push(new_toast_item);
}