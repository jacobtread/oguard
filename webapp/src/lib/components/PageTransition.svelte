<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fly } from 'svelte/transition';

	interface Props {
		url?: string;
		children?: Snippet;
	}

	const { url = '', children }: Props = $props();
</script>

<div class="transition-outer">
	{#key url}
		<div class="transition-inner" in:fly={{ y: -20, duration: 500 }}>
			{@render children?.()}
		</div>
	{/key}
</div>

<style>
	.transition-outer {
		display: grid;
		grid-template: 1fr 1fr;
	}
	.transition-inner {
		grid-row: 1;
		grid-column: 1;
	}
</style>
