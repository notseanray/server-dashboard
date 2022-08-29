<script setup lang="ts">
import { defineProps } from "vue";
import { store } from "../main";
defineProps({
    ip: String,
    host_name: String,
    timestamp: Number,
    name: String,
    memory_used: Number,
    memory_total: Number,
    swap_used: Number,
    swap_total: Number,
    uptime: Number,
    kernel: String,
    disks: Array,
    cpu: String,
    cpu_temp: null,
    load_avg: Array,
    net: Array,
    core_count: Number,
});

const makeMB = (amt: Number): Number => {
    // not sure why, but the crate I'm using in the backend spits out a weird value
    // must divide by this much to get the correct value
    return (amt / 1073.75).toFixed(1);
};

const formatUptime = (uptime: Number): String => {
    if (uptime > 3600) {
        let hours = Math.floor(uptime / 3600);
        return `${hours} hrs ${((uptime - hours * 3600) / 60).toFixed(0)} min`;
    } else if (uptime > 60 && uptime < 3600) {
        return `${(uptime / 60).toFixed(2)} min`;
    }
    return `${uptime} sec`;
};

const loadAvgs = (la: Number[]): String => {
    return la.join(" ");
};

const loadStatus = (host_name: String, ip: String, kernel: String): String => {
    for (let i = 0; i < store.state.status.length; i++) {
        let status = store.state.status[i];
        if (status.name == host_name + "|" + ip + "|" + kernel) {
            return status.symbol;
        }
    }
    return "";
};

const divideStorage = (mb: Number): String => {
    return `${(mb / 1033216000).toFixed(1)} GiB`;
};

const displayNet = (tx: Number, rx: Number): String => {
    return `${(tx / 1073741824).toFixed(1)} GiB ↑ ${(rx / 1073741824).toFixed(
        1
    )} GiB ↓`;
};

const displayPackets = (ptx: Number, prx: Number, uptime: Number): String => {
    return `${(ptx / 1000 / uptime).toFixed(1)}K/s ↑ ${(
        prx /
        1000 /
        uptime
    ).toFixed(1)}K/s ↓`;
};

const formatCpu = (cpuName: String): String => {
    let newName = cpuName
        .replace("AMD ", "")
        .replace("Intel", "")
        .replace("(R)", "")
        .replace("(TM)", "")
        .replace("CPU", "");
    if (newName.length > 30) {
        return `${newName.slice(0, 30)}...`;
    }
    return newName;
};

const cpuTemp = (temp: number | null): number => {
    if (!temp) {
        return 0;
    }
    return temp.toFixed(1);
};
</script>

<template>
    <div class="item">
        <h3>
            name: {{ host_name }}
            <div style="float: right">kernel: {{ this.kernel }}</div>
            <br />
            ip:
            <div style="float: right">{{ this.ip }}</div>
            <br />
            cpu:
            <div style="float: right">
                {{ formatCpu(this.cpu) }} ({{ this.core_count }})
            </div>
            <br />
            load avg: {{ loadAvgs(this.load_avg) }}
            <div style="float: right">
                cpu temp: {{ cpuTemp(this.cpu_temp) }} °C
            </div>
            <br />
            ram:
            <div style="float: right">
                {{ makeMB(this.memory_used) }} MiB /
                {{ makeMB(this.memory_total) }} MiB
            </div>
            <br />
            swap:
            <div style="float: right">
                {{ makeMB(this.swap_used) }} MiB /
                {{ makeMB(this.swap_total) }} MiB
            </div>
            <br />
            uptime: {{ loadStatus(this.host_name, this.ip, this.kernel) }}
            <div style="float: right">{{ formatUptime(this.uptime) }}</div>
            <br />
            disks:
            <div class="diskBox">
                <div
                    v-for="item in this.disks.filter(
                        (x) =>
                            x.total_space > 10332160000 &&
                            !x.mnt_point.includes('docker')
                    )"
                    :key="item"
                >
                    <div class="disk">
                        {{ item.name.slice(0, item.name.length > 15 ? 15 : item.name.length) }} [{{ item.fs_type }}]
                        <br />
                        removable: {{ item.removable }}
                        <br />
                        mnt: {{ item.mnt_point.slice(0, item.mnt_point.length > 15 ? 15 : item.mnt_point.length) }}
                        <br />
                        {{ divideStorage(item.used_space) }} /
                        {{ divideStorage(item.total_space) }}
                    </div>
                </div>
            </div>
            net:
            <div class="diskBox">
                <div
                    v-for="item in this.net.filter((x) => x.tx > 10000)"
                    :key="item"
                >
                    <div class="disk">
                        {{ item.if_name }}
                        <br />
                        {{ displayNet(item.tx, item.rx) }}
                        <br />
                        mean packets:
                        <br />
                        {{ displayPackets(item.ptx, item.prx, this.uptime) }}
                    </div>
                </div>
            </div>
        </h3>
    </div>
</template>

<style scoped>
.item {
    display: inline-block;
    max-width: 35%;
	min-width: 350px;
    background-color: var(--vt-c-black-soft);
    box-shadow: 0 7px 12px 0 rgba(0, 0, 0, 0.4);
    transition: 0.3s;
    border-radius: 4px;
    margin: 4px;
    padding: 4px;
}

.item:hover {
    box-shadow: 0 10px 16px 0 rgba(0, 0, 0, 0.5);
}

.item:before {
    content: " ";
    border-left: 1px solid var(--color-border);
    position: absolute;
    left: 0;
    bottom: calc(50% + 25px);
    height: calc(50% - 25px);
}

.item:after {
    display: inline-block;
    font-size: 11px;
    content: " ";
    border-left: 1px solid var(--color-border);
    position: absolute;
    left: 0;
    top: calc(50% + 25px);
    height: calc(50% - 25px);
}

h3 {
    font-weight: 500;
    margin-bottom: 0.4rem;
    color: var(--color-heading);
}
.diskBox {
    max-width: 95%;
    margin-top: 4px;
	display: flex;
	flex-wrap: wrap;
}
.disk {
    padding: 2px;
	margin: 2px;
    max-width: 120px;
    display: inline-block;
    border: 1px solid;
    border-radius: 4px;
}
</style>
