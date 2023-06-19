<template>
  <a-card class="general-card" :title="t('module.form.query.name')">
    <a-row>
      <a-col :flex="1">
        <a-form
          :model="moduleFormModel"
          :label-col-props="{ span: 4 }"
          :wrapper-col-props="{ span: 20 }"
          label-align="left"
        >
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="name" :label="t('module.form.name')">
                <a-input
                  v-model="moduleFormModel.moduleName"
                  :placeholder="t('module.form.name.placeholder')"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" :label="t('module.form.status')">
                <a-select
                  v-model="moduleFormModel.moduleStatus"
                  :options="statusOptions"
                  :placeholder="t('module.form.selectDefault')"
                />
              </a-form-item>
            </a-col>
          </a-row>
        </a-form>
      </a-col>
      <a-divider style="height: 84px" direction="vertical" />
      <a-col :flex="'86px'" style="text-align: right">
        <a-space direction="vertical" :size="18">
          <a-button type="primary" @click="searchModule">
            <template #icon>
              <icon-search />
            </template>
            {{ t('module.columns.operations.query') }}
          </a-button>
          <a-button @click="moduleReset">
            <template #icon>
              <icon-refresh />
            </template>
            {{ t('module.columns.operations.reset') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-divider style="margin-top: 0" />
    <a-row style="margin-bottom: 16px">
      <a-col :span="16">
        <a-space>
          <a-button
            type="primary"
            @click="handleModuleAddModal"
          >
            <template #icon>
              <icon-plus />
            </template>
            {{ t('module.columns.operations.add') }}
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-table
      row-key="moduleId"
      :loading="moduleLoading"
      :pagination="{
        ...modulePagination,
        showPageSize: true,
        showTotal: true,
        onPageSizeChange: onModulePageSizeChange,
      }"
      :data="moduleRenderData"
      :bordered="false"
      :expandable="
        {
              title: t('module.columns.functionExpand'),
              width: 100,
              expandedRowKeys: moduleExpandedRowKeys,
            }
      "
      @expanded-change="moduleExpandChange"
      @pageChange="onModulePageChange"
    >
      <template #expand-row>
        <a-card class="general-card" :title="t('function.form.query.name')">
          <a-row>
            <a-col :flex="1">
              <a-form
                :model="functionFormModel"
                :label-col-props="{ span: 4 }"
                :wrapper-col-props="{ span: 20 }"
                label-align="left"
              >
                <a-space direction="vertical" :size="0">
                  <a-form-item field="name" :label="t('function.form.name')">
                    <a-input
                      v-model="functionFormModel.functionName"
                      :placeholder="t('function.form.name.placeholder')"
                    />
                  </a-form-item>
                  <a-form-item
                    field="status"
                    :label="t('function.form.status')"
                  >
                    <a-select
                      v-model="functionFormModel.functionStatus"
                      :options="statusOptions"
                      :placeholder="t('function.form.selectDefault')"
                    />
                  </a-form-item>
                </a-space>
              </a-form>
            </a-col>
            <a-divider style="height: 84px" direction="vertical" />
            <a-col :flex="'86px'" style="text-align: right">
              <a-space direction="vertical" :size="18">
                <a-button type="primary" @click="searchFunction">
                  <template #icon>
                    <icon-search />
                  </template>
                  {{ t('function.columns.operations.query') }}
                </a-button>
                <a-button @click="functionReset">
                  <template #icon>
                    <icon-refresh />
                  </template>
                  {{ t('function.columns.operations.reset') }}
                </a-button>
              </a-space>
            </a-col>
          </a-row>
          <a-divider style="margin-top: 0" />
          <a-row style="margin-bottom: 16px">
            <a-col :span="16">
              <a-space>
                <a-button
                  type="primary"
                  @click="handleFunctionAddModal"
                >
                  <template #icon>
                    <icon-plus />
                  </template>
                  {{ t('function.columns.operations.add') }}
                </a-button>
              </a-space>
            </a-col>
          </a-row>
          <a-table
            row-key="functionId"
            :loading="functionLoading"
            :pagination="functionPagination"
            :data="functionRenderData"
            :bordered="false"
            @pageChange="onFunctionPageChange"
          >
            <template #columns>
              <a-table-column
                :title="t('function.columns.functionName')"
                data-index="functionName"
              />
              <a-table-column
                :title="t('function.columns.lastModifiedUserId')"
                data-index="lastModifiedUserId"
              />
              <a-table-column
                :title="t('function.columns.updateTime')"
                data-index="updateTime"
              />
              <a-table-column
                :title="t('function.columns.functionStatus')"
                data-index="functionStatus"
              >
                <template #cell="{ record }">
                  <span
                    v-if="record.functionStatus === '0'"
                    class="circle"
                  ></span>
                  <span v-else class="circle pass"></span>
                  {{
                    t(
                      `function.form.status.${
                        record.functionStatus === '0' ? 'inactive' : 'active'
                      }`
                    )
                  }}
                </template>
              </a-table-column>
              <a-table-column
                :title="t('function.columns.operations')"
                data-index="operations"
              >
                <template #cell="{ record }">
                  <a-button
                    type="text"
                    size="small"
                    @click="handleControlPointConfigurationModal(record)"
                  >
                    {{
                      t(
                        'function.columns.operations.controlPointConfiguration'
                      )
                    }}
                  </a-button>
                  <a-button
                    type="text"
                    size="small"
                    @click="handleFunctionEditModal(record)"
                  >
                    {{ t('function.columns.operations.edit') }}
                  </a-button>
                  <a-button
                    type="text"
                    size="small"
                    @click="handleFunctionViewModal(record)"
                  >
                    {{ t('function.columns.operations.view') }}
                  </a-button>
                  <a-popconfirm
                    :content="t('function.delete.alert.message')"
                    :ok-text="t('global.button.confirm')"
                    :cancel-text="t('global.button.cancel')"
                    :ok-loading="handleLoading"
                    @ok="handleFunctionDeleteOk(record)"
                  >
                    <a-button
                      type="text"
                      size="small"
                    >
                      {{ t('function.columns.operations.delete') }}
                    </a-button>
                  </a-popconfirm>
                  <a-popconfirm
                    v-if="record.functionStatus === '0'"
                    :content="t('function.enable.alert.message')"
                    :ok-text="t('global.button.confirm')"
                    :cancel-text="t('global.button.cancel')"
                    :ok-loading="handleLoading"
                    @ok="handleFunctionEnableOk(record)"
                  >
                    <a-button
                      type="text"
                      size="small"
                    >
                      {{ t('function.columns.operations.enable') }}
                    </a-button>
                  </a-popconfirm>
                  <a-popconfirm
                    v-else
                    :content="t('function.disable.alert.message')"
                    :ok-text="t('global.button.confirm')"
                    :cancel-text="t('global.button.cancel')"
                    :ok-loading="handleLoading"
                    @ok="handleFunctionDisableOk(record)"
                  >
                    <a-button
                      type="text"
                      size="small"
                    >
                      {{ t('function.columns.operations.disable') }}
                    </a-button>
                  </a-popconfirm>
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </template>
      <template #columns>
        <a-table-column
          :title="t('module.columns.moduleName')"
          data-index="moduleName"
        />
        <a-table-column
          :title="t('module.columns.lastModifiedUserId')"
          data-index="lastModifiedUserId"
        />
        <a-table-column
          :title="t('module.columns.updateTime')"
          data-index="updateTime"
        />
        <a-table-column
          :title="t('module.columns.moduleStatus')"
          data-index="moduleStatus"
        >
          <template #cell="{ record }">
            <span v-if="record.moduleStatus === '0'" class="circle"></span>
            <span v-else class="circle pass"></span>
            {{
              t(
                `module.form.status.${
                  record.moduleStatus === '0' ? 'inactive' : 'active'
                }`
              )
            }}
          </template>
        </a-table-column>
        <a-table-column
          :title="t('module.columns.operations')"
          data-index="operations"
        >
          <template #cell="{ record }">
            <a-button
              type="text"
              size="small"
              @click="handleModuleEditModal(record)"
            >
              {{ t('module.columns.operations.edit') }}
            </a-button>
            <a-button
              type="text"
              size="small"
              @click="handleModuleViewModal(record)"
            >
              {{ t('module.columns.operations.view') }}
            </a-button>
            <a-popconfirm
              :content="t('module.delete.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleModuleDeleteOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('module.columns.operations.delete') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-if="record.moduleStatus === '0'"
              :content="t('module.enable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleModuleEnableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('module.columns.operations.enable') }}
              </a-button>
            </a-popconfirm>
            <a-popconfirm
              v-else
              :content="t('module.disable.alert.message')"
              :ok-text="t('global.button.confirm')"
              :cancel-text="t('global.button.cancel')"
              :ok-loading="handleLoading"
              @ok="handleModuleDisableOk(record)"
            >
              <a-button type="text" size="small">
                {{ t('module.columns.operations.disable') }}
              </a-button>
            </a-popconfirm>
          </template>
        </a-table-column>
      </template>
    </a-table>
  </a-card>
  <a-modal
    v-model:visible="moduleModalVisible"
    unmount-on-close
    :modal-style="{ width: '800px' }"
  >
    <template v-if="isModuleAdd" #title>
      {{ t('module.add.modal.title') }}
    </template>
    <template v-else-if="isModuleEdit" #title>
      {{ t('module.edit.modal.title') }}
    </template>
    <template v-else #title> {{ t('module.view.modal.title') }}</template>
    <template v-if="isModuleAdd || isModuleEdit" #footer>
      <a-button @click="handleModuleModalCancel"
        >{{ t('global.button.cancel') }}
      </a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleModuleModalConfirm"
        >{{ t('global.button.confirm') }}
      </a-button>
    </template>
    <template v-else #footer><span></span></template>
    <a-form
      ref="moduleModalFormRef"
      :model="moduleModalModel"
      :label-col-props="{ span: 8 }"
      :wrapper-col-props="{ span: 16 }"
      label-align="left"
    >
      <a-row :gutter="16">
        <a-col :span="12">
          <a-form-item
            field="moduleName"
            :label="t('module.form.name')"
            :rules="[
              {
                required: true,
                message: t('module.moduleName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="moduleModalModel.moduleName"
              :disabled="isModuleView"
              :placeholder="t('module.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="moduleRemark"
            :label="t('module.form.remark')"
            :rules="[
              {
                required: true,
                message: t('module.moduleRemark.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="moduleModalModel.moduleRemark"
              :disabled="isModuleView"
              :placeholder="t('module.form.remark.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="moduleStatus"
            :label="t('module.form.status')"
            :rules="[
              {
                required: true,
                message: t('module.moduleStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="moduleModalModel.moduleStatus"
              :options="statusOptions"
              :disabled="isModuleView"
              :placeholder="t('module.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="controlModalVisible"
    unmount-on-close
    :modal-style="{ width: '800px' }"
  >
    <template v-if="isControlAdd" #title>
      {{ t('controlPoint.add.modal.title') }}
    </template>
    <template v-else-if="isControlEdit" #title>
      {{ t('controlPoint.edit.modal.title') }}
    </template>
    <template v-else #title>
      {{ t('controlPoint.view.modal.title') }}
    </template>
    <template v-if="isControlAdd || isControlEdit" #footer>
      <a-button @click="handleControlModalCancel"
        >{{ t('global.button.cancel') }}
      </a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleControlModalConfirm"
        >{{ t('global.button.confirm') }}
      </a-button>
    </template>
    <template v-else #footer><span></span></template>
    <a-form
      ref="controlPointModalFormRef"
      :model="controlPointModalModel"
      :label-col-props="{ span: 8 }"
      :wrapper-col-props="{ span: 16 }"
      label-align="left"
    >
      <a-row :gutter="16">
        <a-col :span="12">
          <a-form-item
            field="controlName"
            :label="t('controlPoint.form.name')"
            :rules="[
              {
                required: true,
                message: t('controlPoint.controlName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="controlPointModalModel.controlName"
              :disabled="isControlView"
              :placeholder="t('controlPoint.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="controlCode"
            :label="t('controlPoint.form.code')"
            :rules="[
              {
                required: true,
                message: t('controlPoint.controlCode.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="controlPointModalModel.controlCode"
              :disabled="isControlView"
              :placeholder="t('controlPoint.form.code.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="controlStatus"
            :label="t('controlPoint.form.status')"
            :rules="[
              {
                required: true,
                message: t('controlPoint.controlStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="controlPointModalModel.controlStatus"
              :options="statusOptions"
              :disabled="isControlView"
              :placeholder="t('controlPoint.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="functionModalVisible"
    unmount-on-close
    :modal-style="{ width: '800px' }"
  >
    <template v-if="isFunctionAdd" #title>
      {{ t('function.add.modal.title') }}
    </template>
    <template v-else-if="isFunctionEdit" #title>
      {{ t('function.edit.modal.title') }}
    </template>
    <template v-else #title> {{ t('function.view.modal.title') }}</template>
    <template v-if="isFunctionAdd || isFunctionEdit" #footer>
      <a-button @click="handleFunctionModalCancel"
        >{{ t('global.button.cancel') }}
      </a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleFunctionModalConfirm"
        >{{ t('global.button.confirm') }}
      </a-button>
    </template>
    <template v-else #footer><span></span></template>
    <a-form
      ref="functionModalFormRef"
      :model="functionModalModel"
      :label-col-props="{ span: 8 }"
      :wrapper-col-props="{ span: 16 }"
      label-align="left"
    >
      <a-row :gutter="16">
        <a-col :span="12">
          <a-form-item
            field="functionName"
            :label="t('function.form.name')"
            :rules="[
              {
                required: true,
                message: t('function.functionName.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'input']"
          >
            <a-input
              v-model="functionModalModel.functionName"
              :disabled="isFunctionView"
              :placeholder="t('function.form.name.placeholder')"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item
            field="functionStatus"
            :label="t('function.form.status')"
            :rules="[
              {
                required: true,
                message: t('function.functionStatus.empty.warning'),
              },
            ]"
            :validate-trigger="['change', 'blur']"
          >
            <a-select
              v-model="functionModalModel.functionStatus"
              :options="statusOptions"
              :disabled="isFunctionView"
              :placeholder="t('function.form.selectDefault')"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </a-modal>
  <a-modal
    v-model:visible="controlPointConfigurationModalVisible"
    unmount-on-close
    :footer="null"
    :modal-style="{ width: '80%', padding: 0 }"
  >
    <template #title>
      {{ t('controlPoint.configuration.modal.title') }}
    </template>
    <a-card class="general-card" :title="t('controlPoint.form.query.name')">
      <a-row>
        <a-col :flex="1">
          <a-form
            :model="controlPointFormModel"
            :label-col-props="{ span: 4 }"
            :wrapper-col-props="{ span: 20 }"
            label-align="left"
          >
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item
                  field="controlName"
                  :label="t('controlPoint.form.name')"
                >
                  <a-input
                    v-model="controlPointFormModel.controlName"
                    :placeholder="t('controlPoint.form.name.placeholder')"
                  />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item
                  field="controlCode"
                  :label="t('controlPoint.form.code')"
                >
                  <a-input
                    v-model="controlPointFormModel.controlCode"
                    :placeholder="t('controlPoint.form.code.placeholder')"
                  />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item
                  field="controlStatus"
                  :label="t('controlPoint.form.status')"
                >
                  <a-select
                    v-model="controlPointFormModel.controlStatus"
                    :options="statusOptions"
                    :placeholder="t('controlPoint.form.selectDefault')"
                  />
                </a-form-item>
              </a-col>
            </a-row>
          </a-form>
        </a-col>
        <a-divider style="height: 84px" direction="vertical" />
        <a-col :flex="'86px'" style="text-align: right">
          <a-space direction="vertical" :size="18">
            <a-button type="primary" @click="searchControlPoint">
              <template #icon>
                <icon-search />
              </template>
              {{ t('controlPoint.columns.operations.query') }}
            </a-button>
            <a-button @click="controlPointReset">
              <template #icon>
                <icon-refresh />
              </template>
              {{ t('controlPoint.columns.operations.reset') }}
            </a-button>
          </a-space>
        </a-col>
      </a-row>
      <a-divider style="margin-top: 0" />
      <a-row style="margin-bottom: 16px">
        <a-col :span="16">
          <a-space>
            <a-button
              type="primary"
              @click="handleControlAddModal"
            >
              <template #icon>
                <icon-plus />
              </template>
              {{ t('controlPoint.columns.operations.add') }}
            </a-button>
          </a-space>
        </a-col>
      </a-row>
      <a-table
        row-key="controlId"
        :loading="controlPointLoading"
        :pagination="controlPointPagination"
        :data="controlPointRenderData"
        :bordered="false"
        :scroll="{ y: '500px' }"
        :expandable="
          {
                title: t('controlPoint.columns.resourceExpand'),
                width: 100,
                expandedRowKeys: controlPointExpandedRowKeys,
              }
        "
        @expanded-change="controlPointExpandChange"
        @pageChange="onControlPointPageChange"
      >
        <template #expand-row>
          <a-card class="general-card" :title="t('resource.form.list')">
            <a-row style="margin-bottom: 16px">
              <a-col :span="16">
                <a-space>
                  <a-button
                    type="primary"
                    @click="handleResourceAddModal"
                  >
                    <template #icon>
                      <icon-plus />
                    </template>
                    {{ t('module.columns.operations.add') }}
                  </a-button>
                </a-space>
              </a-col>
            </a-row>
            <a-table
              row-key="resourceId"
              :loading="resourceLoading"
              :pagination="false"
              :data="resourceRenderData"
              :bordered="false"
            >
              <template #columns>
                <a-table-column
                  :title="t('resource.columns.resourceCode')"
                  data-index="resourceCode"
                />
                <a-table-column
                  :title="t('resource.columns.resourceRemark')"
                  data-index="resourceRemark"
                />
                <a-table-column
                  :title="t('resource.columns.resourceUrl')"
                  data-index="resourceUrl"
                />
                <a-table-column
                  :title="t('resource.columns.resourceMethodType')"
                  data-index="resourceMethodType"
                />
                <a-table-column
                  :title="t('resource.columns.resourceQueryFlag')"
                  data-index="resourceQueryFlag"
                >
                  <template #cell="{ record }">
                    <span
                      v-if="record.resourceQueryFlag !== '1'"
                      class="circle"
                    ></span>
                    <span v-else class="circle pass"></span>
                    {{
                      t(
                        `resource.options.queryFlag.${
                          record.resourceQueryFlag === '1' ? 'yes' : 'no'
                        }`
                      )
                    }}
                  </template>
                </a-table-column>
                <a-table-column
                  :title="t('resource.columns.operations')"
                  data-index="operations"
                >
                  <template #cell="{ record }">
                    <a-button
                      type="text"
                      size="small"
                      @click="handleResourceEditModal(record)"
                    >
                      {{ t('resource.columns.operations.edit') }}
                    </a-button>
                    <a-button
                      type="text"
                      size="small"
                      @click="handleResourceViewModal(record)"
                    >
                      {{ t('resource.columns.operations.view') }}
                    </a-button>
                    <a-popconfirm
                      :content="t('resource.delete.alert.message')"
                      :ok-text="t('global.button.confirm')"
                      :cancel-text="t('global.button.cancel')"
                      :ok-loading="handleLoading"
                      @ok="handleResourceDeleteOk(record)"
                    >
                      <a-button
                        type="text"
                        size="small"
                      >
                        {{ t('resource.columns.operations.delete') }}
                      </a-button>
                    </a-popconfirm>
                  </template>
                </a-table-column>
              </template>
            </a-table>
          </a-card>
        </template>
        <template #columns>
          <a-table-column
            :title="t('controlPoint.columns.controlName')"
            data-index="controlName"
          />
          <a-table-column
            :title="t('controlPoint.columns.controlCode')"
            data-index="controlCode"
          />
          <a-table-column
            :title="t('controlPoint.columns.lastModifiedUserId')"
            data-index="lastModifiedUserId"
          />
          <a-table-column
            :title="t('controlPoint.columns.updateTime')"
            data-index="updateTime"
          />
          <a-table-column
            :title="t('controlPoint.columns.controlStatus')"
            data-index="functionStatus"
          >
            <template #cell="{ record }">
              <span v-if="record.controlStatus === '0'" class="circle"></span>
              <span v-else class="circle pass"></span>
              {{
                t(
                  `controlPoint.form.status.${
                    record.controlStatus === '0' ? 'inactive' : 'active'
                  }`
                )
              }}
            </template>
          </a-table-column>
          <a-table-column
            :title="t('controlPoint.columns.operations')"
            data-index="operations"
            :width="280"
          >
            <template #cell="{ record }">
              <a-button
                type="text"
                size="small"
                @click="handleControlEditModal(record)"
              >
                {{ t('controlPoint.columns.operations.edit') }}
              </a-button>
              <a-button
                type="text"
                size="small"
                @click="handleControlViewModal(record)"
              >
                {{ t('controlPoint.columns.operations.view') }}
              </a-button>
              <a-popconfirm
                :content="t('controlPoint.delete.alert.message')"
                :ok-text="t('global.button.confirm')"
                :cancel-text="t('global.button.cancel')"
                :ok-loading="handleLoading"
                @ok="handleControlPointDeleteOk(record)"
              >
                <a-button
                  type="text"
                  size="small"
                >
                  {{ t('controlPoint.columns.operations.delete') }}
                </a-button>
              </a-popconfirm>
              <a-popconfirm
                v-if="record.controlStatus === '0'"
                :content="t('controlPoint.enable.alert.message')"
                :ok-text="t('global.button.confirm')"
                :cancel-text="t('global.button.cancel')"
                :ok-loading="handleLoading"
                @ok="handleControlPointEnableOk(record)"
              >
                <a-button
                  type="text"
                  size="small"
                >
                  {{ t('controlPoint.columns.operations.enable') }}
                </a-button>
              </a-popconfirm>
              <a-popconfirm
                v-else
                :content="t('controlPoint.disable.alert.message')"
                :ok-text="t('global.button.confirm')"
                :cancel-text="t('global.button.cancel')"
                :ok-loading="handleLoading"
                @ok="handleControlPointDisableOk(record)"
              >
                <a-button
                  type="text"
                  size="small"
                >
                  {{ t('controlPoint.columns.operations.disable') }}
                </a-button>
              </a-popconfirm>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>
  </a-modal>
  <a-modal
    v-model:visible="resourceModalVisible"
    unmount-on-close
    :modal-style="{ width: '60%', padding: 0 }"
  >
    <template v-if="isResourceAdd" #title>
      {{ t('resource.add.modal.title') }}
    </template>
    <template v-else-if="isResourceEdit" #title>
      {{ t('resource.edit.modal.title') }}
    </template>
    <template v-else #title> {{ t('resource.view.modal.title') }}</template>
    <template v-if="isResourceAdd || isResourceEdit" #footer>
      <a-button @click="handleResourceModalCancel"
        >{{ t('global.button.cancel') }}
      </a-button>
      <a-button
        type="primary"
        :loading="handleLoading"
        @click="handleResourceModalConfirm"
        >{{ t('global.button.confirm') }}
      </a-button>
    </template>
    <template v-else #footer><span></span></template>
    <a-form
      ref="resourceModalFormRef"
      :model="resourceModalModel"
      :label-col-props="{ span: 8 }"
      :wrapper-col-props="{ span: 16 }"
      label-align="left"
    >
      <a-card class="general-card" :title="t('resource.card.resourceInfo')">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item
              field="resourceCode"
              :label="t('resource.form.code')"
              :validate-trigger="['change', 'input']"
              required
            >
              <a-input
                v-model="resourceModalModel.resourceCode"
                :disabled="isResourceView"
                :placeholder="t('resource.form.code.placeholder')"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item
              field="resourceRemark"
              :label="t('resource.form.remark')"
              :validate-trigger="['change', 'input']"
              required
            >
              <a-input
                v-model="resourceModalModel.resourceRemark"
                :disabled="isResourceView"
                :placeholder="t('resource.form.remark.placeholder')"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item
              field="resourceUrl"
              :label="t('resource.form.url')"
              :validate-trigger="['change', 'input']"
              required
            >
              <a-input
                v-model="resourceModalModel.resourceUrl"
                :disabled="isResourceView"
                :placeholder="t('resource.form.url.placeholder')"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item
              field="resourceMethodType"
              :label="t('resource.form.method')"
              required
              :validate-trigger="['change', 'blur']"
            >
              <a-select
                v-model="resourceModalModel.resourceMethodType"
                :options="methodOptions"
                :disabled="isResourceView"
                :placeholder="t('resource.form.method.placeholder')"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item
              field="resourceQueryFlag"
              :label="t('resource.form.query')"
              required
              :validate-trigger="['change', 'blur']"
            >
              <a-select
                v-model="resourceModalModel.resourceQueryFlag"
                :options="yesOrNoOptions"
                :disabled="isResourceView"
                :placeholder="t('resource.form.query.selectDefault')"
              />
            </a-form-item>
          </a-col>
          <a-col v-if="resourceModalModel.resourceQueryFlag === '1'" :span="12">
            <a-form-item
              field="authorizations"
              :label="t('resource.form.authorization')"
            >
              <a-select
                v-model="resourceModalModel.authorizations"
                :options="dataAuthorizationOptions"
                :disabled="isResourceView"
                multiple
                :placeholder="t('resource.form.authorization.selectDefault')"
              />
            </a-form-item>
          </a-col>
        </a-row>
      </a-card>
      <a-card class="general-card">
        <template #title>
          {{ t('resource.card.clipRule') }}
          <a-tooltip :content="t('resource.clipRule.tip')">
            <icon-exclamation-circle />
          </a-tooltip>
        </template>
        <template v-if="!isResourceView" #actions>
          <a-button @click="resourceModalModelAddClipRule">
            <icon-plus-circle />
          </a-button>
        </template>
        <a-space direction="vertical" fill>
          <a-row
            v-for="(rule, index) in resourceModalModel.resourceClipRules"
            :key="index"
          >
            <a-form-item field="resourceClipRules" no-style>
              <a-row :gutter="8">
                <a-col :span="rule.type === '6' ? 4 : 8">
                  <a-form-item
                    field="resourceClipRules"
                    :validate-trigger="['change', 'input']"
                    required
                    no-style
                  >
                    <a-input
                      v-model="rule.keyword"
                      :disabled="isResourceView"
                      :placeholder="t('resource.clipRule.keyword.placeholder')"
                    />
                  </a-form-item>
                </a-col>
                <a-col :span="rule.type === '6' ? 4 : 8">
                  <a-form-item
                    field="resourceClipRules"
                    :validate-trigger="['change', 'blur']"
                    required
                    no-style
                  >
                    <a-select
                      v-model="rule.type"
                      :options="clipRuleTypeOptions"
                      :disabled="isResourceView"
                      :placeholder="t('resource.clipRule.type.placeholder')"
                    />
                  </a-form-item>
                </a-col>
                <a-col v-if="rule.type === '6'" :span="6">
                  <a-form-item
                    field="resourceClipRules"
                    :validate-trigger="['change', 'input']"
                    required
                    no-style
                  >
                    <a-input
                      v-model="rule.regex"
                      :disabled="isResourceView"
                      :placeholder="t('resource.clipRule.regex.placeholder')"
                    />
                  </a-form-item>
                </a-col>
                <a-col v-if="rule.type === '6'" :span="6">
                  <a-form-item
                    field="resourceClipRules"
                    :validate-trigger="['change', 'input']"
                    required
                    no-style
                  >
                    <a-input
                      v-model="rule.replacement"
                      :disabled="isResourceView"
                      :placeholder="
                        t('resource.clipRule.replacement.placeholder')
                      "
                    />
                  </a-form-item>
                </a-col>
                <a-col :span="2">
                  <a-button @click="resourceModalModelDeleteClipRule(index)">
                    <icon-minus-circle />
                  </a-button>
                </a-col>
              </a-row>
            </a-form-item>
          </a-row>
        </a-space>
      </a-card>
    </a-form>
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
}  from "../../../../../types";
import {
  FunctionRecord,
  generateFunctionFormModel,
  generateModuleFormModel,
  generateControlPointFormModel,
  ControlPointRecord,
  ModuleRecord,
  ResourceRecord,
  generateResourceFormModel,
  generateClipRuleFormModel,
} from './model';
import permission from './permission';
import {
  remoteResourceCall,
  clipRuleGetter,
  dataAuthorizationGetter,
  methodGetter,
  paramWrapper,
  statusGetter,
  yesOrNoGetter,
} from "../../../../../utils";
import {userInfo,systemInfo} from "../../../../../constants";


export default defineComponent({
  setup() {
    const { controlPoints, resources } = permission;
    const moduleModalVisible = ref<boolean>(false);
    const isModuleAdd = ref<boolean>(false);
    const isModuleEdit = ref<boolean>(false);
    const isModuleView = ref<boolean>(false);
    const moduleModalFormRef = ref();
    const functionModalVisible = ref<boolean>(false);
    const isFunctionAdd = ref<boolean>(false);
    const isFunctionEdit = ref<boolean>(false);
    const isFunctionView = ref<boolean>(false);
    const functionModalFormRef = ref();
    const resourceModalFormRef = ref();
    const controlPointModalFormRef = ref();
    const controlModalVisible = ref<boolean>(false);
    const isControlAdd = ref<boolean>(false);
    const isControlEdit = ref<boolean>(false);
    const isControlView = ref<boolean>(false);
    const resourceModalVisible = ref<boolean>(false);
    const isResourceAdd = ref<boolean>(false);
    const isResourceEdit = ref<boolean>(false);
    const isResourceView = ref<boolean>(false);
    const controlPointConfigurationModalVisible = ref<boolean>(false);
    const moduleExpandedRowKeys = ref<string[]>([]);
    const controlPointExpandedRowKeys = ref<string[]>([]);
    const handleLoading = ref<boolean>(false);
    const moduleLoading = ref<boolean>(false);
    const functionLoading = ref<boolean>(false);
    const controlPointLoading = ref<boolean>(false);
    const resourceLoading = ref<boolean>(false);
    const moduleRenderData = ref<ModuleRecord[]>([]);
    const moduleFormModel = ref(generateModuleFormModel());
    const moduleModalModel = ref(generateModuleFormModel());
    const functionRenderData = ref<ModuleRecord[]>([]);
    const functionFormModel = ref(generateFunctionFormModel());
    const functionModalModel = ref(generateFunctionFormModel());
    const resourceModalModel = ref(generateResourceFormModel());
    const controlPointRenderData = ref<ControlPointRecord[]>([]);
    const resourceRenderData = ref<ResourceRecord[]>([]);
    const test = generateControlPointFormModel();
    const controlPointFormModel = ref(test);
    const controlPointModalModel = ref(generateControlPointFormModel());
    const basePagination: Pagination = {
      current: 1,
      pageSize: 10,
    };
    const modulePagination = reactive({
      ...basePagination,
    });
    const functionPagination = reactive({
      ...basePagination,
    });
    const controlPointPagination = reactive({
      ...basePagination,
    });
    const statusOptions = computed<Options[]>(statusGetter);
    const yesOrNoOptions = computed<Options[]>(yesOrNoGetter);
    const dataAuthorizationOptions = computed<Options[]>(
      dataAuthorizationGetter
    );
    const clipRuleTypeOptions = computed<Options[]>(clipRuleGetter);
    const methodOptions = computed<Options[]>(methodGetter);
    const fetchModuleData = async (params: PaginationQuery<ModuleRecord>) => {
      moduleLoading.value = true;
      try {
        const { data } = await remoteResourceCall<
          PaginationQuery<ModuleRecord>,
          PaginationResult<ModuleRecord>
        >(
          controlPoints['module.list'],
          resources['module.resources.list'],
          params
        );
        moduleRenderData.value = data.dataList;
        modulePagination.current = data.currentPage;
        modulePagination.pageSize = data.pageSize;
        modulePagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        moduleLoading.value = false;
      }
    };
    const fetchControlPointData = async (
      params: PaginationQuery<ControlPointRecord>
    ) => {
      controlPointLoading.value = true;
      try {
        const { data } = await remoteResourceCall<
          PaginationQuery<ControlPointRecord>,
          PaginationResult<ControlPointRecord>
        >(
          controlPoints['controlPoint.list'],
          resources['controlPoint.resources.list'],
          params
        );
        controlPointRenderData.value = data.dataList;
        controlPointPagination.current = data.currentPage;
        controlPointPagination.pageSize = data.pageSize;
        controlPointPagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        controlPointLoading.value = false;
      }
    };
    const fetchControlResourceData = async (controlId: string) => {
      resourceLoading.value = true;
      try {
        const { data } = await remoteResourceCall<
          { controlId: string },
          Array<ResourceRecord>
        >(
          controlPoints['resourcePoint.list'],
          resources['resourcePoint.resources.list'],
          {
            controlId,
          }
        );
        resourceRenderData.value = data;
      } catch (err) {
        console.log(err);
      } finally {
        resourceLoading.value = false;
      }
    };
    const fetchFunctionData = async (
      params: PaginationQuery<FunctionRecord>
    ) => {
      functionLoading.value = true;
      try {
        const { data } = await remoteResourceCall<
          PaginationQuery<FunctionRecord>,
          PaginationResult<FunctionRecord>
        >(
          controlPoints['function.list'],
          resources['function.resources.list'],
          params
        );
        functionRenderData.value = data.dataList;
        functionPagination.current = data.currentPage;
        functionPagination.pageSize = data.pageSize;
        functionPagination.total = data.total;
      } catch (err) {
        console.log(err);
      } finally {
        functionLoading.value = false;
      }
    };
    const searchModule = () => {
      moduleExpandedRowKeys.value = [];
      fetchModuleData({
        condition: {
          ...paramWrapper(moduleFormModel.value),
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
    };
    const searchControlPoint = async () => {
      controlPointExpandedRowKeys.value = [];
      fetchControlPointData({
        condition: {
          ...paramWrapper(controlPointFormModel.value),
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
    };
    const searchFunction = () => {
      fetchFunctionData({
        condition: {
          ...paramWrapper(functionFormModel.value),
          moduleId: moduleExpandedRowKeys.value[0],
        },
        currentPage: basePagination.current,
        pageSize: basePagination.pageSize,
      });
    };
    const onModulePageChange = (currentPage: number) => {
      fetchModuleData({
        ...basePagination,
        condition: {
          ...paramWrapper(moduleFormModel.value),
          systemId: systemInfo.systemId,
        },
        currentPage,
      });
    };
    const onControlPointPageChange = (currentPage: number) => {
      fetchControlPointData({
        ...basePagination,
        condition: paramWrapper(controlPointFormModel.value),
        currentPage,
      });
    };
    const onModulePageSizeChange = (pageSize: number) => {
      basePagination.current = 1;
      basePagination.pageSize = pageSize;
      fetchModuleData({
        ...basePagination,
        condition: {
          ...paramWrapper(moduleFormModel.value),
          systemId: systemInfo.systemId,
        },
        currentPage: basePagination.current,
      });
    };
    const onFunctionPageChange = (currentPage: number) => {
      fetchFunctionData({
        ...basePagination,
        condition: paramWrapper(functionFormModel.value),
        currentPage,
      });
    };
    const onFunctionPageSizeChange = (pageSize: number) => {
      basePagination.current = 1;
      basePagination.pageSize = pageSize;
      fetchFunctionData({
        ...basePagination,
        condition: paramWrapper(functionFormModel.value),
        currentPage: basePagination.current,
      });
    };
    searchModule();
    const moduleReset = () => {
      moduleFormModel.value = generateModuleFormModel();
      searchModule();
    };
    const controlPointReset = () => {
      controlPointFormModel.value = {
        ...generateControlPointFormModel(),
        functionId: controlPointFormModel.value.functionId,
      };
      searchControlPoint();
    };
    const handleModuleModalCancel = () => {
      moduleModalVisible.value = false;
    };
    const handleControlModalCancel = () => {
      controlModalVisible.value = false;
    };
    const handleControlModalConfirm = async () => {
      const { controlName, controlCode, controlStatus } =
        controlPointModalModel.value;
      if (!controlName) {
        controlPointModalFormRef.value.setFields({
          controlName: {
            status: 'error',
            message: t('controlPoint.controlName.empty.warning'),
          },
        });
      } else if (!controlCode) {
        controlPointModalFormRef.value.setFields({
          controlCode: {
            status: 'error',
            message: t('controlPoint.controlCode.empty.warning'),
          },
        });
      } else if (!controlStatus) {
        controlPointModalFormRef.value.setFields({
          controlStatus: {
            status: 'error',
            message: t('controlPoint.controlStatus.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isControlAdd.value) {
          controlPoint = controlPoints['controlPoint.add'];
          resource = resources['controlPoint.resources.save'];
        } else {
          controlPoint = controlPoints['controlPoint.edit'];
          resource = resources['controlPoint.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<ControlPointRecord, void>(
            controlPoint,
            resource,
            {
              ...paramWrapper(controlPointModalModel.value),
              lastModifiedUserId: userInfo.userCode,
            }
          );
          controlModalVisible.value = false;
          controlPointReset();
        } catch (err) {
          console.log(err);
        } finally {
          handleLoading.value = false;
        }
      }
    };
    const handleModuleModalConfirm = async () => {
      const { moduleName, moduleStatus, moduleRemark } = moduleModalModel.value;
      if (!moduleName) {
        moduleModalFormRef.value.setFields({
          moduleName: {
            status: 'error',
            message: t('module.moduleName.empty.warning'),
          },
        });
      } else if (!moduleRemark) {
        moduleModalFormRef.value.setFields({
          moduleRemark: {
            status: 'error',
            message: t('module.moduleRemark.empty.warning'),
          },
        });
      } else if (!moduleStatus) {
        moduleModalFormRef.value.setFields({
          moduleStatus: {
            status: 'error',
            message: t('module.moduleStatus.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isModuleAdd.value) {
          controlPoint = controlPoints['module.add'];
          resource = resources['module.resources.save'];
        } else {
          controlPoint = controlPoints['module.edit'];
          resource = resources['module.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<ModuleRecord, void>(controlPoint, resource, {
            ...moduleModalModel.value,
            systemId: systemInfo.systemId,
          });
          moduleModalVisible.value = false;
          moduleReset();
        } catch (err) {
          console.log(err);
        } finally {
          handleLoading.value = false;
        }
      }
    };
    const handleResourceModalCancel = () => {
      resourceModalVisible.value = false;
    };
    const handleResourceModalConfirm = async () => {
      const { resourceCode, resourceRemark } = resourceModalModel.value;
      if (!resourceCode) {
        resourceModalFormRef.value.setFields({
          resourceCode: {
            status: 'error',
            message: t('resource.resourceCode.empty.warning'),
          },
        });
      } else if (!resourceRemark) {
        resourceModalFormRef.value.setFields({
          resourceRemark: {
            status: 'error',
            message: t('resource.resourceRemark.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isResourceAdd.value) {
          controlPoint = controlPoints['resourcePoint.add'];
          resource = resources['resourcePoint.resources.save'];
        } else {
          controlPoint = controlPoints['resourcePoint.edit'];
          resource = resources['resourcePoint.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<ResourceRecord, void>(
            controlPoint,
            resource,
            {
              ...resourceModalModel.value,
              lastModifiedUserId: userInfo.userCode,
            }
          );
          resourceModalVisible.value = false;
          await fetchControlResourceData(
            resourceModalModel.value.controlId as string
          );
        } catch (err) {
          console.log(err);
        } finally {
          handleLoading.value = false;
        }
      }
    };
    const functionReset = () => {
      functionFormModel.value = {
        ...generateFunctionFormModel(),
        moduleId: functionFormModel.value.moduleId,
      };
      searchFunction();
    };
    const handleFunctionModalCancel = () => {
      functionModalVisible.value = false;
    };
    const handleFunctionModalConfirm = async () => {
      const { functionName, functionStatus } = functionModalModel.value;
      if (!functionName) {
        functionModalFormRef.value.setFields({
          functionName: {
            status: 'error',
            message: t('function.functionName.empty.warning'),
          },
        });
      } else if (!functionStatus) {
        functionModalFormRef.value.setFields({
          functionStatus: {
            status: 'error',
            message: t('function.functionStatus.empty.warning'),
          },
        });
      } else {
        let controlPoint = '';
        let resource = '';
        if (isFunctionAdd.value) {
          controlPoint = controlPoints['function.add'];
          resource = resources['function.resources.save'];
        } else {
          controlPoint = controlPoints['function.edit'];
          resource = resources['function.resources.edit'];
        }
        try {
          handleLoading.value = true;
          await remoteResourceCall<FunctionRecord, void>(
            controlPoint,
            resource,
            {
              ...functionModalModel.value,
              systemId: systemInfo.systemId,
            }
          );
          functionModalVisible.value = false;
          functionReset();
        } catch (err) {
          console.log(err);
        } finally {
          handleLoading.value = false;
        }
      }
    };
    const handleModuleDeleteOk = async (record: ModuleRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ModuleRecord, void>(
          controlPoints['module.delete'],
          resources['module.resources.delete'],
          {
            moduleId: record.moduleId,
            systemId: systemInfo.systemId,
          }
        );
        moduleReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleModuleDisableOk = async (record: ModuleRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ModuleRecord, void>(
          controlPoints['module.disable'],
          resources['module.resources.disable'],
          {
            moduleId: record.moduleId,
            systemId: systemInfo.systemId,
          }
        );
        moduleReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleModuleEnableOk = async (record: ModuleRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ModuleRecord, void>(
          controlPoints['module.enable'],
          resources['module.resources.enable'],
          {
            moduleId: record.moduleId,
            systemId: systemInfo.systemId,
          }
        );
        moduleReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleFunctionDeleteOk = async (record: FunctionRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<FunctionRecord, void>(
          controlPoints['function.delete'],
          resources['function.resources.delete'],
          {
            functionId: record.functionId,
            moduleId: record.moduleId,
            systemId: systemInfo.systemId,
          }
        );
        functionReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleFunctionDisableOk = async (record: FunctionRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<FunctionRecord, void>(
          controlPoints['function.disable'],
          resources['function.resources.disable'],
          {
            functionId: record.functionId,
            moduleId: record.moduleId,
            systemId: systemInfo.systemId,
          }
        );
        functionReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleFunctionEnableOk = async (record: FunctionRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<FunctionRecord, void>(
          controlPoints['function.enable'],
          resources['function.resources.enable'],
          {
            functionId: record.functionId,
            moduleId: record.moduleId,
            systemId: systemInfo.systemId,
          }
        );
        functionReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleControlPointDeleteOk = async (record: ControlPointRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ControlPointRecord, void>(
          controlPoints['controlPoint.delete'],
          resources['controlPoint.resources.delete'],
          {
            controlId: record.controlId,
            systemId: systemInfo.systemId,
          }
        );
        controlPointReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleControlPointDisableOk = async (record: ControlPointRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ControlPointRecord, void>(
          controlPoints['controlPoint.disable'],
          resources['controlPoint.resources.disable'],
          {
            controlId: record.controlId,
            systemId: systemInfo.systemId,
          }
        );
        controlPointReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleControlPointEnableOk = async (record: ControlPointRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ControlPointRecord, void>(
          controlPoints['controlPoint.enable'],
          resources['controlPoint.resources.enable'],
          {
            controlId: record.controlId,
            systemId: systemInfo.systemId,
          }
        );
        controlPointReset();
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleResourceDeleteOk = async (record: ResourceRecord) => {
      try {
        handleLoading.value = true;
        await remoteResourceCall<ResourceRecord, void>(
          controlPoints['resourcePoint.delete'],
          resources['resourcePoint.resources.delete'],
          {
            resourceId: record.resourceId,
          }
        );
        await fetchControlResourceData(
          resourceModalModel.value.controlId as string
        );
      } catch (err) {
        console.log(err);
      } finally {
        handleLoading.value = false;
      }
    };
    const handleModuleAddModal = () => {
      moduleModalVisible.value = true;
      isModuleAdd.value = true;
      isModuleEdit.value = false;
      isModuleView.value = false;
      moduleModalModel.value = {
        ...generateModuleFormModel(),
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleModuleEditModal = (record: ModuleRecord) => {
      moduleModalVisible.value = true;
      isModuleAdd.value = false;
      isModuleEdit.value = true;
      isModuleView.value = false;
      moduleModalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleModuleViewModal = (record: ModuleRecord) => {
      moduleModalVisible.value = true;
      isModuleAdd.value = false;
      isModuleEdit.value = false;
      isModuleView.value = true;
      moduleModalModel.value = {
        ...record,
      };
    };
    const handleFunctionAddModal = () => {
      functionModalVisible.value = true;
      isFunctionAdd.value = true;
      isFunctionEdit.value = false;
      isFunctionView.value = false;
      functionModalModel.value = {
        ...generateFunctionFormModel(),
        moduleId: functionModalModel.value.moduleId,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleFunctionEditModal = (record: FunctionRecord) => {
      functionModalVisible.value = true;
      isFunctionAdd.value = false;
      isFunctionEdit.value = true;
      isFunctionView.value = false;
      functionModalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleFunctionViewModal = (record: FunctionRecord) => {
      functionModalVisible.value = true;
      isFunctionAdd.value = false;
      isFunctionEdit.value = false;
      isFunctionView.value = true;
      functionModalModel.value = {
        ...record,
      };
    };
    const handleControlAddModal = () => {
      controlModalVisible.value = true;
      isControlAdd.value = true;
      isControlEdit.value = false;
      isControlView.value = false;
      controlPointModalModel.value = {
        ...generateControlPointFormModel(),
        functionId: controlPointModalModel.value.functionId,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleControlEditModal = (record: ControlPointRecord) => {
      controlModalVisible.value = true;
      isControlAdd.value = false;
      isControlEdit.value = true;
      isControlView.value = false;
      controlPointModalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleControlViewModal = (record: ControlPointRecord) => {
      controlModalVisible.value = true;
      isControlAdd.value = false;
      isControlEdit.value = false;
      isControlView.value = true;
      controlPointModalModel.value = {
        ...record,
      };
    };
    const handleResourceAddModal = () => {
      resourceModalVisible.value = true;
      isResourceAdd.value = true;
      isResourceEdit.value = false;
      isResourceView.value = false;
      resourceModalModel.value = {
        ...generateResourceFormModel(),
        controlId: resourceModalModel.value.controlId,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleResourceEditModal = (record: ResourceRecord) => {
      resourceModalVisible.value = true;
      isResourceAdd.value = false;
      isResourceEdit.value = true;
      isResourceView.value = false;
      resourceModalModel.value = {
        ...record,
        lastModifiedUserId: userInfo.userCode,
      };
    };
    const handleResourceViewModal = (record: ResourceRecord) => {
      resourceModalVisible.value = true;
      isResourceAdd.value = false;
      isResourceEdit.value = false;
      isResourceView.value = true;
      resourceModalModel.value = {
        ...record,
      };
    };
    const handleControlPointConfigurationModal = (record: FunctionRecord) => {
      controlPointConfigurationModalVisible.value = true;
      controlPointFormModel.value = {
        ...controlPointFormModel.value,
        functionId: record.functionId,
      };
      controlPointModalModel.value.functionId = record.functionId;
      searchControlPoint();
    };
    const moduleExpandChange = (keyArray: string[]) => {
      moduleExpandedRowKeys.value = keyArray.filter(
        (key) => !moduleExpandedRowKeys.value.includes(key)
      );
      if (keyArray.length !== 0) {
        searchFunction();
        functionModalModel.value.moduleId = moduleExpandedRowKeys
          .value[0] as string;
      }
    };
    const controlPointExpandChange = (keyArray: string[]) => {
      controlPointExpandedRowKeys.value = keyArray.filter(
        (key) => !controlPointExpandedRowKeys.value.includes(key)
      );
      if (keyArray.length !== 0) {
        const controlPoint = controlPointRenderData.value.filter(
          (controlPoint) =>
            controlPoint.controlId === controlPointExpandedRowKeys.value[0]
        );
        resourceModalModel.value.controlId = controlPoint[0].controlId;
        fetchControlResourceData(resourceModalModel.value.controlId as string);
      }
    };
    const resourceModalModelAddClipRule = () => {
      resourceModalModel.value.resourceClipRules?.push(
        generateClipRuleFormModel()
      );
    };
    const resourceModalModelDeleteClipRule = (index: number) => {
      resourceModalModel.value.resourceClipRules?.splice(index, 1);
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
        'menu.system-management.module': '模块管理',
        'module.form.query.name': '模块查询',
        'module.form.name': '模块名称',
        'module.form.name.placeholder': '请输入模块名称',
        'module.form.remark': '模块描述',
        'module.form.remark.placeholder': '请输入模块描述',
        'module.form.status': '模块状态',
        'module.form.status.active': '生效',
        'module.form.status.inactive': '失效',
        'module.form.selectDefault': '全部',
        'module.columns.functionExpand': '展开业务功能',
        'module.columns.moduleCode': '模块代码',
        'module.columns.moduleName': '模块名称',
        'module.columns.moduleStatus': '模块状态',
        'module.columns.updateTime': '最新变更时间',
        'module.columns.lastModifiedUserId': '最新变更用户',
        'module.columns.operations': '操作',
        'module.columns.operations.query': '查询',
        'module.columns.operations.reset': '重置',
        'module.columns.operations.add': '新增',
        'module.columns.operations.edit': '编辑',
        'module.columns.operations.view': '查看',
        'module.columns.operations.delete': '删除',
        'module.columns.operations.disable': '禁用',
        'module.columns.operations.enable': '启用',
        'module.columns.operations.moduleUser': '模块用户',
        'module.columns.operations.moduleMutex': '模块互斥',
        'module.add.modal.title': '新增模块',
        'module.edit.modal.title': '编辑模块',
        'module.view.modal.title': '查看模块',
        'module.delete.alert.message': '您确定要删除该模块吗?',
        'module.enable.alert.message': '您确定要启用该模块吗?',
        'module.disable.alert.message': '您确定要禁用该模块吗?',
        'module.moduleName.empty.warning': '模块名称为空.',
        'module.moduleRemark.empty.warning': '模块描述为空.',
        'module.moduleStatus.empty.warning': '模块状态未选择.',
        'function.form.query.name': '业务功能查询',
        'function.form.name': '业务功能名称',
        'function.form.name.placeholder': '请输入业务功能名称',
        'function.form.status': '业务功能状态',
        'function.form.status.active': '生效',
        'function.form.status.inactive': '失效',
        'function.form.selectDefault': '全部',
        'function.columns.functionCode': '业务功能代码',
        'function.columns.functionName': '业务功能名称',
        'function.columns.functionStatus': '业务功能状态',
        'function.columns.updateTime': '最新变更时间',
        'function.columns.lastModifiedUserId': '最新变更用户',
        'function.columns.operations': '操作',
        'function.columns.operations.query': '查询',
        'function.columns.operations.reset': '重置',
        'function.columns.operations.add': '新增',
        'function.columns.operations.edit': '编辑',
        'function.columns.operations.view': '查看',
        'function.columns.operations.delete': '删除',
        'function.columns.operations.disable': '禁用',
        'function.columns.operations.enable': '启用',
        'function.columns.operations.controlPointConfiguration': '控制点配置',
        'function.add.modal.title': '新增业务功能',
        'function.edit.modal.title': '编辑业务功能',
        'function.view.modal.title': '查看业务功能',
        'function.delete.alert.message': '您确定要删除该业务功能吗?',
        'function.enable.alert.message': '您确定要启用该业务功能吗?',
        'function.disable.alert.message': '您确定要禁用该业务功能吗?',
        'function.functionName.empty.warning': '业务功能名称为空.',
        'function.functionCode.empty.warning': '业务功能代码为空.',
        'function.functionStatus.empty.warning': '业务功能状态未选择.',
        'controlPoint.form.query.name': '控制点查询',
        'controlPoint.form.name': '控制点名称',
        'controlPoint.form.name.placeholder': '请输入控制点名称',
        'controlPoint.form.code': '控制点代码',
        'controlPoint.form.code.placeholder': '请输入控制点代码',
        'controlPoint.form.status': '控制点状态',
        'controlPoint.form.status.active': '生效',
        'controlPoint.form.status.inactive': '失效',
        'controlPoint.form.selectDefault': '全部',
        'controlPoint.columns.resourceExpand': '展开资源点',
        'controlPoint.columns.controlCode': '控制点代码',
        'controlPoint.columns.controlName': '控制点名称',
        'controlPoint.columns.controlStatus': '控制点状态',
        'controlPoint.columns.updateTime': '最新变更时间',
        'controlPoint.columns.lastModifiedUserId': '最新变更用户',
        'controlPoint.columns.operations': '操作',
        'controlPoint.columns.operations.query': '查询',
        'controlPoint.columns.operations.reset': '重置',
        'controlPoint.columns.operations.add': '新增',
        'controlPoint.columns.operations.edit': '编辑',
        'controlPoint.columns.operations.view': '查看',
        'controlPoint.columns.operations.delete': '删除',
        'controlPoint.columns.operations.disable': '禁用',
        'controlPoint.columns.operations.enable': '启用',
        'controlPoint.add.modal.title': '新增控制点',
        'controlPoint.edit.modal.title': '编辑控制点',
        'controlPoint.view.modal.title': '查看控制点',
        'controlPoint.configuration.modal.title': '配置控制点',
        'controlPoint.delete.alert.message': '您确定要删除该控制点吗?',
        'controlPoint.enable.alert.message': '您确定要启用该控制点吗?',
        'controlPoint.disable.alert.message': '您确定要禁用该控制点吗?',
        'controlPoint.controlName.empty.warning': '控制点名称为空.',
        'controlPoint.controlCode.empty.warning': '控制点代码为空.',
        'controlPoint.controlStatus.empty.warning': '控制点状态未选择.',
        'resource.form.list': '资源点列表',
        'resource.options.queryFlag.yes': '是',
        'resource.options.queryFlag.no': '否',
        'resource.columns.resourceQueryFlag': '是否查询',
        'resource.columns.resourceCode': '资源点代码',
        'resource.columns.resourceName': '资源点名称',
        'resource.columns.resourceStatus': '资源点状态',
        'resource.columns.resourceRemark': '用途',
        'resource.columns.resourceUrl': '资源点URL',
        'resource.columns.resourceMethodType': '请求方式',
        'resource.columns.operations': '操作',
        'resource.columns.operations.query': '查询',
        'resource.columns.operations.reset': '重置',
        'resource.columns.operations.add': '新增',
        'resource.columns.operations.edit': '编辑',
        'resource.columns.operations.view': '查看',
        'resource.columns.operations.delete': '删除',
        'resource.columns.operations.disable': '禁用',
        'resource.columns.operations.enable': '启用',
        'resource.add.modal.title': '新增资源点',
        'resource.edit.modal.title': '编辑资源点',
        'resource.view.modal.title': '查看资源点',
        'resource.delete.alert.message': '您确定要删除该资源点吗?',
        'resource.enable.alert.message': '您确定要启用该资源点吗?',
        'resource.disable.alert.message': '您确定要禁用该资源点吗?',
        'resource.resourceName.empty.warning': '资源点名称为空.',
        'resource.resourceCode.empty.warning': '资源点代码为空.',
        'resource.resourceStatus.empty.warning': '资源点状态未选择.',
        'resource.form.code': '资源点代码',
        'resource.form.code.placeholder': '请输入资源点代码',
        'resource.form.remark': '资源点用途',
        'resource.form.remark.placeholder': '请输入资源点用途',
        'resource.form.url': '资源链接',
        'resource.form.url.placeholder': '请输入资源点链接',
        'resource.form.method': '请求方式',
        'resource.form.method.placeholder': '请选择请求方式',
        'resource.form.query': '是否查询',
        'resource.form.query.selectDefault': '请选择是否查询标志',
        'resource.form.authorization': '数据权限',
        'resource.form.authorization.selectDefault': '不配置',
        'resource.card.clipRule': '脱敏规则',
        'resource.card.resourceInfo': '资源点信息',
        'resource.clipRule.keyword.placeholder': '字段名称',
        'resource.clipRule.type.placeholder': '过滤类型',
        'resource.clipRule.regex.placeholder': '过滤正则',
        'resource.clipRule.replacement.placeholder': '过滤替换',
        'resource.clipRule.tip':'针对资源点的正则脱敏，需填写正确的接口字段名称和过滤规则',
      }
      return map[key]
    }
    return {
      t,
      controlPointConfigurationModalVisible,
      moduleModalVisible,
      functionModalVisible,
      controlModalVisible,
      isModuleAdd,
      isFunctionAdd,
      isModuleEdit,
      isFunctionEdit,
      isModuleView,
      isFunctionView,
      isControlAdd,
      isControlEdit,
      isControlView,
      resourceModalVisible,
      isResourceAdd,
      isResourceEdit,
      isResourceView,
      moduleModalFormRef,
      functionModalFormRef,
      resourceModalFormRef,
      controlPointModalFormRef,
      handleLoading,
      moduleLoading,
      functionLoading,
      controlPointLoading,
      resourceLoading,
      searchModule,
      searchFunction,
      onModulePageChange,
      onFunctionPageChange,
      onControlPointPageChange,
      onModulePageSizeChange,
      onFunctionPageSizeChange,
      handleModuleAddModal,
      handleFunctionAddModal,
      handleModuleEditModal,
      handleFunctionEditModal,
      handleModuleViewModal,
      handleFunctionViewModal,
      handleControlAddModal,
      handleControlEditModal,
      handleControlViewModal,
      handleControlPointConfigurationModal,
      handleModuleModalCancel,
      handleResourceModalCancel,
      handleFunctionModalCancel,
      handleControlModalCancel,
      handleControlModalConfirm,
      handleModuleModalConfirm,
      handleFunctionModalConfirm,
      handleResourceModalConfirm,
      handleModuleDeleteOk,
      handleModuleDisableOk,
      handleModuleEnableOk,
      handleFunctionDeleteOk,
      handleFunctionDisableOk,
      handleFunctionEnableOk,
      handleControlPointDeleteOk,
      handleControlPointDisableOk,
      handleControlPointEnableOk,
      handleResourceDeleteOk,
      handleResourceAddModal,
      handleResourceEditModal,
      handleResourceViewModal,
      resourceModalModelAddClipRule,
      resourceModalModelDeleteClipRule,
      moduleRenderData,
      modulePagination,
      moduleFormModel,
      moduleModalModel,
      moduleReset,
      searchControlPoint,
      controlPointReset,
      functionRenderData,
      functionPagination,
      functionFormModel,
      functionModalModel,
      controlPointPagination,
      controlPointRenderData,
      controlPointFormModel,
      controlPointModalModel,
      resourceModalModel,
      functionReset,
      resourceRenderData,
      statusOptions,
      yesOrNoOptions,
      dataAuthorizationOptions,
      clipRuleTypeOptions,
      methodOptions,
      moduleExpandedRowKeys,
      controlPointExpandedRowKeys,
      controlPointExpandChange,
      moduleExpandChange,
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
