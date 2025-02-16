<script lang="ts">
	import { onMount } from 'svelte';
	
	import '$lib/interface/styles/root.scss';
	import BrandLogo from '$lib/interface/visuals/brand_logo.svelte';
	
	let header_hover = false;
	onMount(() => header_hover = document.body.scrollTop > 0);
</script>

<div class="cool_top_blur" class:hover={header_hover}/>
<nav>
	<div class="header" class:hover={header_hover}>
		<a class="brand_logo" href="/" title="Ribbon">
			<BrandLogo height={30}/>
		</a>
		<div class="links">
			<a href="/dashboard">
				Dashboard
			</a>
		</div>
	</div>
</nav>
<main>
	<slot/>
</main>
<footer>
	<div class="brand">
		<BrandLogo height={32}/>
		<p>
			An experimental Roblox-centric Discord Bot developed by <a href="https://hakumi.cafe" target="_blank">HAKUMI</a>.
		</p>
	</div>
	<div class="links">
		<a href="/terms">
			Terms
		</a>
		<a href="/privacy">
			Privacy
		</a>
	</div>
</footer>

<svelte:document on:scroll={() => header_hover = document.documentElement.scrollTop > 0}/>

<style lang="scss">
	.cool_top_blur {
		backdrop-filter: blur(6px);
		background: color-mix(in oklab, hsl(315 10% 9%) 80%, transparent);
		height: 96px;
		left: 0;
		mask-image: linear-gradient(#000 20%, transparent 80%);
		opacity: 0;
		position: fixed;
		top: 0;
		transition: opacity .5s;
		width: 100%;
		&.hover {
			opacity: 1;
		}
	}
	nav {
		left: 0;
		padding: 0 16px;
		position: fixed;
		top: 0;
		width: 100%;
		.header {
			align-items: center;
			backdrop-filter: blur(16px);
			border-radius: 24px;
			display: flex;
			height: 64px;
			margin: 16px auto;
			max-width: 1200px;
			padding: 16px 32px;
			transition: background .2s, box-shadow .2s, padding .5s;
			&.hover {
				background: hsla(315, 10%, 5%, .3);
				box-shadow: inset 0 0 0 1px hsla(315, 80%, 90%, .15);
				padding: 16px 24px;
			}
			.brand_logo {
				color: #fff;
				margin: 0 16px 0 0;
			}
			.links {
				margin: 0 0 0 auto;
				a {
					color: #fff;
					font-weight: 500;
					text-decoration: none;
				}
			}
		}
	}
	main {
		min-height: calc(100vh - 94px);
		margin: 0 auto;
		max-width: 1200px;
		padding: 80px 16px;
		width: 100%;
	}
	footer {
		align-items: center;
		display: flex;
		margin: auto auto 0 auto;
		max-width: 1200px;
		padding: 48px 16px;
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
		.links {
			display: flex;
			gap: 32px;
			margin-left: auto;
			a {
				color: hsl(315 20% 60%);
				font-size: .9em;
				font-weight: 400;
				text-decoration: none;
				transition: color .5s;
				&:hover {
					color: hsl(315 30% 80%);
				}
			}
		}
	}
</style>