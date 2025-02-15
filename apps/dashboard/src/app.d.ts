import 'unplugin-icons/types/svelte';
import type { Session } from '$lib/shared/types/auth';
declare global {
	namespace App {
		interface Locals {
			session: Session | null
		}
		interface PageData {
			session: Session | null
		}
	}
}

export {};