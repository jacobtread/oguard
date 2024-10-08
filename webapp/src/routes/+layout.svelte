<script lang="ts">
	import '@fontsource-variable/inter';
	import '$lib/styles/global.scss';
	import '@carbon/charts-svelte/styles.css';

	import { QueryClientProvider } from '@tanstack/svelte-query';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';

	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration';
	import relativeTime from 'dayjs/plugin/relativeTime';
	import localizedFormat from 'dayjs/plugin/localizedFormat';
	import updateLocale from 'dayjs/plugin/updateLocale';
	import utc from 'dayjs/plugin/utc';
	import timezone from 'dayjs/plugin/timezone';

	import { getLocaleFromNavigator, init, register } from 'svelte-i18n';
	import { Toaster } from 'svelte-sonner';
	import { navigating, page } from '$app/stores';
	import { queryClient } from '$lib/api/utils';

	import PageTransition from '$lib/components/PageTransition.svelte';
	import Header from '$lib/components/Header.svelte';
	import Alert, { AlertType } from '$/lib/components/Alert.svelte';
	import LoadingScreen from '$/lib/sections/LoadingScreen.svelte';

	register('en', () => import('../locales/en.json'));

	const i18nPromise = init({
		fallbackLocale: 'en',
		initialLocale: getLocaleFromNavigator() ?? 'en',
		ignoreTag: false
	});

	dayjs.extend(duration);
	dayjs.extend(relativeTime, {
		thresholds: [
			{ l: 's', r: 1 },
			{ l: 'ss', r: 1 },
			{ l: 'm', r: 1 },
			{ l: 'mm', r: 59, d: 'minute' },
			{ l: 'h', r: 1 },
			{ l: 'hh', r: 23, d: 'hour' },
			{ l: 'd', r: 1 },
			{ l: 'dd', r: 29, d: 'day' },
			{ l: 'M', r: 1 },
			{ l: 'MM', r: 11, d: 'month' },
			{ l: 'y' },
			{ l: 'yy', d: 'year' }
		]
	});
	dayjs.extend(localizedFormat);
	dayjs.extend(updateLocale);
	dayjs.extend(utc);
	dayjs.extend(timezone);

	dayjs.updateLocale('en', {
		relativeTime: {
			future: 'in %s',
			past: '%s ago',
			s: 'a few seconds',
			ss: '%d seconds',
			m: 'a minute',
			mm: '%d minutes',
			h: 'an hour',
			hh: '%d hours',
			d: 'a day',
			dd: '%d days',
			M: 'a month',
			MM: '%d months',
			y: 'a year',
			yy: '%d years'
		}
	});
</script>

<svelte:head>
	<title>OGuard</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	{#await i18nPromise}
		<LoadingScreen />
	{:then}
		<div class="layout">
			<Header />
			{#if $navigating}
				<LoadingScreen />
			{:else}
				<main class="main">
					<PageTransition url={$page.url.toString()}>
						<slot />
					</PageTransition>
				</main>
			{/if}
		</div>
	{:catch error}
		<Alert type={AlertType.ERROR} message={`Failed to load translations: ${error.messages}`} />
	{/await}

	<Toaster />

	<SvelteQueryDevtools />
</QueryClientProvider>

<style lang="scss">
	.layout {
		display: flex;
		flex-flow: column;
		width: 100%;
		background-color: #f4f6f8;
		height: 100vh;
	}

	.main {
		flex: auto;
		overflow: auto;
	}
</style>
