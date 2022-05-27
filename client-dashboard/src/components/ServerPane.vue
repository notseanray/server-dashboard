<script setup lang="ts">
import ServerItem from "./ServerItem.vue";
import { store } from "../main";
import { inject } from 'vue'
const axios: any = inject('axios')  // inject axios
let storedData = store.state.items;
for (let ip of store.state.servers) {
  setTimeout(() => {
    axios
      .get("http://" + ip + "/data_all")
      .then((response: { data: any }) => {
        for (let i = 0; i < response.data.length; i++) {
          let r = response.data[i];
          storedData.push(r);
          store.state.loadAverageChart = storedData;
        }
    });
  }, 500);
}
const getList = () => {
  let newItems = store.state.items;
  let newStatus = store.state.status;
  for (let ip of store.state.servers) {
    setTimeout(() => {
      axios
        .get("http://" + ip + "/data")
        .then((response: { data: any }) => {
          for (let i = 0; i < response.data.length; i++) {
            let r = response.data[i];
            newItems = newItems.filter(x => x.ip != r.ip);
            newItems.push(r);
            store.state.items = newItems;
            newStatus = newStatus.filter(x => x.name != r.host_name + "|" + r.ip + "|" + r.kernel)
            newStatus.push({
              name: r.host_name + "|" + r.ip + "|" + r.kernel,
              time: Date.now() / 1000,
              symbol: ""
            });
            store.state.status = newStatus;
          }
      });
    }, 500);
  }
};
getList();
setInterval(() => {
  if (store.state.enabled) {
    for (let i = 0; i < store.state.status.length; i++) {
      let element = store.state.status[i];
      if (element.time + 6 < Date.now() / 1000) {
        element.symbol = "⚠";
      } else {
        element.symbol = "";
      }
    }
    getList();
  }
}, 5000);
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
  display: flex;
  flex-flow: row nowrap;
  justify-content: space-around;
}
.serverBox > div {
   flex-basis: 50%;
}
</style>