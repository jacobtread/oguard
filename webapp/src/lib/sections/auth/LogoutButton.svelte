<script lang="ts">
	import { createLogoutMutation } from '$/lib/api/login';
	import { i18nContext } from '$/lib/i18n/i18n.svelte';

	const i18n = i18nContext.get();

	export let open = true;
	export let onClose: () => void = () => (open = false);

	const logoutMutation = createLogoutMutation();

	const onLogout = async () => {
		try {
			await logoutMutation.mutateAsync();
		} catch (err) {
			console.error('logout failed', err);
		} finally {
			onClose();
		}
	};
</script>

<button disabled={logoutMutation.isPending} class="button" on:click={onLogout}>
	{i18n.f('logout')}
</button>
