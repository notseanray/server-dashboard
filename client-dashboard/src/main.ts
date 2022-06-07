import { createApp } from "vue";
import { createStore } from 'vuex'
import axios from 'axios'
import VueAxios from 'vue-axios'

import App from "./App.vue";
import router from "./router";
import createPersistedState from "vuex-persistedstate";

let storage = localStorage.getItem("relays");

export const stringToColor = (ip: any): String => {
  if (!ip) {
    return "#008000";
  }
  let hash = 12;
  for (let i = 0; i < ip.length; i++) {
    let char = ip.charCodeAt(i);
    if (char > 1000000 || char < -1000000) {
      char = 1000000;
    }
    hash += char;
  }
  let hex = Math.abs(hash * hash).toString(16);
  if (hex.length < 6) {
    for (let i = hex.length; i < 6; i++) {
      hex += "f";
    }
  }
  return "#" + hex.slice(0, 6);
}

export const shortenLabel = (ip: any): any => {
  if (!ip) {
    return ip;
  }
  let separator = ip.indexOf("|");
  if (separator < 0) {
    return ip;
  }
  let shortened = ip.slice(separator + 1);
  let ipSeparator = ip.indexOf(":");
  if (ipSeparator < 0) {
    return shortened;
  }
  let extraShortened = shortened.slice(0, ipSeparator);
  return extraShortened;
}

export const defaultChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
};

export const generateRange = (max: number) => {
  let range: number[] = [];
  for (let i = 0; i < max; i++) {
      range.push(i);
  }
  return range;
}

export const stringToColorWithField = (ip: any, field: number): String => {
  if (!ip) {
    return "#008000";
  }
  let hash = 12;
  for (let i = 0; i < ip.length; i++) {
    let char = ip.charCodeAt(i);
    if (char > 1000000 || char < -1000000) {
      char = 1000000;
    }
    hash += char;
  }
  let hex = Math.abs(hash * hash * field).toString(16);
  if (hex.length < 6) {
    for (let i = hex.length; i < 6; i++) {
      hex += "f";
    }
  }
  return "#" + hex.slice(0, 6);
}

export const store = createStore({
    state: {
      servers: (!!storage) ? storage?.split("|") : [],
      count: 0,
      enabled: true,
      items: [],
      unix: Date.now() / 1000,
      status: [],
      loadAverageChartOne: [[]],
      loadAverageChartFive: [[]],
      loadAverageChartFifteen: [[]],
      TemperatureChart: [[]],
      RamChart: [[]],
    },
    plugins: [createPersistedState()],
  });

const app = createApp(App);

app.use(router);
app.use(store);

app.use(VueAxios, axios)
app.provide('axios', app.config.globalProperties.axios)  // provide 'axios'

app.mount("#app");


  