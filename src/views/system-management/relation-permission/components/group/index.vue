<template>
  <a-card class="general-card" :title="t('group.form.query.name')">
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
              <a-form-item field="name" :label="t('group.form.name')">
                <a-input
                  v-model="formModel.groupName"
                  :placeholder="t('group.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="name" :label="t('group.form.code')">
                <a-input
                  v-model="formModel.groupCode"
                  :placeholder="t('group.form.code.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" :label="t('group.form.status')">
                <a-select
                  v-model="formModel.groupStatus"
                  :options="statusOptions"
                  :placeholder="t('group.form.selectDefault')"
                />
              </a-form-item>
            </a-col>
          </a-row>
        </a-form>
      </a-col>
      <a-divider style="height: 84px" direction="vertical"/>
      <a-col :flex="'86px'" style="text-align: right">
        <a-space direction="vertical" :size="18">
          <a-button type="primary" @click="search">
            <template #icon>
              <icon-search/>
            </template>
            {{ t('group.columns.operations.query') }}
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh/>
            </template>
            {{ t('group.columns.operations.reset') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-divider style="margin-top: 0"/>
    <a-row style="margin-bottom: 16px">
      <a-col :span="16">
        <a-space>
          <a-button type="primary" @click="handleAddModal">
            <template #icon>
              <icon-plus/>
            </template>
            {{ t('group.columns.operations.add') }}
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
          :title="t('group.columns.groupName')"
          data-index="groupName"
        />
        <a-table-column
          :title="t('group.columns.groupCode')"
          data-index="groupCode"
        />
        <a-table-column
          :title="t('group.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="t('group.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="t('group.columns.groupStatus')"
          data-index="groupStatus"
        >
          <template #cell="{ record }">
            <span v-if="record.groupStatus === '0'" class="circle"></span>
            <span v-else class="circle pass"></span>
            {{
              t(
                `group.form.status.${
                  record.groupStatus === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="t('group.columns.operations')"
          data-index="operations"
        >
          <template #cell="{ record }">
            <a-button
              type="text"
              size="small"
              @click="handleEditModal(record)"
            >
              {{ t('group.columns.operations.edit') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleViewModal(record)"
            >
              {{ t('group.columns.operations.view') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleGroupUserModal(record)"
            >
              {{ t('group.columns.operations.groupUser') }}
            </a-button>
            <a-popconfirm
              :content="t('group.delete.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDeleteOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('group.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.groupStatus === '0'"
              :content="t('group.enable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleEnableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('group.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="t('group.disable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDisableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('group.columns.operations.disable') }}
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
    <template v-if="isAdd" #title> {{ t('group.add.modal.title') }}</template>
    <template v-else-if="isEdit" #title>
      {{ t('group.edit.modal.title') }}
    </template
    >
    <template v-else #title> {{ t('group.view.modal.title') }}</template>
    <template v-if="isAdd || isEdit" #footer>
      <a-button @click="handleModalCancel">{{
          t('global.button.cancel')
        }}
      </a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleModalConfirm"
      >{{ t('global.button.confirm') }}
      </a-button
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
            :label="t('group.form.name')"
            :rules="[
              {
                required: true,
                message: t('group.groupName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.groupName"
              :disabled="isView"
              :placeholder="t('group.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="groupCode"
            :label="t('group.form.code')"
            :rules="[
              {
                required: true,
                message: t('group.groupCode.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.groupCode"
              :disabled="isEdit || isView"
              :placeholder="t('group.form.code.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="groupStatus"
            :label="t('group.form.status')"
            :rules="[
              {
                required: true,
                message: t('group.groupStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="modalModel.groupStatus"
              :options="statusOptions"
              :disabled="isView"
              :placeholder="t('group.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="groupUserModalVisible"
    unmount-on-close
    :title="t('group.user.modal.title')"
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
            :title="t('organization.columns.organizationName')"
            data-index="organizationName"
          />
          <a-table-column
            :title="t('organization.columns.organizationCode')"
            data-index="organizationCode"
          />
          <a-table-column
            :title="t('user.columns.userName')"
            data-index="userName"
          />
          <a-table-column
            :title="t('user.columns.userCode')"
            data-index="userCode"
          />
        </template>
      </a-table>
    </a-card>
  </a-modal>
</template>

<script lang="ts">
import {computed, defineComponent, reactive, ref} from 'vue';
import {Options, Pagination, PaginationQuery, PaginationResult,} from '../../../../../types';
import {paramWrapper, remoteResourceCall, statusGetter} from '../../../../../utils';
import {generateFormModel, GroupRecord, GroupUserRecord,} from './model';
import permission from './permission';
import {RoleRecord} from '../role/model';
import {systemInfo, userInfo} from "../../../../../constants";

export default defineComponent({
  setup() {
    const {controlPoints, resources} = permission;
    const modalVisible = ref<boolean>(false);
    const groupUserModalVisible = ref<boolean>(false);
    const groupUserLoading = ref<boolean>(false);
    const groupUserData = ref<GroupRecord[]>([]);
    const selectedGroupRecord = ref<GroupRecord>({});
    const isAdd = ref<boolean>(false);
    const isEdit = ref<boolean>(false);
    const isView = ref<boolean>(false);
    const handleLoading = ref<boolean>(false);
    const loading = ref<boolean>(false)
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
      loading.value = true;
      try {
        const {data} = await remoteResourceCall<
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
        loading.value = false;
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
    search();
    const reset = () => {
      formModel.value = generateFormModel();
    };
    const handleModalCancel = () => {
      modalVisible.value = false;
    };
    const handleModalConfirm = async () => {
      const {groupCode, groupName, groupStatus} = modalModel.value;
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
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleEditModal = (record: GroupRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = true;
      isView.value = false;
      modalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
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
        const {data} = await remoteResourceCall<
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
    const t = (key: string) => {
      const map = {
        'global.button.confirm': '确定',
        'global.button.save': '保存',
        'global.message.save.success': '保存成功！',
        'global.button.cancel': '取消',
        'global.form.status.active': '生效',
        'global.form.status.inactive': '失效',
        'global.form.options.yes':'是',
        'global.form.options.no':'否',
        'global.method.options.post':'POST',
        'global.method.options.get':'GET',
        'global.authorization.options.currentOrganization': '本机构',
        'global.authorization.options.currentSubOrganization': '本机构及下级机构',
        'global.authorization.options.selfCurrentOrganization': '本人及本机构',
        'global.clipRule.options.name': '客户姓名',
        'global.clipRule.options.bankCard': '银行卡号',
        'global.clipRule.options.idCard': '身份证件',
        'global.clipRule.options.email': '电子邮件',
        'global.clipRule.options.phone': '电话手机',
        'global.clipRule.options.other': '其他',
        'menu.authority-management.group': '岗位管理',
        'group.form.query.name': '岗位查询',
        'group.form.code': '岗位代码',
        'group.form.code.placeholder': '请输入岗位代码',
        'group.form.name': '岗位名称',
        'group.form.name.placeholder': '请输入岗位名称',
        'group.form.status': '岗位状态',
        'group.form.status.active': '生效',
        'group.form.status.inactive': '失效',
        'group.form.selectDefault': '全部',
        'group.columns.groupCode': '岗位代码',
        'group.columns.groupName': '岗位名称',
        'group.columns.groupStatus': '岗位状态',
        'group.columns.updateTime': '最新变更时间',
        'group.columns.lastModifiedUserId': '最新变更用户',
        'group.columns.operations': '操作',
        'group.columns.operations.query': '查询',
        'group.columns.operations.reset': '重置',
        'group.columns.operations.add': '新增',
        'group.columns.operations.edit': '编辑',
        'group.columns.operations.view': '查看',
        'group.columns.operations.delete': '删除',
        'group.columns.operations.disable': '禁用',
        'group.columns.operations.enable': '启用',
        'group.columns.operations.groupUser': '岗位用户',
        'group.columns.operations.groupMutex': '岗位互斥',
        'group.user.modal.title': '岗位用户',
        'group.add.modal.title': '新增岗位',
        'group.edit.modal.title': '编辑岗位',
        'group.view.modal.title': '查看岗位',
        'group.groupName.empty.warning': '岗位名称为空.',
        'group.groupCode.empty.warning': '岗位代码为空.',
        'group.groupStatus.empty.warning': '岗位状态未选择.',
        'group.delete.alert.message': '您确定要删除该岗位吗?',
        'group.enable.alert.message': '您确定要启用该岗位吗?',
        'group.disable.alert.message': '您确定要禁用该岗位吗?',
      }
      return map[key];
    }
    return {
      t,
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

