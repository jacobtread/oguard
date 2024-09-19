<script lang="ts">
	import Spinner from '../components/Spinner.svelte';
</script>

<div class="loader-background">
	<Spinner />
</div>

<style lang="scss">
	.loader-background {
		width: 100%;
		height: 100vh;

		display: flex;
		align-items: center;
		justify-content: center;

		font-size: 6rem;
		color: #999;
	}
</style>
