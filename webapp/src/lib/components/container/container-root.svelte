<div class="container">
	<slot />
</div>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.container {
		width: 100%;
		margin: 0 auto;
		margin-top: 1rem;

		background-color: #fff;
		border: $border;
		border-radius: 0.25rem;
	}
</style>
