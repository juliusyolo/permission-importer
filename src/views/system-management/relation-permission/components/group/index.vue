<template>
  <a-card class="general-card" :title="$t('group.form.query.name')">
    <a-row>
      <a-col :flex="1">
        <a-form
          :model="formModel"
          :label-col-props="{ span: 4 }"
          :wrapper-col-props="{ span: 20 }"
          label-align="left"
        >
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="name" :label="$t('group.form.name')">
                <a-input
                  v-model="formModel.groupName"
                  :placeholder="$t('group.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="name" :label="$t('group.form.code')">
                <a-input
                  v-model="formModel.groupCode"
                  :placeholder="$t('group.form.code.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" :label="$t('group.form.status')">
                <a-select
                  v-model="formModel.groupStatus"
                  :options="statusOptions"
                  :placeholder="$t('group.form.selectDefault')"
                />
              </a-form-item>
            </a-col>
          </a-row>
        </a-form>
      </a-col>
      <a-divider style="height: 84px" direction="vertical" />
      <a-col :flex="'86px'" style="text-align: right">
        <a-space direction="vertical" :size="18">
          <a-button type="primary" @click="search">
            <template #icon>
              <icon-search />
            </template>
            {{ $t('group.columns.operations.query') }}
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh />
            </template>
            {{ $t('group.columns.operations.reset') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-divider style="margin-top: 0" />
    <a-row style="margin-bottom: 16px">
      <a-col :span="16">
        <a-space>
          <a-button v-show="addVisible" type="primary" @click="handleAddModal">
            <template #icon>
              <icon-plus />
            </template>
            {{ $t('group.columns.operations.add') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-table
      row-key="groupId"
      :loading="loading"
      :pagination="{
        ...pagination,
        showPageSize: true,
        showTotal: true,
        onPageSizeChange: onPageSizeChange,
      }"
      :data="renderData"
      :bordered="false"
      @pageChange="onPageChange"
    >
      <template #columns>
        <a-table-column
          :title="$t('group.columns.groupName')"
          data-index="groupName"
        />
        <a-table-column
          :title="$t('group.columns.groupCode')"
          data-index="groupCode"
        />
        <a-table-column
          :title="$t('group.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="$t('group.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="$t('group.columns.groupStatus')"
          data-index="groupStatus"
        >
          <template #cell="{ record }">
            <span v-if="record.groupStatus === '0'" class="circle"></span>
            <span v-else class="circle pass"></span>
            {{
              $t(
                `group.form.status.${
                  record.groupStatus === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="$t('group.columns.operations')"
          data-index="operations"
        >
          <template #cell="{ record }">
            <a-button
              v-show="editVisible"
              type="text"
              size="small"
              @click="handleEditModal(record)"
            >
              {{ $t('group.columns.operations.edit') }}
            </a-button>
            <a-button
              v-show="viewVisible"
              type="text"
              size="small"
              @click="handleViewModal(record)"
            >
              {{ $t('group.columns.operations.view') }}
            </a-button>
            <a-button
              v-show="groupUserVisible"
              type="text"
              size="small"
              @click="handleGroupUserModal(record)"
            >
              {{ $t('group.columns.operations.groupUser') }}
            </a-button>
            <a-popconfirm
              :content="$t('group.delete.alert.message')"
              :ok-text="$t('global.button.confirm')"
              :cancel-text="$t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDeleteOk(record)"
            >
              <a-button v-show="deleteVisible" type="text" size="small">
                {{ $t('group.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.groupStatus === '0'"
              :content="$t('group.enable.alert.message')"
              :ok-text="$t('global.button.confirm')"
              :cancel-text="$t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleEnableOk(record)"
            >
              <a-button v-show="enableVisible" type="text" size="small">
                {{ $t('group.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="$t('group.disable.alert.message')"
              :ok-text="$t('global.button.confirm')"
              :cancel-text="$t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDisableOk(record)"
            >
              <a-button v-show="disableVisible" type="text" size="small">
                {{ $t('group.columns.operations.disable') }}
              </a-button>
            </a-popconfirm>
          </template>
        </a-table-column>
      </template>
    </a-table>
  </a-card>
  <a-modal
    v-model:visible="modalVisible"
    unmount-on-close
    :modal-style="{ width: '800px' }"
  >
    <template v-if="isAdd" #title> {{ $t('group.add.modal.title') }}</template>
    <template v-else-if="isEdit" #title>
      {{ $t('group.edit.modal.title') }}</template
    >
    <template v-else #title> {{ $t('group.view.modal.title') }}</template>
    <template v-if="isAdd || isEdit" #footer>
      <a-button @click="handleModalCancel">{{
        $t('global.button.cancel')
      }}</a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleModalConfirm"
        >{{ $t('global.button.confirm') }}</a-button
      >
    </template>
    <template v-else #footer><span></span></template>
    <a-form
      ref="modalFormRef"
      :model="modalModel"
      :label-col-props="{ span: 8 }"
      :wrapper-col-props="{ span: 16 }"
      label-align="left"
    >
      <a-row :gutter="16">
        <a-col :span="12">
          <a-form-item
            field="groupName"
            :label="$t('group.form.name')"
            :rules="[
              {
                required: true,
                message: $t('group.groupName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.groupName"
              :disabled="isView"
              :placeholder="$t('group.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="groupCode"
            :label="$t('group.form.code')"
            :rules="[
              {
                required: true,
                message: $t('group.groupCode.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.groupCode"
              :disabled="isEdit || isView"
              :placeholder="$t('group.form.code.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="groupStatus"
            :label="$t('group.form.status')"
            :rules="[
              {
                required: true,
                message: $t('group.groupStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="modalModel.groupStatus"
              :options="statusOptions"
              :disabled="isView"
              :placeholder="$t('group.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="groupUserModalVisible"
    unmount-on-close
    :title="$t('group.user.modal.title')"
    :modal-style="{ width: '800px' }"
    :footer="false"
  >
    <a-card>
      <a-table
        row-key="userId"
        :data="groupUserData"
        :bordered="false"
        :loading="groupUserLoading"
        :pagination="{ ...groupUserPagination, showTotal: true }"
        @page-change="groupUserPageChange"
      >
        <template #columns>
          <a-table-column
            :title="$t('organization.columns.organizationName')"
            data-index="organizationName"
          />
          <a-table-column
            :title="$t('organization.columns.organizationCode')"
            data-index="organizationCode"
          />
          <a-table-column
            :title="$t('user.columns.userName')"
            data-index="userName"
          />
          <a-table-column
            :title="$t('user.columns.userCode')"
            data-index="userCode"
          />
        </template>
      </a-table>
    </a-card>
  </a-modal>
</template>

<script lang="ts">
import { defineComponent, computed, ref, reactive } from 'vue';
import { useI18n } from 'vue-i18n';
import useLoading from '@/hooks/loading';
import {
  Pagination,
  Options,
  PaginationResult,
  PaginationQuery,
  SystemInfo,
} from '@/types/global';
import { hasControlPoint, remoteResourceCall } from '@/utils/permission-utils';
import {
  generateFormModel,
  GroupRecord,
  GroupUserRecord,
} from '@/views/system-management/relation-permission/components/group/model';
import permission from '@/views/system-management/relation-permission/components/group/permission';
import { paramWrapper, statusGetter } from '@/utils/global';
import { useUserStore } from '@/store';
import { RoleRecord } from '@/views/system-management/relation-permission/components/role/model';

export default defineComponent({
  setup() {
    const { controlPoints, resources } = permission;
    const listVisible = hasControlPoint(controlPoints['group.list']);
    const addVisible = hasControlPoint(controlPoints['group.add']);
    const editVisible = hasControlPoint(controlPoints['group.edit']);
    const viewVisible = hasControlPoint(controlPoints['group.view']);
    const deleteVisible = hasControlPoint(controlPoints['group.delete']);
    const enableVisible = hasControlPoint(controlPoints['group.enable']);
    const disableVisible = hasControlPoint(controlPoints['group.disable']);
    const groupUserVisible = hasControlPoint(controlPoints['group.user']);
    const modalVisible = ref<boolean>(false);
    const groupUserModalVisible = ref<boolean>(false);
    const groupUserLoading = ref<boolean>(false);
    const groupUserData = ref<GroupRecord[]>([]);
    const selectedGroupRecord = ref<GroupRecord>({});
    const isAdd = ref<boolean>(false);
    const isEdit = ref<boolean>(false);
    const isView = ref<boolean>(false);
    const handleLoading = ref<boolean>(false);
    const userStore = useUserStore();
    const systemInfo = userStore.useSystemInfo() as SystemInfo;
    const { loading, setLoading } = useLoading(true);
    const { t } = useI18n();
    const renderData = ref<GroupRecord[]>([]);
    const formModel = ref(generateFormModel());
    const modalModel = ref(generateFormModel());
    const modalFormRef = ref();
    const basePagination: Pagination = {
      current: 1,
      pageSize: 10,
    };
    const pagination = reactive({
      ...basePagination,
    });
    const groupUserPagination = reactive({
      ...basePagination,
    });
    const statusOptions = computed<Options[]>(statusGetter);
    const fetchData = async (params: PaginationQuery<GroupRecord>) => {
      setLoading(true);
      try {
        const { data } = await remoteResourceCall<
          PaginationQuery<GroupRecord>,
          PaginationResult<GroupRecord>
        >(
          controlPoints['group.list'],
          resources['group.resources.list'],
          params
        );
        renderData.value = data.dataList;
        pagination.current = data.currentPage;
        pagination.pageSize = data.pageSize;
        pagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        setLoading(false);
      }
    };
    const search = () => {
      fetchData({
        condition: {
          ...(paramWrapper(formModel.value) as GroupRecord),
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
    };
    const onPageChange = (currentPage: number) => {
      fetchData({
        ...basePagination,
        condition: {
          ...(paramWrapper(formModel.value) as RoleRecord),
          systemId: systemInfo.systemId,
        },
        currentPage,
      });
    };
    const onPageSizeChange = (pageSize: number) => {
      basePagination.current = 1;
      basePagination.pageSize = pageSize;
      fetchData({
        ...basePagination,
        condition: {
          ...paramWrapper(formModel.value),
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
      });
    };
    if (listVisible) {
      search();
    }
    const reset = () => {
      formModel.value = generateFormModel();
    };
    const handleModalCancel = () => {
      modalVisible.value = false;
    };
    const handleModalConfirm = async () => {
      const { groupCode, groupName, groupStatus } = modalModel.value;
      if (!groupName) {
        modalFormRef.value.setFields({
          groupName: {
            status: 'error',
            message: t('group.groupName.empty.warning'),
          },
        });
      } else if (!groupCode) {
        modalFormRef.value.setFields({
          groupCode: {
            status: 'error',
            message: t('group.groupCode.empty.warning'),
          },
        });
      } else if (!groupStatus) {
        modalFormRef.value.setFields({
          groupStatus: {
            status: 'error',
            message: t('group.groupStatus.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isAdd.value) {
          controlPoint = controlPoints['group.add'];
          resource = resources['group.resources.save'];
        } else {
          controlPoint = controlPoints['group.edit'];
          resource = resources['group.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<GroupRecord, void>(controlPoint, resource, {
            ...modalModel.value,
            systemId: systemInfo.systemId,
          });
          modalVisible.value = false;
          search();
        } catch (err) {
          console.log(err);
        } finally {
          handleLoading.value = false;
        }
      }
    };
    const handleDeleteOk = async (record: GroupRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<GroupRecord, void>(
          controlPoints['group.delete'],
          resources['group.resources.delete'],
          {
            groupId: record.groupId,
            systemId: systemInfo.systemId,
          }
        );
        search();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleDisableOk = async (record: GroupRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<GroupRecord, void>(
          controlPoints['group.disable'],
          resources['group.resources.disable'],
          {
            groupId: record.groupId,
            systemId: systemInfo.systemId,
          }
        );
        search();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleEnableOk = async (record: GroupRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<GroupRecord, void>(
          controlPoints['group.enable'],
          resources['group.resources.enable'],
          {
            groupId: record.groupId,
            systemId: systemInfo.systemId,
          }
        );
        search();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleAddModal = () => {
      modalVisible.value = true;
      isAdd.value = true;
      isEdit.value = false;
      isView.value = false;
      modalModel.value = {
        ...generateFormModel(),
        lastModifiedUserId: userStore.userCode,
      };
    };
    const handleEditModal = (record: GroupRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = true;
      isView.value = false;
      modalModel.value = {
        ...record,
        lastModifiedUserId: userStore.userCode,
      };
    };
    const handleViewModal = (record: GroupRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = false;
      isView.value = true;
      modalModel.value = {
        ...record,
      };
    };
    const fetchGroupUser = async (params: PaginationQuery<GroupRecord>) => {
      try {
        groupUserLoading.value = true;
        const { data } = await remoteResourceCall<
          PaginationQuery<GroupRecord>,
          PaginationResult<GroupUserRecord>
        >(
          controlPoints['group.user'],
          resources['group.resources.groupUser'],
          params
        );
        groupUserData.value = data.dataList;
        groupUserPagination.current = data.currentPage;
        groupUserPagination.pageSize = data.pageSize;
        groupUserPagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        groupUserLoading.value = false;
      }
    };
    const handleGroupUserModal = async (record: GroupRecord) => {
      selectedGroupRecord.value = record;
      fetchGroupUser({
        condition: {
          groupId: selectedGroupRecord.value.groupId,
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
      groupUserModalVisible.value = true;
    };
    const groupUserPageChange = (currentPage: number) => {
      fetchGroupUser({
        ...basePagination,
        condition: {
          groupId: selectedGroupRecord.value.groupId,
          systemId: systemInfo.systemId,
        },
        currentPage,
      });
    };
    return {
      addVisible,
      listVisible,
      editVisible,
      deleteVisible,
      viewVisible,
      disableVisible,
      enableVisible,
      groupUserVisible,
      modalVisible,
      handleModalCancel,
      handleModalConfirm,
      handleDeleteOk,
      handleDisableOk,
      handleEnableOk,
      handleAddModal,
      handleEditModal,
      handleViewModal,
      modalFormRef,
      isAdd,
      isEdit,
      isView,
      handleLoading,
      loading,
      search,
      onPageChange,
      onPageSizeChange,
      renderData,
      pagination,
      formModel,
      modalModel,
      reset,
      statusOptions,
      groupUserModalVisible,
      groupUserLoading,
      groupUserData,
      handleGroupUserModal,
      groupUserPageChange,
      groupUserPagination,
    };
  },
});
</script>

<style scoped lang="less">
:deep(.arco-table-th) {
  &:last-child {
    .arco-table-th-item-title {
      margin-left: 16px;
    }
  }
}
</style>
