<script lang="ts">
	import type { DeviceBatteryHistory } from '$/lib/api/types';
	import { AreaChart, ScaleTypes, type AreaChartOptions } from '@carbon/charts-svelte';
	import dayjs from 'dayjs';

	export let history: DeviceBatteryHistory[];

	const options: AreaChartOptions = {
		title: 'Battery Capacity (Today)',
		canvasZoom: { enabled: true },
		locale: {
			date(value) {
				return dayjs(value).format('L LT');
			}
		},
		zoomBar: {
			top: { enabled: true }
		},
		axes: {
			bottom: {
				title: 'Date',
				mapsTo: 'date',
				scaleType: ScaleTypes.TIME
			},
			left: {
				mapsTo: 'value',
				title: 'Battery Capacity',
				scaleType: ScaleTypes.LINEAR,
				percentage: true
			}
		},

		curve: 'curveMonotoneX',
		height: '400px'
	};

	const data = history.map((value) => ({
		date: value.created_at,
		value: value.state.capacity
	}));
</script>

<AreaChart {data} {options} style="padding:2rem;height:400px;" />
