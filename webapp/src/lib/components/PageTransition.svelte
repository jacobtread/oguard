<script lang="ts">
	import { fly } from 'svelte/transition';
	export let url: string = '';
</script>

<div class="transition-outer">
	{#key url}
		<div class="transition-inner" in:fly={{ y: -20, duration: 500 }}>
			<slot />
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
