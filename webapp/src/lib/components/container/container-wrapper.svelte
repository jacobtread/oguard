<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		maxWidth?: 'xs' | 'sm' | 'md' | 'lg';
		children?: Snippet;
	}

	const { maxWidth = 'lg', children }: Props = $props();
</script>

<div class="wrapper" data-max-width={maxWidth}>
	{@render children?.()}
</div>

<style>
	.wrapper {
		padding: 1rem;
		width: 100%;
		max-width: 80rem;
		margin: 0 auto;
	}

	.wrapper[data-max-width='lg'] {
		max-width: 80rem;
	}

	.wrapper[data-max-width='md'] {
		max-width: 70rem;
	}

	.wrapper[data-max-width='sm'] {
		max-width: 60rem;
	}

	.wrapper[data-max-width='xs'] {
		max-width: 40rem;
	}
</style>
