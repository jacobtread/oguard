<script lang="ts">
	import AuthGuard from '$lib/components/auth/AuthGuard.svelte';
</script>

<AuthGuard>
	<slot />
</AuthGuard>
