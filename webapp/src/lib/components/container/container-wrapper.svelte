<script lang="ts">
	export let maxWidth: 'xs' | 'sm' | 'md' | 'lg' = 'lg';
</script>

<div class="wrapper" data-max-width={maxWidth}>
	<slot />
</div>

<style lang="scss">
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
