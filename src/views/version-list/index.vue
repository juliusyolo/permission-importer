<script lang="ts">
import {defineComponent, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {VersionControl, VersionControlPaginationResult} from "./model";
import {Message, PaginationProps} from "@arco-design/web-vue";
import {useRouter} from "vue-router";

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
    const exportConfigurationModalVisible = ref<boolean>(false)
    const addVersionControlFormData = ref<VersionControl>({
      version: '',
      releaseTime: ''
    });
    const addVersionControlFormRef = ref()
    const exportConfigurationModel = ref<string[]>(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
    const selectedVersionControlModel = ref<VersionControl>()
    const router = useRouter()
    const pagination = ref<PaginationProps>({
      current: 1,
      pageSize: 10,
      total: 0,
      showTotal: true
    })
    const versionControlPaginationResult = ref<VersionControlPaginationResult>();
    const getVersionControlData = () => {
      loading.value = true
      invoke("get_version_control_list_by_pagination", {
        currentPage: pagination.value.current,
        pageSize: pagination.value.pageSize
      }).then(result => {
        versionControlPaginationResult.value = result as VersionControlPaginationResult
        data.value = versionControlPaginationResult.value?.dataList as VersionControl[]
        pagination.value.total = versionControlPaginationResult.value?.total
      }).catch(errorMsg => {
        Message.error({content: errorMsg, duration: 1000})
      }).finally(() => {
        loading.value = false;
      });
    }
    const pageChange = (page: number) => {
      pagination.value.current = page
      getVersionControlData()
    }
    const addVersionControlClick = () => {
      addVersionControlModalVisible.value = true;
    }
    const exportConfigurationClick = (record: VersionControl) => {
      selectedVersionControlModel.value = record
      exportConfigurationModalVisible.value = true
    }
    const enterConfigurationClick = (record:VersionControl) =>{
      router.push({
        path:'/system-management/page-configuration',
        query: {...record}
      })
    }
    const handleAddVersionControlBeforeOk = async (done: any) => {
      if (addVersionControlFormData.value.version && addVersionControlFormData.value.releaseTime) {
        console.log("hello")
        invoke("save_version_control", {
          version: addVersionControlFormData.value.version,
          releaseTime: addVersionControlFormData.value.releaseTime
        }).then((result) => {
          pagination.value.current = 1;
          getVersionControlData();
          done()
          addVersionControlFormRef.value.resetFields();
          addVersionControlFormRef.value.clearValidate();
          addVersionControlFormData.value = {};
        }).catch((errorMsg) => {
          Message.error({content: errorMsg, duration: 1000})
          done(false)
        })
      } else {
        addVersionControlFormRef.value.setFields({
          version: addVersionControlFormData.value.version ? undefined : {
            status: 'error',
            message: '版本是必填项'
          },
          releaseTime: addVersionControlFormData.value.releaseTime ? undefined : {
            status: 'error',
            message: '上线时间是必填项'
          }
        })
        done(false)
      }
    };
    const handleAddVersionControlCancel = () => {
      addVersionControlFormRef.value.resetFields();
      addVersionControlFormRef.value.clearValidate();
      addVersionControlFormData.value = {};
      addVersionControlModalVisible.value = false;
    }
    const handleExportConfigurationCancel = () => {
      exportConfigurationModel.value = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
      exportConfigurationModalVisible.value = false;
    }

    const handleExportConfigurationBeforeOk = () => {

    }
    return {
      columns,
      data,
      pagination,
      loading,
      pageChange,
      enterConfigurationClick,
      addVersionControlClick,
      exportConfigurationClick,
      getVersionControlData,
      addVersionControlModalVisible,
      exportConfigurationModalVisible,
      addVersionControlFormRef,
      addVersionControlFormData,
      exportConfigurationModel,
      handleAddVersionControlCancel,
      handleAddVersionControlBeforeOk,
      handleExportConfigurationCancel,
      handleExportConfigurationBeforeOk
    }
  }
})

</script>

<template>
  <div style="width: calc(100% - 10px);padding: 5px">
    <a-space direction="vertical" style="width: 100%">
      <a-space>
        <a-button type="primary" @click="addVersionControlClick">
          <template #icon>
            <icon-plus/>
          </template>
          新增版本
        </a-button>
        <a-button type="primary" @click="getVersionControlData">
          <template #icon>
            <icon-refresh/>
          </template>
          刷新
        </a-button>
      </a-space>

      <a-table key="version" :pagination="pagination" :loading="loading" :columns="columns" :data="data"
               @page-change="pageChange">
        <template #optional="{ record }">
          <a-space>
            <a-button @click="enterConfigurationClick(record)">进入配置</a-button>
            <a-button @click="exportConfigurationClick(record)">导出配置</a-button>
          </a-space>
        </template>
      </a-table>
    </a-space>
    <a-modal :top="20" :align-center="false" v-model:visible="addVersionControlModalVisible" title="新增版本"
             @cancel="handleAddVersionControlCancel"
             @before-ok="handleAddVersionControlBeforeOk">
      <a-form ref="addVersionControlFormRef" :model="addVersionControlFormData">
        <a-form-item field="version" label="版本" :validate-trigger="['focus','input','blur','change']"
                     :rules="[{required:true,message:'版本是必填项'}]">
          <a-input v-model="addVersionControlFormData.version" placeholder="请输入版本号"/>
        </a-form-item>
        <a-form-item field="releaseTime" label="上线时间" :validate-trigger="['focus','input','blur','change']"
                     :rules="[{required:true,message:'上线时间是必填项'}]">
          <a-date-picker
            showTime
            v-model="addVersionControlFormData.releaseTime"
            style="width: 100%"
          />
        </a-form-item>
      </a-form>
    </a-modal>
    <a-modal :top="20" :align-center="false" v-model:visible="exportConfigurationModalVisible" title="导出配置"
             @cancel="handleExportConfigurationCancel"
             @before-ok="handleExportConfigurationBeforeOk">
      <a-checkbox-group direction="vertical" v-model="exportConfigurationModel">
        <a-checkbox value="8" disabled>角色</a-checkbox>
        <a-checkbox value="8" disabled>岗位</a-checkbox>
        <a-checkbox value="9" disabled>流程</a-checkbox>
        <a-checkbox value="1" disabled>菜单</a-checkbox>
        <a-checkbox value="2" disabled>模块</a-checkbox>
        <a-checkbox value="3" disabled>功能点</a-checkbox>
        <a-checkbox value="4" disabled>控制点</a-checkbox>
        <a-checkbox value="5" disabled>资源点</a-checkbox>
        <a-checkbox value="6" disabled>数据权限点</a-checkbox>
        <a-checkbox value="7" disabled>权限授权记录</a-checkbox>
        <a-checkbox value="10">机构</a-checkbox>
        <a-checkbox value="11">用户</a-checkbox>
        <a-checkbox value="12">机构角色关系</a-checkbox>
        <a-checkbox value="13">机构岗位关系</a-checkbox>
        <a-checkbox value="14">用户机构关系</a-checkbox>
        <a-checkbox value="15">用户机构角色关系</a-checkbox>
        <a-checkbox value="16">用户机构岗位关系</a-checkbox>
      </a-checkbox-group>
    </a-modal>
  </div>

</template>

<style scoped>

</style>
