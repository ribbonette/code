<script lang="ts">
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';
	
	import { browser } from '$app/environment';
	
	import '$lib/interface/styles/root.scss';
	import BrandLogo from '$lib/interface/visuals/brand_logo.svelte';
	import ToastOverlay from '$lib/interface/components/overlays/toast_overlay.svelte';
	const query_client = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser
			}
		}
	});
</script>

<QueryClientProvider client={query_client}>
	<header>
		<h1>
			<BrandLogo height={30}/>
		</h1>
	</header>
	<main>
		<slot/>
	</main>
	<footer>
		<BrandLogo height={32}/>
		<p>
			An experimental Roblox-centric Discord Bot developed by <a href="https://hakumi.cafe" target="_blank">HAKUMI</a>.
		</p>
	</footer>
	<ToastOverlay/>
</QueryClientProvider>

<style lang="scss">
	header {
		align-items: center;
		background: hsl(315 60% 80%);
		border-radius: 16px;
		box-shadow: 0 4px 16px 0 #00000040;
		color: #fff;
		display: flex;
		height: 64px;
		margin: 16px auto;
		max-width: 1200px;
		padding: 16px;
		width: 100%;
		h1 {
			margin: 0 16px 0 8px;
		}
	}
	main {
		display: flex;
		flex-direction: column;
		min-height: calc(100vh - 94px);
		margin: 0 auto;
		max-width: 1200px;
		width: 100%;
	}
	footer {
		margin: auto auto 0 auto;
		max-width: 1200px;
		padding: 48px 0;
		width: 100%;
		p {
			color: hsl(315 10% 40%);
			font-size: .9em;
			font-weight: 300;
			a {
				color: hsl(315 60% 80%);
				font-weight: 400;
				text-decoration: none;
				transition: color .5s;
				&:hover {
					color: hsl(315 40% 90%);
				}
			}
		}
	}
</style>