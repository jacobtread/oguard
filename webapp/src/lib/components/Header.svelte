<script lang="ts">
	import { base } from '$app/paths';

	import EventsIcon from '~icons/solar/notification-unread-lines-bold-duotone';
	import HomeIcon from '~icons/solar/home-bold-duotone';
	import HistoryIcon from '~icons/solar/sort-by-time-bold-duotone';
	import ConfigureIcon from '~icons/solar/settings-bold-duotone';
	import { page } from '$app/stores';
	import { _ } from 'svelte-i18n';
</script>

<div class="header">
	<div class="logo-area">
		<img class="logo" src="/oguard.svg" alt="OGuard Logo" />

		<h1 class="title">OGuard</h1>
	</div>
</div>
<div class="sub-header">
	<nav class="nav">
		<ul class="nav-list">
			<li class="nav-list__item" class:nav-link--selected={$page.route.id == '/'}>
				<a class="nav-link" href="{base}/">
					<HomeIcon class="nav-lin__item" />
					{$_('pages.home')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/events"
					class:nav-link--selected={$page.route.id == '/events'}
				>
					<EventsIcon class="nav-lin__item" />
					{$_('pages.events')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/history"
					class:nav-link--selected={$page.route.id == '/history'}
				>
					<HistoryIcon class="nav-lin__item" />
					{$_('pages.history')}
				</a>
			</li>
			<li class="nav-list__item">
				<a
					class="nav-link"
					href="{base}/configure"
					class:nav-link--selected={$page.route.id == '/configure'}
				>
					<ConfigureIcon class="nav-lin__item" />
					{$_('pages.configure')}
				</a>
			</li>
		</ul>
	</nav>
</div>

<style lang="scss">
	.header {
		min-width: 15rem;
		border: 1px solid #efefef;
		box-shadow: 2px 10px 20px rgba(0, 0, 0, 0.1);
		text-align: center;
		padding: 1rem;
		display: flex;
		align-items: center;

		flex-flow: row;
		gap: 1rem;
	}
	.sub-header {
		min-width: 15rem;
		background-color: #34495e;
		box-shadow: 2px 10px 20px rgba(0, 0, 0, 0.5);
		text-align: center;
		display: flex;
		align-items: center;

		flex-flow: row;
		gap: 1rem;
		margin-bottom: 0.5rem;
	}

	.nav-list {
		display: flex;
		flex-flow: row;
		list-style: none;
		gap: 1.5rem;
		align-items: center;
	}

	.nav-link {
		display: flex;
		gap: 0.25rem;
		text-decoration: none;
		color: #fff;
		padding: 1rem;

		&--selected {
			border-bottom: 0.2rem solid #ffffff;
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
