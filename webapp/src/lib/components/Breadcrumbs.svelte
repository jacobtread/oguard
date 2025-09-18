<script lang="ts">
	import ArrowRight from '~icons/solar/alt-arrow-right-linear';

	interface BreadcrumbPart {
		label: string;
		href?: string;
	}

	interface Props {
		parts: BreadcrumbPart[];
	}

	const { parts }: Props = $props();
</script>

<div class="breadcrumbs">
	{#each parts as part, index}
		{#if part.href !== undefined}
			<a class="breadcrumbs__crumb" href={part.href}>{part.label}</a>
		{:else}
			<span class="breadcrumbs__crumb">{part.label}</span>
		{/if}

		{#if index < parts.length - 1}
			<span class="breadcrumbs__divider"><ArrowRight /></span>
		{/if}
	{/each}
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	.breadcrumbs {
		display: inline-flex;
		flex-flow: row;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5em 0.75em;
		font-size: 1rem;
		border-radius: 0.25rem;

		margin-bottom: 1rem;
	}

	.breadcrumbs__crumb {
		color: palette.$gray-600;
		text-decoration: none;
	}
	a.breadcrumbs__crumb {
		&:hover {
			text-decoration: underline;
		}
	}

	.breadcrumbs__divider {
		color: palette.$gray-600;
		display: flex;
		align-items: center;
	}
</style>
