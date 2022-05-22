import { createApp } from "vue";
import { createStore } from 'vuex'

import App from "./App.vue";
import router from "./router";

const store = createStore({
    state () {
        return {
            count: 0
        }
    },
    mutations: {
        increment (state) {
            state.count++
        }
    }
})

const app = createApp(App);

app.use(router);
app.use(store);

app.mount("#app");
