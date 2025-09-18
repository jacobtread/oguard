<script lang="ts">
	import type { Duration } from '$lib/api/types';
	import { watch } from 'runed';

	interface Props {
		duration: Duration;
	}

	let { duration = $bindable() }: Props = $props();

	let hours = $state(0);
	let minutes = $state(0);
	let seconds = $state(0);

	// Update the parts when the duration changes
	watch(
		() => duration,
		(duration) => {
			let newHours = Math.floor(duration.secs / 3600);
			let newMinutes = Math.floor((duration.secs % 3600) / 60);
			let newSeconds = duration.secs % 60;

			if (newHours !== hours || newMinutes !== minutes || newSeconds !== seconds) {
				hours = newHours;
				minutes = newMinutes;
				seconds = newSeconds;
			}
		}
	);

	// Update duration when hours, minutes, or seconds change
	watch(
		() => ({ hours, minutes, seconds }),
		({ hours, minutes, seconds }) => {
			duration.secs = hours * 3600 + minutes * 60 + seconds;
		}
	);
</script>

<div class="parts">
	<label class="part">
		Hours
		<input class="part__input" type="number" bind:value={hours} min="0" />
	</label>
	<label class="part">
		Minutes
		<input class="part__input" type="number" bind:value={minutes} min="0" max="59" />
	</label>
	<label class="part">
		Seconds
		<input class="part__input" type="number" bind:value={seconds} min="0" max="59" />
	</label>
</div>

<style lang="scss">
	@use '$styles/palette.scss' as palette;

	.part__input {
		padding: 0.5rem 0.75rem;
		font-size: 0.9rem;
		border-radius: 0.25rem;
		cursor: pointer;
		background-color: palette.$gray-100;
		border: 0.1rem solid palette.$gray-300;
		text-decoration: none;
		display: block;
		width: 100%;
		margin-top: 0.5rem;
	}

	.parts {
		display: flex;
		gap: 1rem;
		padding: 0.5rem 0;
	}

	.part {
		display: block;
		flex: auto;
		width: 100%;
		font-size: 0.9rem;
		color: palette.$gray-700;
	}
</style>
