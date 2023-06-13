<script lang="ts">
import {defineComponent, onMounted, reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {VersionControl} from "./model";

export default defineComponent({
  name: 'version-list',
  setup() {
    onMounted(()=>{
      getVersionControlData()
    })
    const columns = [{
      title: '版本',
      dataIndex: 'version',
    }, {
      title: '上线时间',
      dataIndex: 'releaseTime',
    }, {
      title: '操作',
      slotName: 'optional'
    }];
    const data = ref<VersionControl[]>([])
    const pagination = reactive({
      current: 1,
      pageSize: 10
    })
    const getVersionControlData = async () => {
      data.value = await invoke("get_version_control_list_by_pagination", {
        currentPage: pagination.current,
        pageSize: pagination.pageSize
      });
      console.log(data.value)
    }
    return {
      columns,
      data,
      pagination
    }
  }
})

</script>

<template>
  <a-table key="version" :pagination="pagination" :columns="columns" :data="data">
    <template #optional="{ record }">
      <a-space>
        <a-button @click="$modal.info({ title:'Name', content:record.name })">进入配置</a-button>
        <a-button @click="$modal.info({ title:'Name', content:record.name })">导出配置</a-button>
      </a-space>
    </template>
  </a-table>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
