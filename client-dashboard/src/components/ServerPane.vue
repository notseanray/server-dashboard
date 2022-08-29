<script setup lang="ts">
// @ts-nocheck
import ServerItem from "./ServerItem.vue";
import { store } from "../main";
import { inject } from "vue";
const axios: any = inject("axios"); // inject axios
let first = true;
let trigger = true;
let ipMap: { ip: String; position: Number }[] = [];
let updateList: { index: number; dp: number }[] = [];
let updateList5: { index: number; dp: number }[] = [];
let updateList15: { index: number; dp: number }[] = [];
let temperatureGraph: { index: number; dp: number }[] = [];
let ramGraph: { index: number; dp: number }[] = [];

function find(ip: String) {
    for (let i = 0; i < ipMap.length; i++) {
        if (ipMap[i].ip == ip) {
            return i;
        }
    }
    return -1;
}
const getList = () => {
    if (trigger) {
        return;
    }
    let newItems: any[] = store.state.items;
    let newStatus: any[] = store.state.status;
    const getNew = (ip: String) => {
        let oneMinute: any[] = [];
        let fiveMinute: any[] = [];
        let fifteenMinute: any[] = [];
        setTimeout(() => {
            fetch("http://" + ip + "/data")
                .then((res) => res.json())
                .then((response: any) => {
                    for (let i = 0; i < response.length; i++) {
                        let r: any = response[i];
                        const label = ip + "|" + r.ip;
                        let assign = find(label);
                        if (assign < 0) {
                            oneMinute.push({
                                ip: label,
                                data: [],
                            });
                            fiveMinute.push({
                                ip: label,
                                data: [],
                            });
                            fifteenMinute.push({
                                ip: label,
                                data: [],
                            });
                            ipMap.push({ ip: ip + "|" + r.ip, position: i });
                            assign = i;
                        }
                        newItems = newItems.filter((x: any) => x.ip != r.ip);
                        newItems.push(r);
                        store.state.items = newItems;
                        newStatus = newStatus.filter(
                            (x: any) =>
                                x.name !=
                                r.host_name + "|" + r.ip + "|" + r.kernel
                        );
                        newStatus.push({
                            name: r.host_name + "|" + r.ip + "|" + r.kernel,
                            time: Date.now() / 1000,
                            symbol: "",
                        });
                        store.state.status = newStatus;
                        updateList.push({
                            index: assign,
                            dp: r.load_avg[0] / r.core_count,
                        });
                        updateList5.push({
                            index: assign,
                            dp: r.load_avg[1] / r.core_count,
                        });
                        updateList15.push({
                            index: assign,
                            dp: r.load_avg[2] / r.core_count,
                        });
                        temperatureGraph.push({
                            index: assign,
                            dp: r.cpu_temp,
                        });
                        ramGraph.push({
                            index: assign,
                            dp: (r.memory_used * 100) / r.memory_total,
                        });
                    }
                });
        }, 500);
    };
    for (const ip of store.state.servers) {
        getNew(ip);
    }
};

const updateRam = () => {
    if (trigger) {
        return;
    }
    let newRam = [];
    for (const line of store.state.RamChart) {
        let newLine: Number[] = [];
        let first = true;
        for (const dp of line.data) {
            if (first) {
                first = false;
                continue;
            }
            newLine.push(dp);
        }
        newRam.push({ ip: line.ip, data: newLine });
    }
    store.state.RamChart = newRam;
};

const updateTemp = () => {
    if (trigger) {
        return;
    }
    let newTemp = [];
    for (const line of store.state.TemperatureChart) {
        let newLine: Number[] = [];
        let first = true;
        for (const dp of line.data) {
            if (first) {
                first = false;
                continue;
            }
            newLine.push(dp);
        }
        newTemp.push({ ip: line.ip, data: newLine });
    }
    store.state.TemperatureChart = newTemp;
};

setInterval(() => {
    updateRam();
    updateTemp();
}, 10000);

