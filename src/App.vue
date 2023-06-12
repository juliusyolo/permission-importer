<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  setup() {
const themeMedia = window.matchMedia("(prefers-color-scheme: light)");
    
// if (themeMedia.matches) {
//         console.log('light')
//     } else {
//         console.log('dark')
//     }
    return {

    }
  }
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
