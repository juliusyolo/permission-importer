<template>
  <a-card class="general-card" :title="t('menu.form.query.name')">
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
              <a-form-item field="name" :label="t('menu.form.name')">
                <a-input
                  v-model="formModel.menuName"
                  :placeholder="t('menu.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" :label="t('menu.form.status')">
                <a-select
                  v-model="formModel.menuStatus"
                  :options="statusOptions"
                  :placeholder="t('menu.form.selectDefault')"
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
            {{ t('menu.columns.operations.query') }}
          </a-button>
          <a-button @click="reset">
            <template #icon>
              <icon-refresh />
            </template>
            {{ t('menu.columns.operations.reset') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-row style="margin-bottom: 16px">
      <a-col :span="16">
        <a-space>
          <a-button
            type="primary"
            @click="handleRootAddModal"
          >
            <template #icon>
              <icon-plus />
            </template>
            {{ t('menu.columns.operations.add') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-divider style="margin-top: 0" />
    <a-table
      row-key="menuId"
      :loading="loading"
      :pagination="false"
      :data="renderData"
      :bordered="false"
      :load-more="loadMore"
    >
      <template #columns>
        <a-table-column
          :title="t('menu.columns.menuName')"
          data-index="menuName"
          fixed="left"
        />
        <a-table-column
          :title="t('menu.columns.menuIcon')"
          data-index="menuIcon"
        >
          <template #cell="{ record }">
            <component :is="record?.menuIcon" />
          </template>
        </a-table-column>
        <a-table-column
          :title="t('menu.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="t('menu.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="t('menu.columns.menuStatus')"
          data-index="menuStatus"
        >
          <template #cell="{ record }">
            <span v-if="record.menuStatus === '0'" class="circle"></span>
            <span v-else class="circle pass"></span>
            {{
              t(
                `menu.form.status.${
                  record.menuStatus === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="t('menu.columns.operations')"
          data-index="operations"
          fixed="right"
        >
          <template #cell="{ record }">
            <a-button
              type="text"
              size="small"
              @click="handleAddModal(record)"
            >
              {{ t('menu.columns.operations.add') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleEditModal(record)"
            >
              {{ t('menu.columns.operations.edit') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleViewModal(record)"
            >
              {{ t('menu.columns.operations.view') }}
            </a-button>
            <a-popconfirm
              :content="t('menu.delete.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDeleteOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('menu.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.menuStatus === '0'"
              :content="t('menu.enable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleEnableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('menu.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="t('menu.disable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleDisableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('menu.columns.operations.disable') }}
              </a-button>
            </a-popconfirm>
            <a-button type="text" size="small">
              {{ t('menu.columns.operations.menuUser') }}
            </a-button>
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
    <template v-if="isAdd" #title> {{ t('menu.add.modal.title') }}</template>
    <template v-else-if="isEdit" #title>
      {{ t('menu.edit.modal.title') }}
    </template>
    <template v-else #title> {{ t('menu.view.modal.title') }}</template>
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
            field="upMenuName"
            :label="t('menu.form.upMenuName')"
            validate-trigger="input"
            required
          >
            <a-input
              v-model="modalModel.upMenuName"
              disabled
              :placeholder="t('menu.form.empty.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="menuName"
            :label="t('menu.form.name')"
            :rules="[
              {
                required: true,
                message: t('menu.menuName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.menuName"
              :disabled="isView"
              :placeholder="t('menu.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="menuOrder"
            :label="t('menu.form.menuOrder')"
            :rules="[
              {
                required: true,
                message: t('menu.menuOrder.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input-number
              v-model="modalModel.menuOrder"
              :disabled="isView"
              :precision="0"
              :min="1"
              :step="1"
              :placeholder="t('menu.form.menuOrder.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="menuStatus"
            :label="t('menu.form.status')"
            :rules="[
              {
                required: true,
                message: t('menu.menuStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="modalModel.menuStatus"
              :options="statusOptions"
              :disabled="isView"
              :placeholder="t('menu.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="menuRemark"
            :label="t('menu.form.menuRemark')"
            :rules="[
              {
                required: true,
                message: t('menu.menuRemark.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.menuRemark"
              :disabled="isView"
              :placeholder="t('menu.form.menuRemark.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="menuPath"
            :label="t('menu.form.menuPath')"
            :rules="[
              {
                required: true,
                message: t('menu.menuPath.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.menuPath"
              :disabled="isView"
              :placeholder="t('menu.form.menuPath.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="menuComponent"
            :label="t('menu.form.menuComponent')"
            :rules="[
              {
                required: true,
                message: t('menu.menuComponent.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.menuComponent"
              :disabled="isView"
              :placeholder="t('menu.form.menuComponent.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="i18nKey"
            :label="t('menu.form.i18nKey')"
            :rules="[
              {
                required: true,
                message: t('menu.i18nKey.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="modalModel.i18nKey"
              :disabled="isView"
              :placeholder="t('menu.form.i18nKey.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item field="menuIcon" :label="t('menu.form.menuIcon')">
            <a-select
              v-model="modalModel.menuIcon"
              :options="iconOptions"
              :disabled="isView"
              allow-clear
              :placeholder="
                isView
                  ? t('menu.form.empty.placeholder')
                  : t('menu.form.menuIcon.placeholder')
              "
            >
              <template #label="{ data }">
                <component :is="data?.label" />
              </template>
              <template #option="{ data }">
                <component :is="data?.label" />
              </template>
            </a-select>
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item field="menuIcon" :label="t('menu.form.functionId')">
            <a-input
              v-model="modalModel.functionName"
              disabled
              :placeholder="
                isView
                  ? t('menu.form.empty.placeholder')
                  : t('menu.form.functionId.placeholder')
              "
            />
            <a-button :disabled="isView" @click="clearModuleFunction">
              <icon-minus />
            </a-button>
            <a-button :disabled="isView" @click="handleModuleFunctionModal">
              <icon-plus />
            </a-button>
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="moduleFunctionModalVisible"
    unmount-on-close
    :modal-style="{ width: '800px' }"
    :ok-text="t('global.button.confirm')"
    :cancel-text="t('global.button.cancel')"
    :title="t('menu.moduleFunction.modal.title')"
    @ok="handleModuleFunctionModalConfirm"
    @cancel="handleModuleFunctionModalCancel"
  >
    <a-card class="general-card" :title="t('menu.moduleFunction.card.title')">
      <a-row>
        <a-col :flex="1">
          <a-form
            :model="functionFormModel"
            :label-col-props="{ span: 4 }"
            :wrapper-col-props="{ span: 20 }"
            label-align="left"
          >
            <a-space direction="vertical" :size="0">
              <a-form-item field="name" :label="t('module.form.name')">
                <a-input
                  v-model="functionFormModel.moduleName"
                  :placeholder="t('module.form.name.placeholder')"
                />
              </a-form-item>
              <a-form-item field="name" :label="t('function.form.name')">
                <a-input
                  v-model="functionFormModel.functionName"
                  :placeholder="t('function.form.name.placeholder')"
                />
              </a-form-item>
            </a-space>
          </a-form>
        </a-col>
        <a-divider style="height: 84px" direction="vertical" />
        <a-col :flex="'86px'" style="text-align: right">
          <a-space direction="vertical" :size="18">
            <a-button type="primary" @click="searchModuleFunction">
              <template #icon>
                <icon-search />
              </template>
              {{ t('menu.columns.operations.query') }}
            </a-button>
            <a-button @click="moduleFunctionReset">
              <template #icon>
                <icon-refresh />
              </template>
              {{ t('menu.columns.operations.reset') }}
            </a-button>
          </a-space>
        </a-col>
      </a-row>
      <a-divider style="margin-top: 0" />
      <a-table
        row-key="functionId"
        :loading="moduleFunctionLoading"
        :pagination="functionPagination"
        :data="moduleFunctionRenderData"
        :bordered="false"
        :row-selection="{ type: 'radio', selectedRowKeys }"
        @selection-change="selectionChange"
        @pageChange="onModuleFunctionPageChange"
      >
        <template #columns>
          <a-table-column
            :title="t('module.columns.moduleName')"
            data-index="moduleName"
          />
          <a-table-column
            :title="t('function.columns.functionName')"
            data-index="functionName"
          />
        </template>
      </a-table>
    </a-card>
  </a-modal>
</template>

<script lang="ts">
import {remoteResourceCall,iconGetter, paramWrapper, statusGetter } from "../../../../../utils";
import { defineComponent, computed, ref, reactive } from 'vue';
import {
  Pagination,
  Options,
  SystemInfo,
  PaginationQuery,
  PaginationResult,
} from "../../../../../types";
import { Message } from '@arco-design/web-vue';
import {FunctionRecord, generateFunctionFormModel} from "../module/model";
import {generateFormModel, MenuRecord} from "./model";
import permission from "./permission";
import {userInfo,systemInfo} from "../../../../../constants";

export default defineComponent({
  setup() {
    const { controlPoints, resources } = permission;
    const modalVisible = ref<boolean>(false);
    const moduleFunctionModalVisible = ref<boolean>(false);
    const isAdd = ref<boolean>(false);
    const isEdit = ref<boolean>(false);
    const isView = ref<boolean>(false);
    const modalFormRef = ref();
    const loading = ref<boolean>(false)
    const handleLoading = ref<boolean>(false);
    const renderData = ref<MenuRecord[]>([]);
    const formModel = ref(generateFormModel());
    const modalModel = ref(generateFormModel());
    const functionFormModel = ref(generateFunctionFormModel());
    const moduleFunctionLoading = ref<boolean>(false);
    const moduleFunctionRenderData = ref<FunctionRecord[]>([]);
    const selectedRowKeys = ref<string[]>([]);
    const basePagination: Pagination = {
      current: 1,
      pageSize: 10,
    };
    const functionPagination = reactive({
      ...basePagination,
    });
    const statusOptions = computed<Options[]>(statusGetter);
    const iconOptions = computed<Options[]>(iconGetter);
    const loadMore = async (
      record: MenuRecord,
      done: (arg0: MenuRecord[]) => void
    ) => {
      loading.value=true;
      try {
        const { data } = await remoteResourceCall<
          MenuRecord,
          Array<MenuRecord>
        >(controlPoints['menu.list'], resources['menu.resources.tree.list'], {
          upMenuId: record.menuId,
          systemId: systemInfo.systemId,
        });
        done(data);
      } catch (err) {
        console.log(err);
      } finally {
        loading.value=false;
      }
    };
    const fetchData = async (params: MenuRecord) => {
      loading.value=true;
      try {
        const { data } = await remoteResourceCall<
          MenuRecord,
          Array<MenuRecord>
        >(
          controlPoints['menu.list'],
          resources['menu.resources.tree.list'],
          params
        );
        renderData.value = data;
      } catch (err) {
        console.log(err);
      } finally {
        loading.value=false;
      }
    };
    fetchData({
      systemId: systemInfo.systemId,
    });
    const searchQuery = async () => {
      loading.value=true;
      try {
        const { data } = await remoteResourceCall<
          MenuRecord,
          Array<MenuRecord>
        >(controlPoints['menu.list'], resources['menu.resources.list'], {
          ...paramWrapper(formModel.value),
          systemId: systemInfo.systemId,
        });
        renderData.value = data;
      } catch (err) {
        console.log(err);
      } finally {
        loading.value=false;
      }
    };
    const search = () => {
      searchQuery();
    };
    const reset = () => {
      formModel.value = generateFormModel();
      fetchData({
        systemId: systemInfo.systemId,
      });
    };
    const fetchModuleFunctionData = async (
      params: PaginationQuery<FunctionRecord>
    ) => {
      loading.value=true;
      try {
        const { data } = await remoteResourceCall<
          PaginationQuery<FunctionRecord>,
          PaginationResult<FunctionRecord>
        >(
          controlPoints['menu.add'],
          resources['menu.resources.moduleFunctionList'],
          params
        );
        moduleFunctionRenderData.value = data.dataList;
        functionPagination.current = data.currentPage;
        functionPagination.pageSize = data.pageSize;
        functionPagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        loading.value=false;
      }
    };
    const handleModalCancel = () => {
      modalVisible.value = false;
    };
    const handleModalConfirm = async () => {
      const {
        menuOrder,
        menuName,
        menuStatus,
        menuRemark,
        menuPath,
        menuComponent,
        i18nKey,
      } = modalModel.value;
      if (!menuName) {
        modalFormRef.value.setFields({
          menuName: {
            status: 'error',
            message: t('menu.menuName.empty.warning'),
          },
        });
      } else if (!menuOrder) {
        modalFormRef.value.setFields({
          menuOrder: {
            status: 'error',
            message: t('menu.menuOrder.empty.warning'),
          },
        });
      } else if (!menuStatus) {
        modalFormRef.value.setFields({
          menuStatus: {
            status: 'error',
            message: t('menu.menuStatus.empty.warning'),
          },
        });
      } else if (!menuRemark) {
        modalFormRef.value.setFields({
          menuRemark: {
            status: 'error',
            message: t('menu.menuRemark.empty.warning'),
          },
        });
      } else if (!menuPath) {
        modalFormRef.value.setFields({
          menuPath: {
            status: 'error',
            message: t('menu.menuPath.empty.warning'),
          },
        });
      } else if (!menuComponent) {
        modalFormRef.value.setFields({
          menuComponent: {
            status: 'error',
            message: t('menu.menuComponent.empty.warning'),
          },
        });
      } else if (!i18nKey) {
        modalFormRef.value.setFields({
          i18nKey: {
            status: 'error',
            message: t('menu.i18nKey.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isAdd.value) {
          controlPoint = controlPoints['menu.add'];
          resource = resources['menu.resources.save'];
        } else {
          controlPoint = controlPoints['menu.edit'];
          resource = resources['menu.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<MenuRecord, void>(controlPoint, resource, {
            ...paramWrapper(modalModel.value),
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
    const handleDeleteOk = async (record: MenuRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<MenuRecord, void>(
          controlPoints['menu.delete'],
          resources['menu.resources.delete'],
          {
            menuId: record.menuId,
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
    const handleDisableOk = async (record: MenuRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<MenuRecord, void>(
          controlPoints['menu.disable'],
          resources['menu.resources.disable'],
          {
            menuId: record.menuId,
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
    const handleEnableOk = async (record: MenuRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<MenuRecord, void>(
          controlPoints['menu.enable'],
          resources['menu.resources.enable'],
          {
            menuId: record.menuId,
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
    const handleAddModal = (record: MenuRecord) => {
      modalVisible.value = true;
      isAdd.value = true;
      isEdit.value = false;
      isView.value = false;
      modalModel.value = {
        ...generateFormModel(),
        upMenuName: record.menuName,
        upMenuId: record.menuId,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleRootAddModal = () => {
      modalVisible.value = true;
      isAdd.value = true;
      isEdit.value = false;
      isView.value = false;
      modalModel.value = {
        ...generateFormModel(),
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleEditModal = (record: MenuRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = true;
      isView.value = false;
      modalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleViewModal = (record: MenuRecord) => {
      modalVisible.value = true;
      isAdd.value = false;
      isEdit.value = false;
      isView.value = true;
      modalModel.value = {
        ...record,
      };
    };
    const searchModuleFunction = () => {
      selectedRowKeys.value = [];
      fetchModuleFunctionData({
        condition: {
          ...paramWrapper(functionFormModel.value),
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
    };
    const onModuleFunctionPageChange = (currentPage: number) => {
      fetchModuleFunctionData({
        ...basePagination,
        condition: {
          ...paramWrapper(functionFormModel.value),
          systemId: systemInfo.systemId,
        },
        currentPage,
      });
    };
    const selectionChange = (rowKeys: string[]) => {
      selectedRowKeys.value = rowKeys;
    };
    const moduleFunctionReset = () => {
      functionFormModel.value = generateFunctionFormModel();
      searchModuleFunction();
    };
    const handleModuleFunctionModal = () => {
      moduleFunctionModalVisible.value = true;
      moduleFunctionReset();
    };
    const clearModuleFunction = () => {
      modalModel.value.functionId = '';
      modalModel.value.functionName = '';
    };
    const handleModuleFunctionModalConfirm = () => {
      if (selectedRowKeys.value.length === 1) {
        const functionRecord = moduleFunctionRenderData.value.filter(
          (record) => record.functionId === selectedRowKeys.value[0]
        )[0];
        modalModel.value.functionId = functionRecord.functionId;
        modalModel.value.functionName = functionRecord.functionName;
        //  moduleFunctionModalVisible.value = false;
      } else {
        Message.warning(t('menu.moduleFunction.empty.warning'));
      }
    };
    const handleModuleFunctionModalCancel = () => {
      moduleFunctionModalVisible.value = false;
    };
    const t = (key: string)=>{
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
        'menu.system-management.menu': '菜单管理',
        'menu.form.query.name': '菜单查询',
        'menu.moduleFunction.card.title': '业务功能查询',
        'menu.form.menuOrder': '菜单顺序',
        'menu.form.menuOrder.placeholder': '请输入菜单顺序',
        'menu.form.menuIcon': '菜单图标',
        'menu.form.menuIcon.placeholder': '请选择菜单图标',
        'menu.form.functionId': '关联业务功能',
        'menu.form.functionId.placeholder': '请选择关联业务功能',
        'menu.form.menuRemark': '菜单说明',
        'menu.form.menuRemark.placeholder': '请输入菜单说明',
        'menu.form.menuPath': '菜单路径',
        'menu.form.menuPath.placeholder': '请输入菜单路径',
        'menu.form.menuComponent': '菜单组件',
        'menu.form.menuComponent.placeholder': '请输入菜单组件',
        'menu.form.i18nKey': '国际化键',
        'menu.form.i18nKey.placeholder': '请输入国际化键',
        'menu.form.name': '菜单名称',
        'menu.form.name.placeholder': '请输入菜单名称',
        'menu.form.empty.placeholder': '无',
        'menu.form.status': '菜单状态',
        'menu.form.status.active': '生效',
        'menu.form.status.inactive': '失效',
        'menu.form.selectDefault': '全部',
        'menu.form.upMenuName': '上级菜单',
        'menu.form.upMenuName.placeholder': '请输入上级菜单',
        'menu.form.menuLevel': '菜单层级',
        'menu.form.menuLevel.placeholder': '请输入菜单层级',
        'menu.columns.menuIcon': '菜单图标',
        'menu.columns.menuName': '菜单名称',
        'menu.columns.upMenuName': '上级菜单名称',
        'menu.columns.menuStatus': '菜单状态',
        'menu.columns.updateTime': '最新变更时间',
        'menu.columns.lastModifiedUserId': '最新变更用户',
        'menu.columns.operations': '操作',
        'menu.columns.operations.query': '查询',
        'menu.columns.operations.reset': '重置',
        'menu.columns.operations.add': '新增子菜单',
        'menu.columns.operations.edit': '编辑',
        'menu.columns.operations.view': '查看',
        'menu.columns.operations.delete': '删除',
        'menu.columns.operations.disable': '禁用',
        'menu.columns.operations.enable': '启用',
        'menu.columns.operations.menuUser': '菜单用户',
        'menu.columns.operations.menuMutex': '菜单互斥',
        'menu.moduleFunction.modal.title': '业务功能列表',
        'menu.add.modal.title': '新增菜单',
        'menu.edit.modal.title': '编辑菜单',
        'menu.view.modal.title': '查看菜单',
        'menu.delete.alert.message': '您确定要删除该菜单吗?',
        'menu.enable.alert.message': '您确定要启用该菜单吗?',
        'menu.disable.alert.message': '您确定要禁用该菜单吗?',
        'menu.menuName.empty.warning': '菜单名称为空.',
        'menu.menuOrder.empty.warning': '菜单顺序为空.',
        'menu.menuStatus.empty.warning': '菜单状态未选择.',
        'menu.menuRemark.empty.warning': '菜单说明为空.',
        'menu.menuPath.empty.warning': '菜单路径为空.',
        'menu.menuComponent.empty.warning': '菜单组件为空.',
        'menu.i18nKey.empty.warning': '国际化键为空.',
        'menu.moduleFunction.empty.warning': '请选择业务功能！',
      }
      return map[key];
    }
    return {
      t,
      modalVisible,
      moduleFunctionModalVisible,
      isView,
      isAdd,
      isEdit,
      handleModalCancel,
      handleModalConfirm,
      handleDeleteOk,
      handleDisableOk,
      handleEnableOk,
      handleAddModal,
      handleRootAddModal,
      handleEditModal,
      handleViewModal,
      modalFormRef,
      loading,
      handleLoading,
      search,
      renderData,
      formModel,
      modalModel,
      reset,
      statusOptions,
      iconOptions,
      loadMore,
      moduleFunctionLoading,
      moduleFunctionRenderData,
      functionFormModel,
      searchModuleFunction,
      moduleFunctionReset,
      handleModuleFunctionModal,
      clearModuleFunction,
      functionPagination,
      handleModuleFunctionModalConfirm,
      handleModuleFunctionModalCancel,
      onModuleFunctionPageChange,
      selectionChange,
      selectedRowKeys,
    };
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
