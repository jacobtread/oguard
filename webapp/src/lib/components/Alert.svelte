<script module lang="ts">
	export const AlertType = {
		SUCCESS: 'success',
		INFO: 'info',
		WARN: 'warn',
		ERROR: 'error'
	} as const;

	type AlertType = (typeof AlertType)[keyof typeof AlertType];
</script>

<script lang="ts">
	import ErrorIcon from '~icons/solar/danger-triangle-bold-duotone';
	import InfoIcon from '~icons/solar/info-circle-bold-duotone';
	import CheckIcon from '~icons/solar/check-circle-bold-duotone';

	interface Props {
		type: AlertType;
		message: string;
	}

	const { type, message }: Props = $props();
</script>

<div class="alert" data-type={type}>
	{#if type === AlertType.INFO}
		<InfoIcon />
	{:else if type === AlertType.SUCCESS}
		<CheckIcon />
	{:else if type === AlertType.WARN}
		<ErrorIcon />
	{:else if type === AlertType.ERROR}
		<ErrorIcon />
	{/if}

	<p class="alert__message">{message}</p>
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	.alert {
		padding: 1rem;
		display: flex;
		gap: 1rem;

		&[data-type='success'] {
			background-color: #22c55e;
			color: #118d57;
		}
		&[data-type='info'] {
			background-color: #00b8d9;
			color: #006c9c;
		}
		&[data-type='warn'] {
			background-color: #ffab00;
			color: #b76e00;
		}
		&[data-type='error'] {
			background-color: rgba(#ff5630, 0.4);
			color: #b71d18;
		}
	}
</style>
