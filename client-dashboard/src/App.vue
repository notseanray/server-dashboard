<script setup lang="ts">
import { store } from "./main";
import LAGraph from "./components/LAGraph.vue";
import TGraph from "./components/TGraph.vue";
import RamGraph from "./components/RamGraph.vue";
const addServer = (ip: string) => {
  if (ip.includes("http://")) {
    return;
  }
  let formated = ip.replace(/$[/]/g, "").trim();
  if (!store.state.servers.includes(formated)) {
    store.state.servers.push(formated);
    localStorage.setItem("relays", store.state.servers.join("|"));
  }
}

const removeRelay = (ip: string) => {
  let newData = store.state.servers.filter(c => c != ip);
  store.state.servers = newData;
  localStorage.setItem("relays", store.state.servers);
}
</script>

<template>
  <header>
    <div class="wrapper">
      <nav>
        <div class="graphPane">
            <LAGraph />
            <TGraph />
            <RamGraph />
        </div>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/details">Details</RouterLink>
        <br>
	      <input class="addButton" v-model="this.message" placeholder="127.0.0.1:0000" />
        <button class="addButton" @click="addServer(this.message)">
          add relay
        </button>
        <br>
        <br>
        <div class="text" v-if="!!store.state.servers && store.state.servers.length > 0"> Relays: </div>
        <div class="relayList">
          <div v-for="item in store.state.servers" :key="item">
            <button class="minimalButton" @click="removeRelay(item)">
              {{item}}
            </button>
          </div>
        </div>
      </nav>
    </div>
    <RouterView />
  </header>
</template>

<style>
@import "@/assets/base.css";

#app {
  width: 100%;
  margin: 0 auto;
  padding: 2rem;
  background-color: var(--color-background);
  font-weight: normal;
  min-height: 100vh;
}

.graphPane {
  width: 45vw;
}

.text {
  color: var(--vt-c-text-dark-1);
  font-size: 16px;
}

.addButton {
  padding: 4px;
  margin: 4px;
  font-family: Iosevka;
  color: var(--vt-c-text-dark-1);
  background-color: var(--vt-c-black-soft);
  outline: none;
  border: 0px solid;
  border-radius: 3px;
  font-size: 14px;
}

.minimalButton, .minimalButton:focus {
  font-family: Iosevka;
  color: var(--vt-c-black-soft);
  outline: none;
  background: transparent;
  border: 0px solid;
}

.minimalButton:hover {
  box-shadow: 0 4px 8px 0 rgba(0,0,0,0.5);
  text-decoration: line-through;
}

.relayList {
  display: flex;
}

header {
  width: 100%;
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

a {
  text-decoration: none;
  margin: 8px;
  color: var(--vt-c-white-soft);
  transition: 0.4s;
}

@media (hover: hover) {
  a:hover {
    background-color: var(--vt-c-divider-light-1);
  }
}

nav {
  width: 100%;
  font-size: 20px;
  text-align: center;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 1rem;
  border-radius: 4px;
  font-size: 16px;
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  body {
    display: flex;
    place-items: center;
  }

  #app {
    display: grid;
    grid-template-columns: 1fr 1fr;
    padding: 0 2rem;
  }

  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  header .wrapper {
    width: 45vw;
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;
    width: 100vw;
    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
