import adapter from '@sveltejs/adapter-vercel';
import { sveltePreprocess } from 'svelte-preprocess';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
export default {
	extensions: ['.svelte'],
	kit: {
		csrf: { checkOrigin: false },
		adapter: adapter({ runtime: 'edge' })
	},
	preprocess: [
		sveltePreprocess({}),
		vitePreprocess()
	]
};