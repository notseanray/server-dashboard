<script setup lang="ts">
import { ref } from "vue";
import {
    store,
    stringToColor,
    generateRange,
    defaultChartOptions,
    shortenLabel,
} from "../main";
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
    CategoryScale
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
		// @ts-expect-error
        type: Object as PropType<Partial<CSSStyleDeclaration>>,
        default: () => {},
    },
    plugins: {
		// @ts-expect-error
        type: Array as PropType<Plugin<"line">[]>,
        default: () => [],
    },
});
let chartData = ref<ChartData<"line">>({
    datasets: [],
});

setInterval(() => {
    let data = [];
    let length = 0;
    for (const d of store.state.TemperatureChart) {
        if (length == 0) {
			// @ts-expect-error
            length = d.data.length;
        }
        data.push({
			// @ts-expect-error
            label: shortenLabel(d.ip) + " temp",
			// @ts-expect-error
            backgroundColor: stringToColor(d.ip),
			// @ts-expect-error
            data: d.data,
        });
    }
    chartData.value = {
        labels: generateRange(length),
		// @ts-expect-error
        datasets: data,
    };
}, 10000);

setInterval(() => {
    if (
        !!store.state.TemperatureChart ||
		// @ts-expect-error
        !!store.state.TemperatureChart[0].data
    ) {
        return;
    }
    let newTemp = [];
	// @ts-expect-error
    for (const line of store.state.TemperatureChart) {
        let newLine: Number[] = [];
        let first = true;
        for (const dp of line.data) {
            if (first) {
                console.log("skipped")
                first = false;
                continue;
            }
            newLine.push(dp);
        }
        newTemp.push({ ip: line.ip, data: newLine });
    }
	// @ts-expect-error
    store.state.TemperatureChart = newTemp;
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
