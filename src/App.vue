<script setup lang="ts">
import {defineComponent, onMounted} from "vue";
import {useDark} from "@vueuse/core";
onMounted(() => {
  useDark({
    selector: 'body',
    attribute: 'arco-theme',
    valueDark: 'dark',
    valueLight: 'light',
  })
})
</script>

<template>
  <router-view/>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
