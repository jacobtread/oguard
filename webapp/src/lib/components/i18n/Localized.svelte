<script lang="ts">
	import { i18nContext } from '$/lib/i18n/i18n.svelte';

	export let key: string;

	const i18n = i18nContext.get();
</script>

{i18n.f(key)}
