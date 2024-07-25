<script lang="ts">
	export let indent: boolean = false;
</script>

<div class="container__section" class:container__section--indent={indent}>
	<slot />
</div>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.container__section {
		justify-content: space-between;
		align-items: center;
		padding: 1rem;

		&:not(:last-child) {
			border-bottom: $border;
		}

		&--indent {
			padding: 1rem;
			background-color: palette.$gray-200;
		}
	}
</style>
