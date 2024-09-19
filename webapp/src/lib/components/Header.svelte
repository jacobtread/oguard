<script lang="ts">
	import { base } from '$app/paths';

	import EventsIcon from '~icons/solar/notification-unread-lines-bold-duotone';
	import HomeIcon from '~icons/solar/home-bold-duotone';
	import HistoryIcon from '~icons/solar/sort-by-time-bold-duotone';
	import ConfigureIcon from '~icons/solar/settings-bold-duotone';
	import PipelineIcon from '~icons/solar/square-sort-horizontal-bold-duotone';
	import MenuIcon from '~icons/solar/hamburger-menu-outline';

	import { page } from '$app/stores';
	import { t } from 'svelte-i18n';
	import LogoutButton from '$lib/sections/auth/LogoutButton.svelte';
	import { createLoginStateQuery } from '$lib/api/login';

	const loginStateQuery = createLoginStateQuery();

	let navVisible = false;
</script>

<div class="header">
	<div class="logo-area">
		<img class="logo" src="/oguard.svg" alt="OGuard Logo" />

		<h1 class="title">OGuard</h1>
	</div>
	<div class="nav-wrapper">
		<div
			class="nav-bg"
			class:nav-bg--visible={navVisible}
			aria-hidden="true"
			on:click={() => (navVisible = false)}>
			<nav class="nav">
				<ul class="nav-list">
					<li class="nav-list__item">
						<a class="nav-link" href="{base}/" class:nav-link--selected={$page.route.id == '/'}>
							<HomeIcon class="nav-link__item" />
							{$t('pages.home')}
						</a>
					</li>
					<li class="nav-list__item">
						<a
							class="nav-link"
							href="{base}/events"
							class:nav-link--selected={$page.route.id == '/events'}>
							<EventsIcon class="nav-link__item" />
							{$t('pages.events')}
						</a>
					</li>
					<li class="nav-list__item">
						<a
							class="nav-link"
							href="{base}/history"
							class:nav-link--selected={$page.route.id == '/history'}>
							<HistoryIcon class="nav-link__item" />
							{$t('pages.history')}
						</a>
					</li>
					<li class="nav-list__item">
						<a
							class="nav-link"
							href="{base}/pipelines"
							class:nav-link--selected={$page.route.id?.startsWith('/pipelines')}>
							<PipelineIcon class="nav-link__item" />
							{$t('pages.pipelines')}
						</a>
					</li>
					<li class="nav-list__item">
						<a
							class="nav-link"
							href="{base}/configure"
							class:nav-link--selected={$page.route.id == '/configure'}>
							<ConfigureIcon class="nav-link__item" />
							{$t('pages.configure')}
						</a>
					</li>
					<li class="nav-list__item">
						<a
							class="nav-link"
							href="{base}/realtime"
							class:nav-link--selected={$page.route.id == '/realtime'}>
							<ConfigureIcon class="nav-link__item" />
							{$t('pages.realtime')}
						</a>
					</li>
				</ul>
			</nav>
		</div>
	</div>

	<div class="end-actions">
		<button class="button menu-button" on:click={() => (navVisible = !navVisible)}>
			<MenuIcon />
		</button>

		{#if $loginStateQuery.data !== undefined && $loginStateQuery.data.logged_in}
			<LogoutButton />
		{/if}
	</div>
</div>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	.header {
		background-color: #fff;
		min-width: 15rem;
		text-align: center;
		display: flex;
		align-items: center;
		border-bottom: 0.1rem solid #dfe3e8;

		flex-flow: row;
		gap: 1rem;
		padding-left: 1rem;
		position: relative;

		min-height: 3.25rem;
		height: 3.25rem;
	}

	.end-actions {
		padding-right: 0.5rem;
	}

	.nav-wrapper {
		flex: auto;
	}

	.nav-list {
		display: flex;
		flex-flow: row;
		list-style: none;
		align-items: center;
		flex-wrap: wrap;
	}

	.nav-link {
		display: flex;
		align-items: center;
		vertical-align: middle;
		gap: 0.5rem;
		text-decoration: none;
		color: palette.$gray-700;
		padding: 1rem;

		&--selected {
			background: palette.$gray-700;
			color: #fff !important;
		}
	}

	.menu-button {
		padding: 0.1rem 0.25rem;
		font-size: 1.3rem;
		display: none;
		line-height: 1;
	}

	@media screen and (max-width: 70rem) {
		.menu-button {
			display: block;
		}

		.nav {
			background-color: palette.$gray-200;
		}

		.nav-bg {
			display: none;
			position: absolute;

			left: 0;
			top: 100%;

			width: 100%;
			height: calc(100vh - 3.25rem);

			background-color: rgba($color: #000000, $alpha: 0.5);
			z-index: 1;
		}

		.nav-bg--visible {
			display: block;
		}

		.nav-list {
			flex-flow: column;
			align-items: stretch;
		}

		.nav-link {
			width: 100%;
		}
	}

	.title {
		text-transform: uppercase;
		color: #34495e;
		font-size: 1.5rem;
	}

	.logo-area {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.logo {
		height: 2em;
	}
</style>
