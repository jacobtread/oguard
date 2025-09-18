<script lang="ts">
	import LoginDialog from '$lib/sections/auth/LoginDialog.svelte';
	import { createLoginStateQuery } from '$lib/api/login';
	import type { Snippet } from 'svelte';

	interface Props {
		children?: Snippet;
	}

	const { children }: Props = $props();

	const loginStateQuery = createLoginStateQuery();
</script>

{#if loginStateQuery.isError}
	<p>Error</p>
{:else if loginStateQuery.isLoading}
	<p>Loading</p>
{:else if loginStateQuery.data === undefined || !loginStateQuery.data.logged_in}
	<LoginDialog open onClose={() => {}} />
{:else}
	{@render children?.()}
{/if}
