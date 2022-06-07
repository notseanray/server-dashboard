<script setup lang="ts">
import { defineComponent, ref } from "vue";
import { store, stringToColor, generateRange, defaultChartOptions, shortenLabel } from "../main";
import { Line } from "vue-chartjs";
import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
} from "chart.js";

import type { ChartData } from "chart.js/auto";

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
);

defineProps({
    chartId: {
        type: String,
        default: "line-chart",
    },
    width: {
        type: Number,
        default: 350,
    },
    height: {
        type: Number,
        default: 250,
    },
    cssClasses: {
        default: "",
        type: String,
    },
    styles: {
        type: Object as PropType<Partial<CSSStyleDeclaration>>,
        default: () => {},
    },
    plugins: {
      type: Array as PropType<Plugin<'line'>[]>,
      default: () => []
    }
});
let chartData = ref<ChartData<'line'>>({
	datasets: []
});

setInterval(() => {
	let data = [];
    let length = 0;
	for (const d of store.state.RamChart) {
        if (length == 0) {
            length = d.data.length;
        }
		data.push({
			label: shortenLabel(d.ip) + " ram",
			backgroundColor: stringToColor(d.ip),
			data: d.data,
		})
	}
	chartData.value = { 
        labels: generateRange(length),
		datasets: data
	};
}, 10000);
</script>

<template>
    <Line
        :chart-options="defaultChartOptions"
        :chart-data="chartData"
        :chart-id="chartId"
        :plugins="plugins"
        :css-classes="cssClasses"
        :styles="styles"
        :width="width"
        :height="height"
    />
</template>
