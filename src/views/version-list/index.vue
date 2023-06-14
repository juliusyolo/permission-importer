<script lang="ts">
import {defineComponent, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {VersionControl, VersionControlPaginationResult} from "./model";
import {PaginationProps} from "@arco-design/web-vue";

export default defineComponent({
  name: 'version-list',
  setup() {
    onMounted(() => {
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
    const loading = ref<boolean>(false)
    const addVersionControlModalVisible = ref<boolean>(false)
    const addVersionControlFormData = ref<VersionControl>({
      version: '',
      releaseTime:''
    });
    const addVersionControlFormRef = ref()
    const pagination = ref<PaginationProps>({
      current: 1,
      pageSize: 10,
      total: 0,
      showTotal: true
    })
    const versionControlPaginationResult = ref<VersionControlPaginationResult>();
    const getVersionControlData = async () => {
      try {
        loading.value = true
        versionControlPaginationResult.value = await invoke("get_version_control_list_by_pagination", {
          currentPage: pagination.value.current,
          pageSize: pagination.value.pageSize
        });
        data.value = versionControlPaginationResult.value?.data as VersionControl[]
        pagination.value.total = versionControlPaginationResult.value?.total
      } finally {
        loading.value = false
      }
    }
    const pageChange = (page: number) => {
      pagination.value.current = page
      getVersionControlData()
    }
    const addVersionControlClick = ()=>{
      addVersionControlModalVisible.value = true;
    }
    const handleBeforeOk = (done:any) => {
      if (addVersionControlFormData.value.version&&addVersionControlFormData.value.releaseTime){
        window.setTimeout(() => {
          done()
          addVersionControlFormRef.value.resetFields();
          addVersionControlFormRef.value.clearValidate();
          addVersionControlFormData.value = {};
        }, 3000)
      }else{
        addVersionControlFormRef.value.setFields({
          version: {
            status: 'error',
            message: '版本是必填项'
          },
          releaseTime: {
            status: 'error',
            message: '上线时间是必填项'
          }
        })
        done(false)
      }
    };
    const handleCancel = () => {
      addVersionControlFormRef.value.resetFields();
      addVersionControlFormRef.value.clearValidate();
      addVersionControlFormData.value = {};
      addVersionControlModalVisible.value = false;
    }
    return {
      columns,
      data,
      pagination,
      loading,
      pageChange,
      addVersionControlClick,
      addVersionControlModalVisible,
      addVersionControlFormRef,
      addVersionControlFormData,
      handleCancel,
      handleBeforeOk
    }
  }
})

</script>

<template>
  <a-space direction="vertical" style="width: 100%">
    <a-button type="primary" @click="addVersionControlClick">
      <template #icon>
        <icon-plus/>
      </template>
      新增版本
    </a-button>
    <a-table key="version" :pagination="pagination" :loading="loading" :columns="columns" :data="data"
             @page-change="pageChange">
      <template #optional="{ record }">
        <a-space>
          <a-button @click="$modal.info({ title:'Name', content:record.name })">进入配置</a-button>
          <a-button @click="$modal.info({ title:'Name', content:record.name })">导出配置</a-button>
        </a-space>
      </template>
    </a-table>
  </a-space>
  <a-modal v-model:visible="addVersionControlModalVisible" title="新增版本" @cancel="handleCancel" @before-ok="handleBeforeOk">
    <a-form ref="addVersionControlFormRef" :model="addVersionControlFormData">
      <a-form-item field="version" label="版本" :validate-trigger="['focus','input','blur','change']" :rules="[{required:true,message:'版本是必填项'}]">
        <a-input v-model="addVersionControlFormData.version" placeholder="请输入版本号" />
      </a-form-item>
      <a-form-item field="releaseTime" label="上线时间" :validate-trigger="['focus','input','blur','change']" :rules="[{required:true,message:'上线时间是必填项'}]">
        <a-date-picker
          showTime
          v-model="addVersionControlFormData.releaseTime"
          style="width: 100%"
        />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<style scoped>

</style>
