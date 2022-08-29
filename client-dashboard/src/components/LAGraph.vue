<script setup lang="ts">
import { ref } from "vue";
import type { PropType } from "vue";
import {
    generateRange,
    store,
    stringToColorWithField,
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
		// @ts-expect-error
        type: Array as PropType<Plugin<'line'>>,
        default: () => [],
    },
});
let chartData = ref<ChartData<"line">>({
    datasets: [],
});

setInterval(() => {
    let dataOneMinute = [];
    let length = 352;
	const data = JSON.parse(JSON.stringify(store.state.loadAverageChartOne));
    for (let i = 0; i < data.length; i++) {
		const d = data[i];
        dataOneMinute.push({
            label: shortenLabel(d.ip) + " 1",
            backgroundColor: stringToColorWithField(d.ip, 1),
            data: d.data,
        });
    }
    for (const d of JSON.parse(JSON.stringify(store.state.loadAverageChartFive))) {
        dataOneMinute.push({
            label: shortenLabel(d.ip) + " 5",
            backgroundColor: stringToColorWithField(d.ip, 3),
            data: d.data,
        });
    }
    for (const d of JSON.parse(JSON.stringify(store.state.loadAverageChartFifteen))) {
        dataOneMinute.push({
            label: shortenLabel(d.ip) + " 15",
            backgroundColor: stringToColorWithField(d.ip, 6),
            data: d.data,
        });
    }

    chartData.value = {
        labels: generateRange(length),
		// @ts-expect-error
        datasets: dataOneMinute,
    };
}, 10000);

setInterval(() => {
    if (
        !!store.state.loadAverageChartOne ||
		// @ts-expect-error
        !!store.state.loadAverageChartOne[0].data
    ) {
        return;
    }
	// @ts-expect-error
    for (const d of store.state.loadAverageChartOne) {
        let newOne = [];
        let first = true;
        for (const dp of d.data) {
            if (first) {
                first = false;
                continue;
            }
            newOne.push(dp);
        }
        d.data = newOne;
    }
    for (const d of store.state.loadAverageChartFive) {
        let newOne = [];
        let first = true;
		// @ts-expect-error
        for (const dp of d.data) {
            if (first) {
                first = false;
                continue;
            }
            newOne.push(dp);
        }
		// @ts-expect-error
        d.data = newOne;
    }
    for (const d of store.state.loadAverageChartFifteen) {
        let newOne = [];
        let first = true;
		// @ts-expect-error
        for (const dp of d.data) {
            if (first) {
                first = false;
                continue;
            }
            newOne.push(dp);
        }
		// @ts-expect-error
        d.data = newOne;
    }
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
