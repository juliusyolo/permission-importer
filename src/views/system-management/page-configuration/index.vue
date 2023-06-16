<template>
  <div class="container">
    <a-card class="general-card" :body-style="{ paddingTop: '10px' }">
      <a-tabs default-active-key="1">
        <a-tab-pane key="1">
          <template #title>
            <icon-interaction /> 模块管理
          </template>
          <module-component />
        </a-tab-pane>
        <a-tab-pane key="2">
          <template #title>
            <icon-menu /> 菜单管理
          </template>
          <menu-component />
        </a-tab-pane>
      </a-tabs>
    </a-card>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import MenuComponent from '@/views/system-management/page-configuration/components/menu/index.vue';
import ModuleComponent from '@/views/system-management/page-configuration/components/module/index.vue';

export default defineComponent({
  components: {
    MenuComponent,
    ModuleComponent,
  },
  setup() {
    return {};
  },
});
</script>

<style scoped>
:deep(.arco-table-th) {
  &:last-child {
    .arco-table-th-item-title {
      margin-left: 16px;
    }
  }
}
</style>
