<script lang="ts">
	import type { Duration } from '$lib/api/types';

	export let duration: Duration;

	let hours = Math.floor(duration.secs / 3600);
	let minutes = Math.floor((duration.secs % 3600) / 60);
	let seconds = duration.secs % 60;

	function updateDuration() {
		duration.secs = hours * 3600 + minutes * 60 + seconds;
	}

	// eslint-disable-next-line @typescript-eslint/no-unused-expressions
	$: hours, minutes, seconds, updateDuration();
</script>

<div class="parts">
	<label class="part">
		Hours
		<input class="part__input" type="number" bind:value={hours} min="0" on:input={updateDuration} />
	</label>
	<label class="part">
		Minutes
		<input
			class="part__input"
			type="number"
			bind:value={minutes}
			min="0"
			max="59"
			on:input={updateDuration} />
	</label>
	<label class="part">
		Seconds
		<input
			class="part__input"
			type="number"
			bind:value={seconds}
			min="0"
			max="59"
			on:input={updateDuration} />
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
