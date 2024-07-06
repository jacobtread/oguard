<script lang="ts">
	import '@fontsource-variable/inter';
	import '$lib/styles/global.scss';
	import '@carbon/charts-svelte/styles.css';
	import { browser } from '$app/environment';
	import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration';
	import relativeTime from 'dayjs/plugin/relativeTime';
	import localizedFormat from 'dayjs/plugin/localizedFormat';
	import updateLocale from 'dayjs/plugin/updateLocale';
	import utc from 'dayjs/plugin/utc';

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser
			}
		}
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

<QueryClientProvider client={queryClient}>
	<main>
		<slot />
	</main>
	<SvelteQueryDevtools />
</QueryClientProvider>
