<template>
  <a-card class="general-card" :title="t('user.form.query.name')">
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
              <a-form-item field="name" :label="t('user.form.name')">
                <a-input
                  v-model="formModel.userName"
                  :placeholder="t('user.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="name" :label="t('user.form.code')">
                <a-input
                  v-model="formModel.userCode"
                  :placeholder="t('user.form.code.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" :label="t('user.form.status')">
                <a-select
                  v-model="formModel.userStatus"
                  :options="statusOptions"
                  :placeholder="t('user.form.selectDefault')"
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
            {{ t('user.columns.operations.query') }}
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh/>
            </template>
            {{ t('user.columns.operations.reset') }}
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
            {{ t('user.columns.operations.add') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-table
      row-key="userId"
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
          :title="t('user.columns.userName')"
          data-index="userName"
        />
        <a-table-column
          :title="t('user.columns.userCode')"
          data-index="userCode"
        />
        <a-table-column
          :title="t('user.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="t('user.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="t('user.columns.userStatus')"
          data-index="userStatus"
        >
          <template #cell="{ record }">
            <span v-if="record.userStatus === '0'" class="circle"></span>
            <span v-else class="circle pass"></span>
            {{
              t(
                `user.form.status.${
                  record.userStatus === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="t('user.columns.operations')"
          data-index="operations"
        >
          <template #cell="{ record }">
            <a-button
              type="text"
              size="small"
              @click="handleEditModal(record)"
            >
              {{ t('user.columns.operations.edit') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleViewModal(record)"
            >
              {{ t('user.columns.operations.view') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleUserRoleGroupModal(record)"
            >
              {{ t('user.columns.operations.userRoleGroup') }}
            </a-button>
            <a-popconfirm
              :content="t('user.delete.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDeleteOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('user.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.userStatus === '0'"
              :content="t('user.enable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleEnableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('user.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="t('user.disable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDisableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('user.columns.operations.disable') }}
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
    <template v-if="isAdd" #title> {{ t('user.add.modal.title') }}</template>
    <template v-else-if="isEdit" #title>
      {{ t('user.edit.modal.title') }}
    </template
    >
    <template v-else #title> {{ t('user.view.modal.title') }}</template>
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
            field="userName"
            :label="t('user.form.name')"
            :rules="[
              {
                required: true,
                message: t('user.userName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.userName"
              :disabled="isView"
              :placeholder="t('user.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="userCode"
            :label="t('user.form.code')"
            :rules="[
              {
                required: true,
                message: t('user.userCode.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.userCode"
              :disabled="isView"
              :placeholder="t('user.form.code.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="userStatus"
            :label="t('user.form.status')"
            :rules="[
              {
                required: true,
                message: t('user.userStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="modalModel.userStatus"
              :options="statusOptions"
              :disabled="isView"
              :placeholder="t('user.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="organizationIds"
            :label="t('user.form.organizations')"
            :rules="[
              {
                required: true,
                message: t('user.organizationIds.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-tree-select
              v-model="modalModel.organizationIds"
              :disabled="isView"
              :multiple="true"
              :allow-search="true"
              :max-tag-count="2"
              :data="organizationTreeData"
              :field-names="{
                key: 'organizationId',
                title: 'organizationName',
                children: 'children',
                icon: 'icon',
              }"
              :placeholder="t('user.form.organizations.placeholder')"
            ></a-tree-select>
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="userRoleGroupModalVisible"
    unmount-on-close
    :modal-style="{ width: '800px' }"
    :title="t('user.role.group.modal.title')"
    :footer="false"
  >
    <a-tabs
      v-model:active-key="userRelationSaveActiveKey"
      @change="userRelationSaveTabChange"
    >
      <template #extra>
        <a-button
          type="primary"
          size="small"
          :loading="userRelationSaveLoading"
          @click="userRelationSave"
        >{{ t('global.button.save') }}
        </a-button
        >
      </template>
      <a-tab-pane key="userRoleList">
        <template #title>
          <icon-user-group/>
          {{ t('user.tabs.roleList') }}
        </template>
        <a-table
          row-key="orgRoleRelId"
          :pagination="false"
          :data="userRoleData"
          :bordered="false"
          :row-selection="{
            type: 'checkbox',
            showCheckedAll: true,
            selectedRowKeys: selectedOrgRoleRowKeys,
          }"
          :loading="organizationRoleLoading"
          @selection-change="selectedOrgRoleChange"
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
            <a-table-column
              :title="t('role.columns.roleName')"
              data-index="roleName"
            />
            <a-table-column
              :title="t('role.columns.roleCode')"
              data-index="roleCode"
            />
          </template>
        </a-table>
      </a-tab-pane>
      <a-tab-pane key="userGroupList">
        <template #title
        >
          <icon-user-group/>
          {{ t('user.tabs.groupList') }}
        </template>
        <a-table
          row-key="orgGroupRelId"
          :pagination="false"
          :data="userGroupData"
          :bordered="false"
          :row-selection="{
            type: 'checkbox',
            showCheckedAll: true,
            selectedRowKeys: selectedOrgGroupRowKeys,
          }"
          :loading="organizationGroupLoading"
          @selection-change="selectedOrgGroupChange"
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
            <a-table-column
              :title="t('group.columns.groupName')"
              data-index="groupName"
            />
            <a-table-column
              :title="t('group.columns.groupCode')"
              data-index="groupCode"
            />
          </template>
        </a-table>
      </a-tab-pane>
    </a-tabs>
  </a-modal>
</template>

<script lang="ts">
import {computed, defineComponent, reactive, ref} from 'vue';
import {Options, Pagination, PaginationQuery, PaginationResult,} from '../../../../../types';
import {paramWrapper, remoteResourceCall, statusGetter} from '../../../../../utils';
import {
  generateFormModel,
  UserGroupPair,
  UserGroupSave,
  UserOrganizationGroupRecord,
  UserOrganizationRoleRecord,
  UserRecord,
  UserRolePair,
  UserRoleSave,
} from './model';
import permission from './permission';

import {RoleRecord} from '../role/model';
import {Message} from '@arco-design/web-vue';
import {OrganizationRecord} from '../organization/model';
import {organizationInfo, systemInfo, userInfo} from '../../../../../constants';

export default defineComponent({
  setup() {
    const {controlPoints, resources} = permission;
    const userRoleGroupModalVisible = ref<boolean>(false);
    const modalVisible = ref<boolean>(false);
    const isAdd = ref<boolean>(false);
    const isEdit = ref<boolean>(false);
    const isView = ref<boolean>(false);
    const handleLoading = ref<boolean>(false);
    const loading = ref<boolean>(true)
    const renderData = ref<UserRecord[]>([]);
    const userRoleData = ref<UserOrganizationRoleRecord[]>([]);
    const organizationTreeData = ref<OrganizationRecord[]>([]);
    const selectedOrgRoleRowKeys = ref<string[]>([]);
    const selectedOrgGroupRowKeys = ref<string[]>([]);
    const organizationRoleLoading = ref<boolean>(false);
    const organizationGroupLoading = ref<boolean>(false);
    const userGroupData = ref<UserOrganizationGroupRecord[]>([]);
    const userRelationSaveActiveKey = ref<string>('userRoleList');
    const userRelationSaveLoading = ref<boolean>(false);
    const selectedUserRecord = ref<UserRecord>({});
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
    const statusOptions = computed<Options[]>(statusGetter);
    const fetchData = async (params: PaginationQuery<UserRecord>) => {
      loading.value = true;
      try {
        const {data} = await remoteResourceCall<
          PaginationQuery<UserRecord>,
          PaginationResult<UserRecord>
        >(controlPoints['user.list'], resources['user.resources.list'], params);
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
          ...(paramWrapper(formModel.value) as UserRecord),
          systemId: systemInfo.systemId,
          organizationId: organizationInfo.organizationId
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
          organizationId: organizationInfo.organizationId
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
          organizationId: organizationInfo.organizationId
        },
        currentPage: basePagination.current,
      });
    };
    const fetchOrganizationData = async () => {
      loading.value = true;
      try {
        const {data} = await remoteResourceCall<
          OrganizationRecord,
          Array<OrganizationRecord>
        >(
          controlPoints['user.list'],
          resources['user.resources.organizationList'],
          {
            upOrganizationId: organizationInfo.organizationId,
            systemId: systemInfo.systemId,
          }
        );
        organizationTreeData.value = data;
      } catch (err) {
        console.log(err);
      } finally {
        loading.value = false;
      }
    };
    search();
    fetchOrganizationData();
    const reset = () => {
      formModel.value = generateFormModel();
    };
    const handleModalCancel = () => {
      modalVisible.value = false;
    };
    const handleModalConfirm = async () => {
      const {userCode, userName, userStatus, organizationIds} = modalModel.value;
      if (!userName) {
        modalFormRef.value.setFields({
          userName: {
            status: 'error',
            message: t('user.userName.empty.warning'),
          },
        });
      } else if (!userCode) {
        modalFormRef.value.setFields({
          userCode: {
            status: 'error',
            message: t('user.userCode.empty.warning'),
          },
        });
      } else if (!userStatus) {
        modalFormRef.value.setFields({
          userStatus: {
            status: 'error',
            message: t('user.userStatus.empty.warning'),
          },
        });
      } else if (!organizationIds || organizationIds.length === 0) {
        modalFormRef.value.setFields({
          organizationIds: {
            status: 'error',
            message: t('user.organizationIds.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isAdd.value) {
          controlPoint = controlPoints['user.add'];
          resource = resources['user.resources.save'];
        } else {
          controlPoint = controlPoints['user.edit'];
          resource = resources['user.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<UserRecord, void>(controlPoint, resource, {
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
    const handleDeleteOk = async (record: UserRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<UserRecord, void>(
          controlPoints['user.delete'],
          resources['user.resources.delete'],
          {
            userId: record.userId,
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
    const handleDisableOk = async (record: UserRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<UserRecord, void>(
          controlPoints['user.disable'],
          resources['user.resources.disable'],
          {
            userId: record.userId,
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
    const handleEnableOk = async (record: UserRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<UserRecord, void>(
          controlPoints['user.enable'],
          resources['user.resources.enable'],
          {
            userId: record.userId,
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
    const handleEditModal = (record: UserRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = true;
      isView.value = false;
      modalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleViewModal = (record: UserRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = false;
      isView.value = true;
      modalModel.value = {
        ...record,
      };
    };
    const fetchUserRoleData = async (params: UserRecord) => {
      try {
        organizationRoleLoading.value = true;
        const {data} = await remoteResourceCall<UserRecord, UserRolePair>(
          controlPoints['user.userRoleGroup'],
          resources['user.resources.userRoleList'],
          params
        );
        userRoleData.value = data.userOrganizationRoleList;
        selectedOrgRoleRowKeys.value = data.authorizedOrganizationRoleRelIds;
      } catch (err) {
        console.log(err);
      } finally {
        organizationRoleLoading.value = false;
      }
    };
    const fetchUserGroupData = async (params: UserRecord) => {
      try {
        organizationGroupLoading.value = true;
        const {data} = await remoteResourceCall<UserRecord, UserGroupPair>(
          controlPoints['user.userRoleGroup'],
          resources['user.resources.userGroupList'],
          params
        );
        userGroupData.value = data.userOrganizationGroupList;
        selectedOrgGroupRowKeys.value = data.authorizedOrganizationGroupRelIds;
      } catch (err) {
        console.log(err);
      } finally {
        organizationGroupLoading.value = false;
      }
    };
    const handleUserRoleGroupModal = async (record: UserRecord) => {
      userRoleGroupModalVisible.value = true;
      selectedUserRecord.value = record;
      await fetchUserRoleData({
        userId: selectedUserRecord.value.userId,
        systemId: systemInfo.systemId,
        organizationId: organizationInfo.organizationId
      });
      await fetchUserGroupData({
        userId: selectedUserRecord.value.userId,
        systemId: systemInfo.systemId,
        organizationId: organizationInfo.organizationId
      });
    };
    const userRelationSave = async () => {
      if (userRelationSaveActiveKey.value === 'userRoleList') {
        try {
          userRelationSaveLoading.value = true;
          userRoleGroupModalVisible.value = true;
          await remoteResourceCall<UserRoleSave, void>(
            controlPoints['user.userRoleGroup'],
            resources['user.resources.userRoleSave'],
            {
              userId: selectedUserRecord.value.userId,
              systemId: systemInfo.systemId,
              lastModifiedUserId: userInfo.userCode,
              authorizedOrgRoleRelIds: selectedOrgRoleRowKeys.value,
            }
          );
          Message.success(t('global.message.save.success'));
        } catch (err) {
          console.log(err);
        } finally {
          userRelationSaveLoading.value = false;
        }
      } else {
        try {
          userRelationSaveLoading.value = true;
          userRoleGroupModalVisible.value = true;
          await remoteResourceCall<UserGroupSave, void>(
            controlPoints['user.userRoleGroup'],
            resources['user.resources.userGroupSave'],
            {
              userId: selectedUserRecord.value.userId,
              systemId: systemInfo.systemId,
              lastModifiedUserId: userInfo.userCode,
              authorizedOrgGroupRelIds: selectedOrgGroupRowKeys.value,
            }
          );
          Message.success(t('global.message.save.success'));
        } catch (err) {
          console.log(err);
        } finally {
          userRelationSaveLoading.value = false;
        }
      }
    };
    const userRelationSaveTabChange = (key: string) => {
      if (key === 'userRoleList') {
        fetchUserRoleData({
          userId: selectedUserRecord.value.userId,
          systemId: systemInfo.systemId,
          organizationId: organizationInfo.organizationId
        });
      } else {
        fetchUserGroupData({
          userId: selectedUserRecord.value.userId,
          systemId: systemInfo.systemId,
          organizationId: organizationInfo.organizationId
        });
      }
    };
    const selectedOrgRoleChange = (keys: string[]) => {
      selectedOrgRoleRowKeys.value = keys;
    };
    const selectedOrgGroupChange = (keys: string[]) => {
      selectedOrgGroupRowKeys.value = keys;
    };
    const t = (key: string) => {
      const map = {
        'global.button.confirm': '确定',
        'global.button.save': '保存',
        'global.message.save.success': '保存成功！',
        'global.button.cancel': '取消',
        'global.form.status.active': '生效',
        'global.form.status.inactive': '失效',
        'global.form.options.yes': '是',
        'global.form.options.no': '否',
        'global.method.options.post': 'POST',
        'global.method.options.get': 'GET',
        'global.authorization.options.currentOrganization': '本机构',
        'global.authorization.options.currentSubOrganization': '本机构及下级机构',
        'global.authorization.options.selfCurrentOrganization': '本人及本机构',
        'global.clipRule.options.name': '客户姓名',
        'global.clipRule.options.bankCard': '银行卡号',
        'global.clipRule.options.idCard': '身份证件',
        'global.clipRule.options.email': '电子邮件',
        'global.clipRule.options.phone': '电话手机',
        'global.clipRule.options.other': '其他',
        'menu.authority-management.user': '用户管理',
        'user.form.query.name': '用户查询',
        'user.form.code': '用户代码',
        'user.form.code.placeholder': '请输入用户代码',
        'user.form.name': '用户名称',
        'user.form.name.placeholder': '请输入用户名称',
        'user.form.status': '用户状态',
        'user.form.organizations': '机构信息',
        'user.form.status.active': '生效',
        'user.form.status.inactive': '失效',
        'user.form.selectDefault': '全部',
        'user.form.organizations.placeholder': '请选择机构信息',
        'user.columns.userCode': '用户代码',
        'user.columns.userName': '用户名称',
        'user.columns.userStatus': '用户状态',
        'user.columns.updateTime': '最新变更时间',
        'user.columns.lastModifiedUserId': '最新变更用户',
        'user.columns.operations': '操作',
        'user.columns.operations.query': '查询',
        'user.columns.operations.reset': '重置',
        'user.columns.operations.add': '新增',
        'user.columns.operations.edit': '编辑',
        'user.columns.operations.view': '查看',
        'user.columns.operations.delete': '删除',
        'user.columns.operations.disable': '禁用',
        'user.columns.operations.enable': '启用',
        'user.columns.operations.userRoleGroup': '角色岗位',
        'user.role.group.modal.title': '角色岗位列表',
        'user.tabs.roleList': '用户角色列表',
        'user.tabs.groupList': '用户岗位列表',
        'user.add.modal.title': '新增用户',
        'user.edit.modal.title': '编辑用户',
        'user.view.modal.title': '查看用户',
        'user.userName.empty.warning': '用户名称为空.',
        'user.userCode.empty.warning': '用户代码为空.',
        'user.userStatus.empty.warning': '用户状态未选择.',
        'user.organizationIds.empty.warning': '用户机构未选择.',
        'user.delete.alert.message': '您确定要删除该用户吗?',
        'user.enable.alert.message': '您确定要启用该用户吗?',
        'user.disable.alert.message': '您确定要禁用该用户吗?',
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
      userRoleGroupModalVisible,
      handleUserRoleGroupModal,
      userRoleData,
      selectedOrgRoleRowKeys,
      organizationRoleLoading,
      userRelationSaveActiveKey,
      userRelationSaveLoading,
      userRelationSave,
      userRelationSaveTabChange,
      selectedOrgRoleChange,
      organizationGroupLoading,
      userGroupData,
      selectedOrgGroupRowKeys,
      selectedOrgGroupChange,
      organizationTreeData
    };
  },
});
</script>
