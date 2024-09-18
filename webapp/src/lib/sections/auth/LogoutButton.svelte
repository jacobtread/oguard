<script lang="ts">
	import { t } from 'svelte-i18n';
	import { createMutation, useQueryClient } from '@tanstack/svelte-query';
	import { HttpMethod, requestText } from '$lib/api/utils';
	import type { LoginRequest } from '$lib/api/types';

	const client = useQueryClient();

	export let open = true;
	export let onClose: () => void = () => (open = false);

	const logoutMutation = createMutation({
		mutationFn: async () =>
			await requestText<LoginRequest>({
				method: HttpMethod.POST,
				route: `/api/logout`
			}),

		onSuccess: () => {
			client.invalidateQueries({ queryKey: ['login-state'] });
			onClose();
		}
	});
</script>

<button
	disabled={$logoutMutation.isPending}
	class="button"
	on:click={() => $logoutMutation.mutate()}>
	{$t('logout')}
</button>

<style lang="scss">
	@use '$lib/styles/palette.scss' as palette;
</style>
