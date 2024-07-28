<script lang="ts">
	import { base } from '$app/paths';

	import EventsIcon from '~icons/solar/notification-unread-lines-bold-duotone';
	import HomeIcon from '~icons/solar/home-bold-duotone';
	import HistoryIcon from '~icons/solar/sort-by-time-bold-duotone';
	import ConfigureIcon from '~icons/solar/settings-bold-duotone';
	import PipelineIcon from '~icons/solar/square-sort-horizontal-bold-duotone';
	import { page } from '$app/stores';
	import { _ } from 'svelte-i18n';
	import LogoutButton from './auth/LogoutButton.svelte';
	import { createLoginStateQuery } from '$lib/api/login';

	const loginStateQuery = createLoginStateQuery();
</script>

<div class="header">
	<div class="logo-area">
		<img class="logo" src="/oguard.svg" alt="OGuard Logo" />

		<h1 class="title">OGuard</h1>
	</div>

	<nav class="nav">
		<ul class="nav-list">
			<li class="nav-list__item">
				<a class="nav-link" href="{base}/" class:nav-link--selected={$page.route.id == '/'}>
					<HomeIcon class="nav-link__item" />
					{$_('pages.home')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/events"
					class:nav-link--selected={$page.route.id == '/events'}>
					<EventsIcon class="nav-link__item" />
					{$_('pages.events')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/history"
					class:nav-link--selected={$page.route.id == '/history'}>
					<HistoryIcon class="nav-link__item" />
					{$_('pages.history')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/pipelines"
					class:nav-link--selected={$page.route.id?.startsWith('/pipelines')}>
					<PipelineIcon class="nav-link__item" />
					{$_('pages.pipelines')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/configure"
					class:nav-link--selected={$page.route.id == '/configure'}>
					<ConfigureIcon class="nav-link__item" />
					{$_('pages.configure')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/realtime"
					class:nav-link--selected={$page.route.id == '/realtime'}>
					<ConfigureIcon class="nav-link__item" />
					{$_('pages.realtime')}
				</a>
			</li>
		</ul>
	</nav>

	{#if $loginStateQuery.data !== undefined && $loginStateQuery.data.logged_in}
		<div class="end-actions">
			<LogoutButton />
		</div>
	{/if}
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
	}

	.end-actions {
		padding-right: 0.5rem;
	}

	.nav {
		flex: auto;
	}

	.nav-list {
		display: flex;
		flex-flow: row;
		list-style: none;
		align-items: center;
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
