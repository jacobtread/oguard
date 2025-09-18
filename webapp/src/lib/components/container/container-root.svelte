<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children?: Snippet;
	}

	const { children }: Props = $props();
</script>

<div class="container">
	{@render children?.()}
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.container {
		width: 100%;
		margin: 0 auto;

		background-color: #fff;
		border: $border;
		border-radius: 0.25rem;
	}
</style>
