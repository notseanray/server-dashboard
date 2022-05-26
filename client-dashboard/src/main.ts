import { createApp } from "vue";
import { createStore } from 'vuex'
import axios from 'axios'
import VueAxios from 'vue-axios'

import App from "./App.vue";
import router from "./router";
import createPersistedState from "vuex-persistedstate";

let storage = localStorage.getItem("relays");

export const store = createStore({
    state: {
      servers: (!!storage) ? storage?.split("|") : [],
      count: 0,
      enabled: true,
      items: [],
      unix: Date.now() / 1000,
    },
    plugins: [createPersistedState()],
  });

const app = createApp(App);

app.use(router);
app.use(store);

app.use(VueAxios, axios)
app.provide('axios', app.config.globalProperties.axios)  // provide 'axios'

app.mount("#app");


  