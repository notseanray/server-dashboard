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
  cpu_temp: Number,
  load_avg: Array,
  net: Array,
  core_count: Number,
});

const makeMB = (amt: Number): Number => {
  // not sure why, but the crate I'm using in the backend spits out a weird value
  // must divide by this much to get the correct value
  return (amt / 1073.75).toFixed(1);
}

const formatUptime = (uptime: Number): String => {
  if (uptime > 3600) {
    let hours = Math.floor(uptime / 3600);
    return `${hours} hrs ${((uptime - (hours * 3600)) / 60).toFixed(0)} min`;
  } else if (uptime > 60 && uptime < 3600) {
    return `${(uptime / 60).toFixed(2)} min`;
  }
  return `${uptime} sec`;
}

const loadAvgs = (la: Number[]): String => {
  return la.join(" ");
}

const loadStatus = (host_name: String, ip: String, kernel: String): String => {
  for (let i = 0; i < store.state.status.length; i++) {
    let status = store.state.status[i];
    if (status.name == host_name + "|" + ip + "|" + kernel) {
      return status.symbol;
    }
  }
  return "";
}

const divideStorage = (mb: Number): String => {
  return `${(mb / 1033216000).toFixed(1)} GiB`;
}

const displayNet = (tx: Number, rx: Number): String => {
  return `${(tx / 1073741824).toFixed(1)} GiB ↑ ${(rx / 1073741824).toFixed(1)} GiB ↓`;
}

const displayPackets = (ptx: Number, prx: Number, uptime: Number): String => {
  return `${((ptx / 1000) / uptime).toFixed(1)}K/s ↑ ${((prx / 1000) / uptime).toFixed(1)}K/s ↓`;
}

const formatCpu = (cpuName: String): String => {
  if (cpuName.length > 30) {
    return `${cpuName.slice(0, 30)}...`;
  }
  return cpuName;
}
</script>

<template>
  <div class="item">
    <h3>
      name: {{host_name}} <div style="float: right"> kernel: {{this.kernel}} </div>
      <br>
      ip: <div style="float: right"> {{this.ip}} </div>
      <br>
      cpu: <div style="float: right"> {{formatCpu(this.cpu)}} ({{this.core_count}}) </div>
      <br>
      load avg: {{loadAvgs(this.load_avg)}} <div style="float: right">cpu temp: {{this.cpu_temp.toFixed(1)}} °C </div>
      <br>
      ram: <div style="float: right"> {{makeMB(this.memory_used)}} MiB / {{makeMB(this.memory_total)}} MiB </div>
      <br>
      swap: <div style="float: right"> {{makeMB(this.swap_used)}} MiB / {{makeMB(this.swap_total)}} MiB </div>
      <br>
      uptime: {{loadStatus(this.host_name, this.ip, this.kernel)}} <div style="float: right"> {{formatUptime(this.uptime)}} </div> 
      <br>
      disks: 
      <div class="diskBox">
        <div v-for="item in this.disks" :key="item">
          <div class="disk">
            {{item.name}} [{{item.fs_type}}] 
            <br>
            removable: {{item.removable}}
            <br>
            mnt: {{item.mnt_point}}
            <br>
            {{divideStorage(item.used_space)}} / {{divideStorage(item.total_space)}} 
          </div>
        </div>
      </div>
      net:
      <div class="diskBox">
        <div v-for="item in this.net" :key="item">
          <div class="disk">
            {{item.if_name}}
            <br>
            {{displayNet(item.tx, item.rx)}} 
            <br>
            mean packets:
            <br>
            {{displayPackets(item.ptx, item.prx, this.uptime)}} 
          </div>
        </div>
      </div>
    </h3>
  </div>
</template>

<style scoped>
.item {
  margin: 4px;
  background-color: var(--vt-c-black-soft);
}

h3 {
  font-size: 1.2rem;
  font-weight: 500;
  margin-bottom: 0.4rem;
  color: var(--color-heading);
}
.diskBox {
  margin-top: 8px;
  display: flex;
  flex-flow: row nowrap;
  justify-content: space-around;
}
.serverBox > div {
   flex-basis: 50%;
}
.disk {
  padding: 4px;
  flex: 1 0 30%;
  border: 1px solid;
  border-radius: 4px;
}
.item {
  box-shadow: 0 7px 12px 0 rgba(0,0,0,0.4);
  transition: 0.3s;
  border-radius: 4px;
  margin: 10px;
  padding: 10px;
  min-width: 22vw;
}
.item:hover {
  box-shadow: 0 10px 16px 0 rgba(0,0,0,0.5);
}

i {
  transition: 0.4s;
  top: calc(50% - 25px);
  left: -26px;
  position: absolute;
  border: 1px solid var(--color-border);
  background: var(--color-background);
  border-radius: 8px;
  width: 50px;
  height: 50px;
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
  content: " ";
  border-left: 1px solid var(--color-border);
  position: absolute;
  left: 0;
  top: calc(50% + 25px);
  height: calc(50% - 25px);
}

.item:first-of-type:before {
  display: none;
}

.item:last-of-type:after {
  display: none;
}
</style>
