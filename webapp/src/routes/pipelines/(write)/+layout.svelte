<script lang="ts">
	import AuthGuard from '$lib/components/AuthGuard.svelte';
</script>

<AuthGuard>
	<slot />
</AuthGuard>
