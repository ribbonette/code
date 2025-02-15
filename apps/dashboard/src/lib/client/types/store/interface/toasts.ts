export interface Toast {
	kind: 'default',

	actions: ToastAction[],
	content_id: string,
	metadata: any,

	timeout: NodeJS.Timeout
}

export interface ToastAction {
	callback: () => void,
	content_id: string
}

export interface CreateToastOptions {
	actions?: ToastAction[],
	content_id: string,
	metadata?: any
}