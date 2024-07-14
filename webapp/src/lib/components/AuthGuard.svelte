<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import { HttpMethod, requestJson } from '$lib/api/utils';
	import type { LoginState } from '$lib/api/types';
	import LoginDialog from './LoginDialog.svelte';

	const loginStateQuery = createQuery<LoginState>({
		queryKey: ['login-state'],
		queryFn: async () =>
			await requestJson<LoginState>({
				method: HttpMethod.GET,
				route: '/api/login-state'
			})
	});
</script>

{#if $loginStateQuery.isError}
	<p>Error</p>
{:else if $loginStateQuery.isLoading}
	<p>Loading</p>
{:else if $loginStateQuery.data === undefined || !$loginStateQuery.data.logged_in}
	<LoginDialog open onClose={() => {}} />
{:else}
	<slot />
{/if}

<style lang="scss">
	@use '../styles/palette.scss' as palette;
</style>
