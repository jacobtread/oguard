<script lang="ts">
	import AuthGuard from '$lib/sections/auth/AuthGuard.svelte';
	import type { LayoutProps } from './$types';

	const { children }: LayoutProps = $props();
</script>

<AuthGuard>
	{@render children?.()}
</AuthGuard>
