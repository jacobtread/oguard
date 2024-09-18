<script lang="ts">
	import { t } from 'svelte-i18n';
	import { createLogoutMutation } from '$/lib/api/login';

	export let open = true;
	export let onClose: () => void = () => (open = false);

	const logoutMutation = createLogoutMutation();

	const onLogout = async () => {
		try {
			await $logoutMutation.mutateAsync();
		} catch (err) {
			console.error('logout failed', err);
		} finally {
			onClose();
		}
	};
</script>

<button disabled={$logoutMutation.isPending} class="button" on:click={onLogout}>
	{$t('logout')}
</button>
