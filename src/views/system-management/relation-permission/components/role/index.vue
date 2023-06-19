<template>
  <a-card class="general-card" :title="t('role.form.query.name')">
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
              <a-form-item field="name" :label="t('role.form.name')">
                <a-input
                  v-model="formModel.roleName"
                  :placeholder="t('role.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="name" :label="t('role.form.code')">
                <a-input
                  v-model="formModel.roleCode"
                  :placeholder="t('role.form.code.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" :label="t('role.form.status')">
                <a-select
                  v-model="formModel.role_status"
                  :options="statusOptions"
                  :placeholder="t('role.form.selectDefault')"
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
            {{ t('role.columns.operations.query') }}
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh />
            </template>
            {{ t('role.columns.operations.reset') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-divider style="margin-top: 0" />
    <a-row style="margin-bottom: 16px">
      <a-col :span="16">
        <a-space>
          <a-button type="primary" @click="handleAddModal">
            <template #icon>
              <icon-plus />
            </template>
            {{ t('role.columns.operations.add') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-table
      row-key="roleId"
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
          :title="t('role.columns.roleName')"
          data-index="roleName"
        />
        <a-table-column
          :title="t('role.columns.roleCode')"
          data-index="roleCode"
        />
        <a-table-column
          :title="t('role.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="t('role.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="t('role.columns.role_status')"
          data-index="role_status"
        >
          <template #cell="{ record }">
            <span v-if="record.role_status === '0'" class="circle"></span>
            <span v-else class="circle pass"></span>
            {{
              t(
                `role.form.status.${
                  record.role_status === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="t('role.columns.operations')"
          data-index="operations"
        >
          <template #cell="{ record }">
            <a-button
              type="text"
              size="small"
              @click="handleEditModal(record)"
            >
              {{ t('role.columns.operations.edit') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleViewModal(record)"
            >
              {{ t('role.columns.operations.view') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleRoleUserModal(record)"
            >
              {{ t('role.columns.operations.roleUser') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="
                $emit('authorize', {
                  authorizationType: 'R',
                  authorizationKey: record.roleId,
                })
              "
            >
              {{ t('role.columns.operations.roleAuthorize') }}
            </a-button>
            <a-popconfirm
              :content="t('role.delete.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDeleteOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('role.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.role_status === '0'"
              :content="t('role.enable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleEnableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('role.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="t('role.disable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDisableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('role.columns.operations.disable') }}
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
    <template v-if="isAdd" #title> {{ t('role.add.modal.title') }}</template>
    <template v-else-if="isEdit" #title>
      {{ t('role.edit.modal.title') }}
    </template>
    <template v-else #title> {{ t('role.view.modal.title') }}</template>
    <template v-if="isAdd || isEdit" #footer>
      <a-button @click="handleModalCancel"
        >{{ t('global.button.cancel') }}
      </a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleModalConfirm"
        >{{ t('global.button.confirm') }}
      </a-button>
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
            field="roleName"
            :label="t('role.form.name')"
            :rules="[
              {
                required: true,
                message: t('role.roleName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.roleName"
              :disabled="isView"
              :placeholder="t('role.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="roleCode"
            :label="t('role.form.code')"
            :rules="[
              {
                required: true,
                message: t('role.roleCode.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.roleCode"
              :disabled="isEdit || isView"
              :placeholder="t('role.form.code.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="role_status"
            :label="t('role.form.status')"
            :rules="[
              {
                required: true,
                message: t('role.role_status.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="modalModel.role_status"
              :options="statusOptions"
              :disabled="isView"
              :placeholder="t('role.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="roleUserModalVisible"
    unmount-on-close
    :title="t('role.user.modal.title')"
    :modal-style="{ width: '800px' }"
    :footer="false"
  >
    <a-card>
      <a-table
        row-key="userId"
        :data="roleUserData"
        :bordered="false"
        :loading="roleUserLoading"
        :pagination="{ ...roleUserPagination, showTotal: true }"
        @page-change="roleUserPageChange"
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
import { defineComponent, computed, ref, reactive } from 'vue';
import {
  Pagination,
  Options,
  PaginationResult,
  PaginationQuery,
  Done,
  SystemInfo,
} from '../../../../../types';
import { paramWrapper, statusGetter, remoteResourceCall } from '../../../../../utils';
import {
  generateFormModel,
  RoleRecord,
  RoleUserRecord,
} from './model';
import {systemInfo, userInfo} from "../../../../../constants";
import permission from './permission';

export default defineComponent({
  emits: ['authorize'],
  setup() {
    const { controlPoints, resources } = permission;
    const modalVisible = ref<boolean>(false);
    const roleUserModalVisible = ref<boolean>(false);
    const roleUserLoading = ref<boolean>(false);
    const roleUserData = ref<RoleRecord[]>([]);
    const selectedRoleRecord = ref<RoleRecord>({});
    const isAdd = ref<boolean>(false);
    const isEdit = ref<boolean>(false);
    const isView = ref<boolean>(false);
    const handleLoading = ref<boolean>(false);
    const loading = ref<boolean>(true)
    const renderData = ref<RoleRecord[]>([]);
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
    const roleUserPagination = reactive({
      ...basePagination,
    });
    const statusOptions = computed<Options[]>(statusGetter);
    const fetchData = async (params: PaginationQuery<RoleRecord>) => {
      loading.value = true
      try {
        const { data } = await remoteResourceCall<
          PaginationQuery<RoleRecord>,
          PaginationResult<RoleRecord>
        >(controlPoints['role.list'], resources['role.resources.list'], params);
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
          ...(paramWrapper(formModel.value) as RoleRecord),
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
      const { roleCode, roleName, role_status } = modalModel.value;
      if (!roleName) {
        modalFormRef.value.setFields({
          roleName: {
            status: 'error',
            message: t('role.roleName.empty.warning'),
          },
        });
      } else if (!roleCode) {
        modalFormRef.value.setFields({
          roleCode: {
            status: 'error',
            message: t('role.roleCode.empty.warning'),
          },
        });
      } else if (!role_status) {
        modalFormRef.value.setFields({
          role_status: {
            status: 'error',
            message: t('role.role_status.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isAdd.value) {
          controlPoint = controlPoints['role.add'];
          resource = resources['role.resources.save'];
        } else {
          controlPoint = controlPoints['role.edit'];
          resource = resources['role.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<RoleRecord, void>(controlPoint, resource, {
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
    const handleDeleteOk = async (record: RoleRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<RoleRecord, void>(
          controlPoints['role.delete'],
          resources['role.resources.delete'],
          {
            roleId: record.roleId,
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
    const handleDisableOk = async (record: RoleRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<RoleRecord, void>(
          controlPoints['role.disable'],
          resources['role.resources.disable'],
          {
            roleId: record.roleId,
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
    const handleEnableOk = async (record: RoleRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<RoleRecord, void>(
          controlPoints['role.enable'],
          resources['role.resources.enable'],
          {
            roleId: record.roleId,
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
    const handleEditModal = (record: RoleRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = true;
      isView.value = false;
      modalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleViewModal = (record: RoleRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = false;
      isView.value = true;
      modalModel.value = {
        ...record,
      };
    };
    const fetchRoleUser = async (params: PaginationQuery<RoleRecord>) => {
      try {
        roleUserLoading.value = true;
        const { data } = await remoteResourceCall<
          PaginationQuery<RoleRecord>,
          PaginationResult<RoleUserRecord>
        >(
          controlPoints['role.user'],
          resources['role.resources.roleUser'],
          params
        );
        roleUserData.value = data.dataList;
        roleUserPagination.current = data.currentPage;
        roleUserPagination.pageSize = data.pageSize;
        roleUserPagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        roleUserLoading.value = false;
      }
    };
    const handleRoleUserModal = async (record: RoleRecord) => {
      selectedRoleRecord.value = record;
      fetchRoleUser({
        condition: {
          roleId: selectedRoleRecord.value.roleId,
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
      roleUserModalVisible.value = true;
    };
    const roleUserPageChange = (currentPage: number) => {
      fetchRoleUser({
        ...basePagination,
        condition: {
          roleId: selectedRoleRecord.value.roleId,
          systemId: systemInfo.systemId,
        },
        currentPage,
      });
    };
    const t = (key:string)=>{
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
        'menu.authority-management.role': '角色管理',
        'role.form.query.name': '角色查询',
        'role.form.code': '角色代码',
        'role.form.code.placeholder': '请输入角色代码',
        'role.form.name': '角色名称',
        'role.form.name.placeholder': '请输入角色名称',
        'role.form.status': '角色状态',
        'role.form.status.active': '生效',
        'role.form.status.inactive': '失效',
        'role.form.selectDefault': '全部',
        'role.columns.roleCode': '角色代码',
        'role.columns.roleName': '角色名称',
        'role.columns.role_status': '角色状态',
        'role.columns.updateTime': '最新变更时间',
        'role.columns.lastModifiedUserId': '最新变更用户',
        'role.columns.operations': '操作',
        'role.columns.operations.query': '查询',
        'role.columns.operations.reset': '重置',
        'role.columns.operations.add': '新增',
        'role.columns.operations.edit': '编辑',
        'role.columns.operations.view': '查看',
        'role.columns.operations.delete': '删除',
        'role.columns.operations.disable': '禁用',
        'role.columns.operations.enable': '启用',
        'role.columns.operations.roleUser': '角色用户',
        'role.columns.operations.roleMutex': '角色互斥',
        'role.columns.operations.roleAuthorize': '角色授权',
        'role.user.modal.title': '角色用户',
        'role.add.modal.title': '新增角色',
        'role.edit.modal.title': '编辑角色',
        'role.view.modal.title': '查看角色',
        'role.roleName.empty.warning': '角色名称为空.',
        'role.roleCode.empty.warning': '角色代码为空.',
        'role.role_status.empty.warning': '角色状态未选择.',
        'role.delete.alert.message': '您确定要删除该角色吗?',
        'role.enable.alert.message': '您确定要启用该角色吗?',
        'role.disable.alert.message': '您确定要禁用该角色吗?',
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
      roleUserModalVisible,
      roleUserLoading,
      roleUserData,
      handleRoleUserModal,
      roleUserPageChange,
      roleUserPagination,
    };
  },
});
</script>
