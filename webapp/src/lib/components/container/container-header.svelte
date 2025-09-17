<script lang="ts">
	export let title: string | undefined = undefined;
	export let dark: boolean = false;
</script>

<div class="container__header" class:container__header--dark={dark}>
	{#if title !== undefined}
		<h2 class="container__header__title">{title}</h2>
	{/if}

	<slot />
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	$borderWidth: 0.1rem;
	$borderStyle: solid;
	$borderColor: #dfe3e8;
	$border: $borderWidth $borderStyle $borderColor;

	.container__header {
		display: flex;
		padding: 1rem;

		justify-content: space-between;
		align-items: center;

		border-bottom: $border;

		&--dark {
			background-color: palette.$gray-200;
		}
	}

	.container__header__title {
		font-size: 1.25rem;
		color: palette.$gray-800;
	}

	.container__header--dark .container__header__title {
		color: palette.$gray-700;
	}
</style>
