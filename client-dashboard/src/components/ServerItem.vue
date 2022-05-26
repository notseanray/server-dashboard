<script setup lang="ts">
import { defineProps } from "vue";
import { store } from "../main";
defineProps({
    host_name: String,
    timestamp: Number,
    name: String,
    memory_used: Number,
    memory_total: Number,
    swap_used: Number,
    swap_total: Number,
    uptime: Number,
    kernel: String,
    cpu: String,
    cpu_temp: Number,
    load_avg: [Number],
    cpus: [Number],
});

const makeMB = (amt: Number): Number => {
  // not sure why, but the crate I'm using in the backend spits out a weird value
  // must divide by this much to get the correct value
  return (amt / 1073.75).toFixed(1);
}

const formatUptime = (uptime: Number): String => {
  if (uptime > 3600) {
    return `${(uptime / 3600).toFixed(2)} hrs`;
  } else if (uptime > 60 && uptime < 3600) {
    return `${(uptime / 60).toFixed(2)} min`;
  }
  return `${uptime} sec`;
}

const showStatus = (old_timestamp: Number): String => {
  return (store.state.unix + 5 < old_timestamp) ? "⚠" : "";
}

const loadAvgs = (la: Number[]): String => {
  return la.join(" ");
}

</script>

<template>
  <div class="item">
    <div class="details">
      <h3>
        name: {{host_name}} <div style="float: right"> kernel: {{this.kernel}} </div>
        <br>
        load avg: {{loadAvgs(this.load_avg)}} <div style="float: right"> {{this.cpu_temp.toFixed(1)}} °C </div>
        <br>
        ram <div style="float: right"> {{makeMB(this.memory_used)}} MiB / {{makeMB(this.memory_total)}} MiB </div>
        <br>
        swap <div style="float: right"> {{makeMB(this.swap_used)}} MiB / {{makeMB(this.swap_total)}} MiB </div>
        <br>
        uptime {{showStatus(this.timestamp)}} <div style="float: right"> {{formatUptime(this.uptime)}} </div> 
      </h3>
      <slot></slot>
    </div>
  </div>
</template>

<style scoped>
.item {
  flex: 1 0 50%;
  margin: 4px;
  background-color: var(--vt-c-black-soft);
}

h3 {
  font-size: 1.2rem;
  font-weight: 500;
  margin-bottom: 0.4rem;
  color: var(--color-heading);
}

@media (min-width: 1024px) {
  .item {
    box-shadow: 0 7px 12px 0 rgba(0,0,0,0.4);
	  transition: 0.3s;
    border-radius: 4px;
    margin: 20px;
    padding: 20px;
    min-width: 300px;
  }
  .item:hover {
    box-shadow: 0 10px 16px 0 rgba(0,0,0,0.5);
  }

  i {
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
}
</style>
