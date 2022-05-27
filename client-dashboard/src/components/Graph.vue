<script setup lang="ts">
import { defineComponent, h } from 'vue'
import { Line } from 'vue-chartjs'
import { store } from "../main";
import { Chart as ChartJS, Title, Tooltip, Legend, CategoryScale, LinearScale, LineElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineElement, CategoryScale, LinearScale)

defineProps({
 chartId: {
      type: String,
      default: 'line-chart'
    },
    width: {
      type: Number,
      default: 400
    },
    height: {
      type: Number,
      default: 400
    },
    cssClasses: {
      default: '',
      type: String
    },
    styles: {
      type: Object as PropType<Partial<CSSStyleDeclaration>>,
      default: () => {}
    },
    plugins: {
      type: Object as PropType<PluginOptionsByType<'line'>>,
      default: () => {}
    }
})

let oldData = [];
let d = store.state.loadAverageChart;
for (let i = 0; i < d.length; i++) {
    oldData.push(
        {
            data: []
        }
    );
}

const chartData = {
    labels: [ 'January', 'February', 'March' ],
    datasets: [ 
        { 
            label: 'My First dataset',
            backgroundColor: 'rgb(255, 99, 132)',
            borderColor: 'rgb(255, 99, 132)',
            data: [40, 20, 12] 
        } 
    ]
}

const chartOptions = { responsive: true }
</script>

<template>
  <Line
    :chart-options="chartOptions"
    :chart-data="chartData"
    :chart-id="chartId"
    :dataset-id-key="datasetIdKey"
    :plugins="plugins"
    :css-classes="cssClasses"
    :styles="styles"
    width="40vw"
    :height="height"
  />
</template>

