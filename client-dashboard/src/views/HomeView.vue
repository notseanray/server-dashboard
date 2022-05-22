<script setup lang="ts">
import ServerPane from "@/components/ServerPane.vue";
</script>

<template>
  <div class="home">
    <ServerPane />
  </div>
</template>

<style>
@media (min-width: 1024px) {
  .home {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>