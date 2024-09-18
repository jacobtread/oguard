<script lang="ts">
	import AuthGuard from '$/lib/sections/auth/AuthGuard.svelte';
</script>

<AuthGuard>
	<slot />
</AuthGuard>
