<template>
  <a-card class="general-card" :title="$t('organization.form.query.name')">
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
              <a-form-item field="name" :label="$t('organization.form.name')">
                <a-input
                  v-model="formModel.organizationName"
                  :placeholder="$t('organization.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="name" :label="$t('organization.form.code')">
                <a-input
                  v-model="formModel.organizationCode"
                  :placeholder="$t('organization.form.code.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item
                field="status"
                :label="$t('organization.form.status')"
              >
                <a-select
                  v-model="formModel.organizationStatus"
                  :options="statusOptions"
                  :placeholder="$t('organization.form.selectDefault')"
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
            {{ $t('organization.columns.operations.query') }}
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh />
            </template>
            {{ $t('organization.columns.operations.reset') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-divider style="margin-top: 0" />
    <a-table
      row-key="organizationId"
      :loading="loading"
      :pagination="false"
      :data="renderData"
      :bordered="false"
      :load-more="loadMore"
    >
      <template #columns>
        <a-table-column
          :title="$t('organization.columns.organizationName')"
          data-index="organizationName"
          fixed="left"
        />
        <a-table-column
          :title="$t('organization.columns.organizationCode')"
          data-index="organizationCode"
        />
        <a-table-column
          :title="$t('organization.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="$t('organization.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="$t('organization.columns.organizationStatus')"
          data-index="organizationStatus"
        >
          <template #cell="{ record }">
            <span
              v-if="record.organizationStatus === '0'"
              class="circle"
            ></span>
            <span v-else class="circle pass"></span>
            {{
              $t(
                `organization.form.status.${
                  record.organizationStatus === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="$t('organization.columns.operations')"
          data-index="operations"
          fixed="right"
        >
          <template #cell="{ record }">
            <a-button
              v-show="addVisible"
              type="text"
              size="small"
              @click="handleAddModal(record)"
            >
              {{ $t('organization.columns.operations.add') }}
            </a-button>
            <a-button
              v-show="editVisible"
              type="text"
              size="small"
              @click="handleEditModal(record)"
            >
              {{ $t('organization.columns.operations.edit') }}
            </a-button>
            <a-button
              v-show="viewVisible"
              type="text"
              size="small"
              @click="handleViewModal(record)"
            >
              {{ $t('organization.columns.operations.view') }}
            </a-button>
            <a-button
              v-show="organizationRoleVisible"
              type="text"
              size="small"
              @click="handleOrganizationRoleModal(record)"
            >
              {{ $t('organization.columns.operations.organizationRoleGroup') }}
            </a-button>
            <a-button
              v-show="organizationUserVisible"
              type="text"
              size="small"
              @click="handleOrganizationUserModal(record)"
            >
              {{ $t('organization.columns.operations.organizationUser') }}
            </a-button>
            <a-popconfirm
              :content="$t('organization.delete.alert.message')"
              :ok-text="$t('global.button.confirm')"
              :cancel-text="$t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDeleteOk(record)"
            >
              <a-button v-show="deleteVisible" type="text" size="small">
                {{ $t('organization.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.organizationStatus === '0'"
              :content="$t('organization.enable.alert.message')"
              :ok-text="$t('global.button.confirm')"
              :cancel-text="$t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleEnableOk(record)"
            >
              <a-button v-show="enableVisible" type="text" size="small">
                {{ $t('organization.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="$t('organization.disable.alert.message')"
              :ok-text="$t('global.button.confirm')"
              :cancel-text="$t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDisableOk(record)"
            >
              <a-button v-show="disableVisible" type="text" size="small">
                {{ $t('organization.columns.operations.disable') }}
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
    <template v-if="isAdd" #title>
      {{ $t('organization.add.modal.title') }}</template
    >
    <template v-else-if="isEdit" #title>
      {{ $t('organization.edit.modal.title') }}</template
    >
    <template v-else #title>
      {{ $t('organization.view.modal.title') }}</template
    >
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
            field="organizationName"
            :label="$t('organization.form.name')"
            :rules="[
              {
                required: true,
                message: $t('organization.organizationName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.organizationName"
              :disabled="isView"
              :placeholder="$t('organization.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="organizationCode"
            :label="$t('organization.form.code')"
            :rules="[
              {
                required: true,
                message: $t('organization.organizationCode.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.organizationCode"
              :disabled="isView"
              :placeholder="$t('organization.form.code.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="organizationStatus"
            :label="$t('organization.form.status')"
            :rules="[
              {
                required: true,
                message: $t('organization.organizationStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="modalModel.organizationStatus"
              :options="statusOptions"
              :disabled="isView"
              :placeholder="$t('organization.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="upOrganizationName"
            :label="$t('organization.form.upOrganizationName')"
            validate-trigger="input"
            required
          >
            <a-input
              v-model="modalModel.upOrganizationName"
              disabled
              :placeholder="
                $t('organization.form.upOrganizationName.placeholder')
              "
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="organizationLevel"
            :label="$t('organization.form.organizationLevel')"
            validate-trigger="input"
            required
          >
            <a-input
              v-model="modalModel.organizationLevel"
              disabled
              :placeholder="
                $t('organization.form.organizationLevel.placeholder')
              "
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="organizationRoleGroupModalVisible"
    unmount-on-close
    :title="$t('organization.roleGroup.modal.title')"
    :modal-style="{ width: '800px' }"
    :footer="false"
  >
    <a-tabs
      v-model:active-key="organizationRelationActiveKey"
      @change="organizationRelationTabChange"
    >
      <template #extra>
        <a-button
          type="primary"
          size="small"
          :loading="organizationRelationOkLoading"
          @click="handleOrganizationRelationSave"
          >{{ $t('global.button.save') }}</a-button
        >
      </template>
      <a-tab-pane key="organizationRoleList">
        <template #title>
          <icon-user-group />
          {{ $t('organization.tabs.roleList') }}
        </template>
        <a-table
          row-key="roleId"
          :pagination="false"
          :data="roleRecordData"
          :row-selection="{
            type: 'checkbox',
            showCheckedAll: true,
            selectedRowKeys: selectedRoleRowKeys,
          }"
          :bordered="false"
          :loading="organizationRoleLoading"
          @selection-change="selectedRoleChange"
        >
          <template #columns>
            <a-table-column
              :title="$t('role.columns.roleName')"
              data-index="roleName"
            />
            <a-table-column
              :title="$t('role.columns.roleCode')"
              data-index="roleCode"
            />
          </template>
        </a-table>
      </a-tab-pane>
      <a-tab-pane key="organizationGroupList">
        <template #title
          ><icon-user-group />
          {{ $t('organization.tabs.groupList') }}
        </template>
        <a-table
          row-key="groupId"
          :pagination="false"
          :data="groupRecordData"
          :row-selection="{
            type: 'checkbox',
            showCheckedAll: true,
            selectedRowKeys: selectedGroupRowKeys,
          }"
          :bordered="false"
          :loading="organizationGroupLoading"
          @selection-change="selectedGroupChange"
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
          </template>
        </a-table>
      </a-tab-pane>
    </a-tabs>
  </a-modal>
  <a-modal
    v-model:visible="organizationUserModalVisible"
    unmount-on-close
    :title="$t('organization.user.modal.title')"
    :modal-style="{ width: '800px' }"
    :footer="false"
  >
    <a-card>
      <a-table
        row-key="userOrgRelId"
        :pagination="false"
        :data="userRecordData"
        :bordered="false"
        :loading="organizationUserLoading"
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
import { paramWrapper, statusGetter } from '@/utils/global';
import { defineComponent, computed, ref, reactive } from 'vue';
import useLoading from '@/hooks/loading';
import { Pagination, Options, Done, SystemInfo } from '@/types/global';
import { hasControlPoint, remoteResourceCall } from '@/utils/permission-utils';
import {
  generateFormModel,
  OrganizationGroupPair,
  OrganizationGroupSave,
  OrganizationRecord,
  OrganizationRolePair,
  OrganizationRoleRecord,
  OrganizationRoleSave,
} from '@/views/system-management/relation-permission/components/organization/model';
import permission from '@/views/system-management/relation-permission/components/organization/permission';
import { useUserStore } from '@/store';
import { useI18n } from 'vue-i18n';
import { RoleRecord } from '@/views/system-management/relation-permission/components/role/model';
import { Message } from '@arco-design/web-vue';
import { GroupRecord } from '@/views/system-management/relation-permission/components/group/model';
import { UserRecord } from '../user/model';

export default defineComponent({
  setup() {
    const { controlPoints, resources } = permission;
    const listVisible = hasControlPoint(controlPoints['organization.list']);
    const addVisible = hasControlPoint(controlPoints['organization.add']);
    const editVisible = hasControlPoint(controlPoints['organization.edit']);
    const viewVisible = hasControlPoint(controlPoints['organization.view']);
    const deleteVisible = hasControlPoint(controlPoints['organization.delete']);
    const enableVisible = hasControlPoint(controlPoints['organization.enable']);
    const disableVisible = hasControlPoint(
      controlPoints['organization.disable']
    );
    const organizationUserVisible = hasControlPoint(
      controlPoints['organization.user']
    );
    const organizationRoleVisible = hasControlPoint(
      controlPoints['organization.roleGroup']
    );
    const modalVisible = ref<boolean>(false);
    const organizationRoleGroupModalVisible = ref<boolean>(false);
    const organizationUserModalVisible = ref<boolean>(false);
    const isAdd = ref<boolean>(false);
    const isEdit = ref<boolean>(false);
    const isView = ref<boolean>(false);
    const organizationRelationOkLoading = ref<boolean>(false);
    const modalFormRef = ref();
    const userStore = useUserStore();
    const { t } = useI18n();
    const { loading, setLoading } = useLoading(false);
    const handleLoading = ref<boolean>(false);
    const organizationRoleLoading = ref<boolean>(false);
    const organizationGroupLoading = ref<boolean>(false);
    const organizationUserLoading = ref<boolean>(false);
    const organizationInfo =
      userStore.useOrganizationInfo() as OrganizationRecord;
    const systemInfo = userStore.useSystemInfo() as SystemInfo;
    const renderData = ref<OrganizationRecord[]>([organizationInfo]);
    const selectedOrganizationRecord = ref<OrganizationRecord>({});
    const roleRecordData = ref<RoleRecord[]>([]);
    const groupRecordData = ref<GroupRecord[]>([]);
    const organizationRelationActiveKey = ref<string>('organizationRoleList');
    const selectedGroupRowKeys = ref<string[]>([]);
    const userRecordData = ref<UserRecord[]>([]);
    const selectedRoleRowKeys = ref<string[]>([]);
    const formModel = ref(generateFormModel());
    const modalModel = ref(generateFormModel());
    const basePagination: Pagination = {
      current: 1,
      pageSize: 10,
    };
    const pagination = reactive({
      ...basePagination,
    });
    const statusOptions = computed<Options[]>(statusGetter);
    const loadMore = async (
      record: OrganizationRecord,
      done: (arg0: OrganizationRecord[]) => void
    ) => {
      setLoading(true);
      try {
        const { data } = await remoteResourceCall<
          OrganizationRecord,
          Array<OrganizationRecord>
        >(
          controlPoints['organization.list'],
          resources['organization.resources.tree.list'],
          {
            upOrganizationId: record.organizationId,
            systemId: systemInfo.systemId,
          }
        );
        done(data);
      } catch (err) {
        console.log(err);
      } finally {
        setLoading(false);
      }
    };
    const searchQuery = async () => {
      setLoading(true);
      try {
        const { data } = await remoteResourceCall<
          OrganizationRecord,
          Array<OrganizationRecord>
        >(
          controlPoints['organization.list'],
          resources['organization.resources.list'],
          {
            ...paramWrapper(formModel.value),
            upOrganizationId: organizationInfo.organizationId,
            systemId: systemInfo.systemId,
          }
        );
        renderData.value = data;
      } catch (err) {
        console.log(err);
      } finally {
        setLoading(false);
      }
    };
    const search = () => {
      searchQuery();
    };
    const reset = () => {
      formModel.value = generateFormModel();
      renderData.value = [organizationInfo];
    };
    const handleModalCancel = () => {
      modalVisible.value = false;
    };
    const handleModalConfirm = async () => {
      const { organizationCode, organizationName, organizationStatus } =
        modalModel.value;
      if (!organizationName) {
        modalFormRef.value.setFields({
          organizationName: {
            status: 'error',
            message: t('organization.organizationName.empty.warning'),
          },
        });
      } else if (!organizationCode) {
        modalFormRef.value.setFields({
          organizationCode: {
            status: 'error',
            message: t('organization.organizationCode.empty.warning'),
          },
        });
      } else if (!organizationStatus) {
        modalFormRef.value.setFields({
          organizationStatus: {
            status: 'error',
            message: t('organization.organizationStatus.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isAdd.value) {
          controlPoint = controlPoints['organization.add'];
          resource = resources['organization.resources.save'];
        } else {
          controlPoint = controlPoints['organization.edit'];
          resource = resources['organization.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<OrganizationRecord, void>(
            controlPoint,
            resource,
            {
              ...modalModel.value,
              systemId: systemInfo.systemId,
            }
          );
          modalVisible.value = false;
          search();
        } catch (err) {
          console.log(err);
        } finally {
          handleLoading.value = false;
        }
      }
    };
    const handleDeleteOk = async (record: OrganizationRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<OrganizationRecord, void>(
          controlPoints['organization.delete'],
          resources['organization.resources.delete'],
          {
            organizationId: record.organizationId,
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
    const handleDisableOk = async (record: OrganizationRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<OrganizationRecord, void>(
          controlPoints['organization.disable'],
          resources['organization.resources.disable'],
          {
            organizationId: record.organizationId,
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
    const handleEnableOk = async (record: OrganizationRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<OrganizationRecord, void>(
          controlPoints['organization.enable'],
          resources['organization.resources.enable'],
          {
            organizationId: record.organizationId,
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
    const handleAddModal = (record: OrganizationRecord) => {
      modalVisible.value = true;
      isAdd.value = true;
      isEdit.value = false;
      isView.value = false;
      modalModel.value = {
        ...generateFormModel(),
        upOrganizationName: record.organizationName,
        organizationLevel: `${
          parseInt(record.organizationLevel || '', 10) + 1
        }`,
        upOrganizationId: record.organizationId,
        lastModifiedUserId: userStore.userCode,
      };
    };
    const handleEditModal = (record: OrganizationRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = true;
      isView.value = false;
      modalModel.value = {
        ...record,
        upOrganizationName: record.upOrganizationName || '无',
        lastModifiedUserId: userStore.userCode,
      };
    };
    const handleViewModal = (record: OrganizationRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = false;
      isView.value = true;
      modalModel.value = {
        upOrganizationName: record.upOrganizationName || '无',
        ...record,
      };
    };
    const fetchOrganizationRoleData = async (params: OrganizationRecord) => {
      try {
        organizationRoleLoading.value = true;
        const { data } = await remoteResourceCall<
          OrganizationRecord,
          OrganizationRolePair
        >(
          controlPoints['organization.roleGroup'],
          resources['organization.resources.organizationRoleList'],
          params
        );
        roleRecordData.value = data.roleList;
        selectedRoleRowKeys.value = data.authorizedRoleIds;
      } catch (err) {
        console.log(err);
      } finally {
        organizationRoleLoading.value = false;
      }
    };
    const fetchOrganizationGroupData = async (params: OrganizationRecord) => {
      try {
        organizationGroupLoading.value = true;
        const { data } = await remoteResourceCall<
          OrganizationRecord,
          OrganizationGroupPair
        >(
          controlPoints['organization.roleGroup'],
          resources['organization.resources.organizationGroupList'],
          params
        );
        groupRecordData.value = data.groupList;
        selectedGroupRowKeys.value = data.authorizedGroupIds;
      } catch (err) {
        console.log(err);
      } finally {
        organizationGroupLoading.value = false;
      }
    };

    const handleOrganizationRoleModal = async (record: OrganizationRecord) => {
      selectedOrganizationRecord.value = record;
      organizationRoleGroupModalVisible.value = true;
      await fetchOrganizationRoleData({
        organizationId: selectedOrganizationRecord.value.organizationId,
        systemId: systemInfo.systemId,
      });
      await fetchOrganizationGroupData({
        organizationId: selectedOrganizationRecord.value.organizationId,
        systemId: systemInfo.systemId,
      });
    };
    const handleOrganizationUserModal = async (record: OrganizationRecord) => {
      try {
        organizationUserLoading.value = true;
        selectedOrganizationRecord.value = record;
        const { data } = await remoteResourceCall<
          OrganizationRecord,
          Array<OrganizationRoleRecord>
        >(
          controlPoints['organization.user'],
          resources['organization.resources.organizationUserList'],
          {
            organizationId: selectedOrganizationRecord.value.organizationId,
            systemId: systemInfo.systemId,
          }
        );
        userRecordData.value = data;
        organizationUserModalVisible.value = true;
      } catch (err) {
        console.log(err);
      } finally {
        organizationUserLoading.value = false;
      }
    };
    const handleOrganizationRelationSave = async () => {
      if (organizationRelationActiveKey.value === 'organizationRoleList') {
        try {
          organizationRelationOkLoading.value = true;
          await remoteResourceCall<OrganizationRoleSave, void>(
            controlPoints['organization.roleGroup'],
            resources['organization.resources.organizationRoleSave'],
            {
              organizationId: selectedOrganizationRecord.value.organizationId,
              systemId: systemInfo.systemId,
              authorizedRoleIds: selectedRoleRowKeys.value,
              lastModifiedUserId: userStore.userCode,
            }
          );
          Message.success(t('global.message.save.success'));
        } catch (err) {
          console.log(err);
        } finally {
          organizationRelationOkLoading.value = false;
        }
      } else {
        try {
          organizationRelationOkLoading.value = true;
          await remoteResourceCall<OrganizationGroupSave, void>(
            controlPoints['organization.roleGroup'],
            resources['organization.resources.organizationGroupSave'],
            {
              organizationId: selectedOrganizationRecord.value.organizationId,
              systemId: systemInfo.systemId,
              authorizedGroupIds: selectedGroupRowKeys.value,
              lastModifiedUserId: userStore.userCode,
            }
          );
          Message.success(t('global.message.save.success'));
        } catch (err) {
          console.log(err);
        } finally {
          organizationRelationOkLoading.value = false;
        }
      }
    };
    const selectedRoleChange = (keys: string[]) => {
      selectedRoleRowKeys.value = keys;
    };
    const selectedGroupChange = (keys: string[]) => {
      console.log(keys)
      selectedGroupRowKeys.value = keys;
    };
    const organizationRelationTabChange = (key: string) => {
      if (key === 'organizationRoleList') {
        fetchOrganizationRoleData({
          organizationId: selectedOrganizationRecord.value.organizationId,
          systemId: systemInfo.systemId,
        });
      } else {
        fetchOrganizationGroupData({
          organizationId: selectedOrganizationRecord.value.organizationId,
          systemId: systemInfo.systemId,
        });
      }
    };
    return {
      addVisible,
      listVisible,
      editVisible,
      deleteVisible,
      viewVisible,
      disableVisible,
      enableVisible,
      organizationUserVisible,
      organizationRoleVisible,
      modalVisible,
      organizationRoleGroupModalVisible,
      organizationUserModalVisible,
      handleOrganizationRoleModal,
      handleOrganizationUserModal,
      handleOrganizationRelationSave,
      organizationRelationOkLoading,
      selectedRoleChange,
      selectedGroupChange,
      isView,
      isAdd,
      isEdit,
      handleModalCancel,
      handleModalConfirm,
      handleDeleteOk,
      handleDisableOk,
      handleEnableOk,
      handleAddModal,
      handleEditModal,
      handleViewModal,
      modalFormRef,
      loading,
      handleLoading,
      organizationRoleLoading,
      organizationUserLoading,
      search,
      renderData,
      roleRecordData,
      userRecordData,
      selectedRoleRowKeys,
      pagination,
      formModel,
      modalModel,
      reset,
      statusOptions,
      loadMore,
      organizationRelationActiveKey,
      groupRecordData,
      selectedGroupRowKeys,
      organizationGroupLoading,
      organizationRelationTabChange
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
