<script lang="ts">
	import '@fontsource-variable/inter';
	import '$lib/styles/global.scss';
	import { browser } from '$app/environment';
	import { QueryClientProvider, QueryClient } from '@tanstack/svelte-query';
	import { SvelteQueryDevtools } from '@tanstack/svelte-query-devtools';
	import dayjs from 'dayjs';
	import duration from 'dayjs/plugin/duration';
	import relativeTime from 'dayjs/plugin/relativeTime';
	import localizedFormat from 'dayjs/plugin/localizedFormat';

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser
			}
		}
	});

	dayjs.extend(duration);
	dayjs.extend(relativeTime);
	dayjs.extend(localizedFormat);
</script>

<QueryClientProvider client={queryClient}>
	<main>
		<slot />
	</main>
	<SvelteQueryDevtools />
</QueryClientProvider>
