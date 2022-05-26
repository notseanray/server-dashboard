<script setup lang="ts">
import ServerItem from "./ServerItem.vue";
import DocumentationIcon from "./icons/IconDocumentation.vue";
import ToolingIcon from "./icons/IconTooling.vue";
import EcosystemIcon from "./icons/IconEcosystem.vue";
import CommunityIcon from "./icons/IconCommunity.vue";
import SupportIcon from "./icons/IconSupport.vue";
import { store } from "../main";
import { inject } from 'vue'
const axios: any = inject('axios')  // inject axios
const getList = () => {
  for (let ip of store.state.servers) {
    setTimeout(() => {
      axios
        .get("http://" + ip + "/data")
        .then((response: { data: any }) => {
          for (let i = 0; i < response.data.length; i++) {
            let newItems = store.state.items.filter(x => x.ip == ip);
            newItems.push(response.data[i]);
            store.state.items = newItems;
          }
      });
    }, 500);
  }
};
getList();
setInterval(() => {
  if (store.state.enabled) {
    store.state.unix = Date.now() / 1000;
    console.log(Date.now() / 1000);
    getList();
  }
}, 5000);
</script>

<template>
  <div class="serverBox">
    <div v-for="item in store.state.items" :key="item">
        <ServerItem 
          :host_name="item.host_name"
          :timestamp="item.timestamp"
          :name="item.name"
          :kernel="item.kernel"
          :memory_used="item.memory_used"
          :memory_total="item.memory_total"
          :swap_used="item.swap_used"
          :swap_total="item.swap_total"
          :uptime="item.uptime"
          :load_avg="item.load_avg"
          :cpu_temp="item.cpu_temp"
        />
    </div>
  </div>
</template>

<style>
.serverBox {
  display: flex;
  flex-wrap: wrap;
  flex-direction: row;
  align-items: center;
}
</style>