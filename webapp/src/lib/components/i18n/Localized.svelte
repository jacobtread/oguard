<script lang="ts">
	import { t } from 'svelte-i18n';

	export let key: string;
</script>

{$t(key)}
