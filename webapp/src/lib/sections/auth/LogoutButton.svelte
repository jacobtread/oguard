<script lang="ts">
	import { createLogoutMutation } from '$lib/api/login';
	import { i18nContext } from '$lib/i18n/i18n.svelte';

	const i18n = i18nContext.get();

	const logoutMutation = createLogoutMutation();

	const onLogout = async () => {
		try {
			await logoutMutation.mutateAsync();
		} catch (err) {
			console.error('logout failed', err);
		}
	};
</script>

<button disabled={logoutMutation.isPending} class="button" onclick={onLogout}>
	{i18n.f('logout')}
</button>