setInterval(() => {
    if (trigger) {
        return;
    }
    for (const dp of temperatureGraph) {
		// @ts-expect-error
        store.state.TemperatureChart[dp.index].data.push(dp.dp);
    }
    for (const dp of updateList) {
		// @ts-expect-error
        store.state.loadAverageChartOne[dp.index].data.push(dp.dp);
    }
    for (const dp of updateList5) {
		// @ts-expect-error
        store.state.loadAverageChartFive[dp.index].data.push(dp.dp);
    }
    for (const dp of updateList15) {
		// @ts-expect-error
        store.state.loadAverageChartFifteen[dp.index].data.push(dp.dp);
    }
    for (const dp of ramGraph) {
		// @ts-expect-error
        store.state.RamChart[dp.index].data.push(dp.dp);
    }
    temperatureGraph.length = 0;
    updateList.length = 0;
    updateList5.length = 0;
    updateList15.length = 0;
    ramGraph.length = 0;
}, 10000);

const load = () => {
    let oneMinute: any[] = [];
    let fiveMinute: any[] = [];
    let fifteenMinute: any[] = [];
    let temperatureInitial: any[] = [];
    let ramInitial: any[] = [];
    for (let ip of store.state.servers) {
        setTimeout(() => {
            axios
                .get("http://" + ip + "/data_all")
                .then((response: { data: any }) => {
                    for (let i = 0; i < response.data.length; i++) {
                        let r = response.data[i];
                        const label = ip + "|" + r.ip;
                        let assign = find(label);
                        if (assign < 0) {
                            oneMinute.push({
                                ip: label,
                                data: [],
                            });
                            fiveMinute.push({
                                ip: label,
                                data: [],
                            });
                            fifteenMinute.push({
                                ip: label,
                                data: [],
                            });
                            temperatureInitial.push({
                                ip: label,
                                data: [],
                            });
                            ramInitial.push({
                                ip: label,
                                data: [],
                            });
                            ipMap.push({ ip: ip + "|" + r.ip, position: i });
                            assign = i;
                        }
                        oneMinute[assign].data.push(
                            r.load_avg[0] / r.core_count
                        );
                        fiveMinute[assign].data.push(
                            r.load_avg[1] / r.core_count
                        );
                        fifteenMinute[assign].data.push(
                            r.load_avg[2] / r.core_count
                        );
                        temperatureInitial[assign].data.push(r.cpu_temp);
                        ramInitial[assign].data.push(
                            (r.memory_used * 100) / r.memory_total
                        );
                        trigger = false;
                    }
                });
        }, 1000);
    }
	// @ts-expect-error
    store.state.loadAverageChartOne = oneMinute;
	// @ts-expect-error
    store.state.loadAverageChartFive = fiveMinute;
	// @ts-expect-error
    store.state.loadAverageChartFifteen = fifteenMinute;
	// @ts-expect-error
    store.state.TemperatureChart = temperatureInitial;
	// @ts-expect-error
    store.state.RamChart = ramInitial;
};

setInterval(() => {
    for (let i = 0; i < store.state.status.length; i++) {
        let element = store.state.status[i];
		// @ts-expect-error
        if (element.time + 11 < Date.now() / 1000) {
			// @ts-expect-error
            element.symbol = "⚠";
        } else {
			// @ts-expect-error
            element.symbol = "";
        }
    }
    if (first) {
        first = false;
        load();
    }
    getList();
}, 10000);
</script>

<template>
    <div class="serverBox">
        <div v-for="item in store.state.items" :key="item">
            <ServerItem
                :ip="item.ip"
                :host_name="item.host_name"
                :timestamp="item.timestamp"
                :name="item.name"
                :kernel="item.kernel"
                :memory_used="item.memory_used"
                :memory_total="item.memory_total"
                :swap_used="item.swap_used"
                :swap_total="item.swap_total"
                :uptime="item.uptime"
                :disks="item.disks"
                :net="item.net"
                :cpu="item.cpu"
                :core_count="item.core_count"
                :load_avg="item.load_avg"
                :cpu_temp="item.cpu_temp"
            />
        </div>
    </div>
</template>

<style>
.serverBox {
    font-size: 9px;
    display: flex;
    flex-wrap: wrap;
}
</style>
