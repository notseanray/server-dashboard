<script setup lang="ts">
import ServerItem from "./ServerItem.vue";
import DocumentationIcon from "./icons/IconDocumentation.vue";
import ToolingIcon from "./icons/IconTooling.vue";
import EcosystemIcon from "./icons/IconEcosystem.vue";
import CommunityIcon from "./icons/IconCommunity.vue";
import SupportIcon from "./icons/IconSupport.vue";
</script>

<template>
  <div class="serverBox">
    <ServerItem :test="42">
      test
    </ServerItem>
    <ServerItem>
      test
    </ServerItem>
    <ServerItem>
      test
    </ServerItem>
    <ServerItem>
      test
    </ServerItem>
    <ServerItem>
      test
    </ServerItem>
    <ServerItem>
      test
    </ServerItem>
    <ServerItem>
      test
    </ServerItem>
  </div>
</template>

<style>
@media (min-width: 1024px) {
  .serverBox {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
}
</style>