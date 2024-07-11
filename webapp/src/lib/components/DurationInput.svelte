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

<div>
	<label>
		Hours:
		<input type="number" bind:value={hours} min="0" on:input={updateDuration} />
	</label>
	<label>
		Minutes:
		<input type="number" bind:value={minutes} min="0" max="59" on:input={updateDuration} />
	</label>
	<label>
		Seconds:
		<input type="number" bind:value={seconds} min="0" max="59" on:input={updateDuration} />
	</label>
</div>
