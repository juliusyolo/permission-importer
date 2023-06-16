<template>
  <a-menu mode="horizontal" :default-selected-keys="['page-configuration']" @menu-item-click="menuItemClick">
    <a-menu-item key="0" :style="{ padding: 0, marginRight: '38px' }" disabled>
      <div
        :style="{
            cursor: 'pointer',
          }"
        @click="backToHome"
      >
        {{ version }}
      </div>
    </a-menu-item>
    <a-menu-item v-for="menuKey in Object.keys(menus)" :key="menuKey">{{ menus[menuKey] }}</a-menu-item>
  </a-menu>
  <div style="width: calc(100% - 20px);padding: 10px">
    <a-space direction="vertical">
      <a-breadcrumb>
        <a-breadcrumb-item>
          <icon-home/>
        </a-breadcrumb-item>
        <a-breadcrumb-item>系统配置</a-breadcrumb-item>
        <a-breadcrumb-item>{{ menus[currentMenuKey] }}</a-breadcrumb-item>
      </a-breadcrumb>
      <router-view/>
    </a-space>
  </div>
</template>

<script lang="ts">

import {defineComponent, onMounted, reactive, ref} from "vue";
import {useRoute, useRouter} from "vue-router";

export default defineComponent({

  setup() {
    const route = useRoute();
    const router = useRouter();
    const version = ref<string>()
    const query = ref()
    const currentMenuKey = ref<string>()
    const menus = reactive({'page-configuration': '页面配置', 'relation-permission': '组织权限配置'})
    onMounted(() => {
      query.value = route.query
      version.value = route.query.version as string
      currentMenuKey.value = route.path.replaceAll("/system-management/", "")
    })
    const backToHome = () => {
      router.push("/")
    }
    const menuItemClick = (key: string) => {
      currentMenuKey.value = key
      router.push({path: key, query: query.value})
    }
    return {
      version,
      query,
      menus,
      currentMenuKey,
      backToHome,
      menuItemClick
    }
  }
})
</script>

<style scoped></style>
