<template>
  <div class="container">
    <a-card class="general-card" :body-style="{ paddingTop: '10px' }">
      <a-tabs>
        <a-tab-pane key="1">
          <template #title
            ><icon-mind-mapping />
            {{ t('menu.authority-management.organization') }}
          </template>
          <organization />
        </a-tab-pane>
        <a-tab-pane key="2">
          <template #title>
            <icon-user /> {{ t('menu.authority-management.user') }}
          </template>
          <user />
        </a-tab-pane>
        <a-tab-pane key="3">
          <template #title>
            <icon-user-group /> {{ t('menu.authority-management.role') }}
          </template>
          <role @authorize="authorizationButtonClick" />
        </a-tab-pane>
        <a-tab-pane key="4">
          <template #title>
            <icon-user-group /> {{ t('menu.authority-management.group') }}
          </template>
          <group />
        </a-tab-pane>
      </a-tabs>
    </a-card>
    <a-modal
      v-model:visible="authorizationModalVisible"
      unmount-on-close
      :modal-style="{ width: '800px' }"
      :footer="false"
    >
      <template #title>
        {{ t('menu.authority-management.title.authorization') }}
      </template>
      <a-tabs
        v-model:active-key="authorizationActiveKey"
        @change="authorizationTabChange"
      >
        <template #extra>
          <a-button type="primary" size="small" :loading="authorizationSaveLoading" @click="authorizationSave"
            >{{ t('global.button.save') }}</a-button
          >
        </template>
        <a-tab-pane key="menuAuthorization">
          <template #title>
            <icon-menu />
            {{ t('menu.authority-management.menuAuthorization') }}
          </template>
          <a-tree
            v-model:checked-keys="menuTreeCompleteCheckedRecordKeys"
            v-model:half-checked-keys="menuTreeHalfCheckedRecordKeys"
            :checkable="true"
            :data="menuTreeData"
            :virtual-list-props="{
              height: 600,
            }"
            @check="onMenuCheck"
          >
            <template #icon>
              <icon-font type="icon-letter-m" />
            </template>
          </a-tree>
        </a-tab-pane>
        <a-tab-pane key="permissionAuthorization">
          <template #title
            ><icon-exclamation-circle />
            {{ t('menu.authority-management.permissionAuthorization') }}
          </template>
          <a-tree
            v-model:checked-keys="permissionTreeCompleteCheckedRecordKeys"
            v-model:half-checked-keys="permissionTreeHalfCheckedRecordKeys"
            :checkable="true"
            :data="permissionTreeData"
            :virtual-list-props="{
              height: 600,
            }"
            @check="onPermissionCheck"
          >
            <template #icon="{ node }">
              <icon-font v-if="node.type === 'F'" type="icon-letter-f" />
              <icon-font v-if="node.type === 'R'" type="icon-letter-r" />
              <icon-font v-if="node.type === 'C'" type="icon-letter-c" />
              <icon-font v-if="node.type === 'D'" type="icon-letter-d" />
            </template>
          </a-tree>
        </a-tab-pane>
      </a-tabs>
    </a-modal>
  </div>
</template>

<script lang="ts">
import {defineComponent, reactive, ref} from 'vue';
import User from '@/views/system-management/relation-permission/components/user/index.vue';
import Role from '@/views/system-management/relation-permission/components/role/index.vue';
import Group from '@/views/system-management/relation-permission/components/group/index.vue';
import Organization from '@/views/system-management/relation-permission/components/organization/index.vue';
import {
  AuthorizationRawRecord,
  AuthorizationRecord,
  AuthorizedPair,
  TreeRecord,
} from './model';
import { MenuRecord } from '../page-configuration/components/menu/model';
import { remoteResourceCall } from '../../../utils';
import { Message, Icon } from '@arco-design/web-vue';
import permission from './permission';
import {SystemInfo} from "../../../types";
import {userInfo,systemInfo} from "../../../constants";

const IconFont = Icon.addFromIconFontCn({});

export default defineComponent({
  components: {
    User,
    Role,
    Group,
    Organization,
    IconFont,
  },
  setup() {
    const { controlPoints, resources } = permission;
    const authorizationModalVisible = ref<boolean>(false);
    const authorizationActiveKey = ref<string>('menuAuthorization');
    const authorizationRawRecord = ref<AuthorizationRawRecord>();
    const menuTreeCompleteCheckedRecords = ref<TreeRecord[]>([]);
    const menuTreeHalfCheckedRecords = ref<TreeRecord[]>([]);
    const menuTreeHalfCheckedRecordKeys = ref<string[]>([]);
    const menuTreeCompleteCheckedRecordKeys = ref<string[]>([]);
    const permissionTreeHalfCheckedRecords = ref<TreeRecord[]>([]);
    const permissionTreeCompleteCheckedRecords = ref<TreeRecord[]>([]);
    const permissionTreeHalfCheckedRecordKeys = ref<string[]>([]);
    const permissionTreeCompleteCheckedRecordKeys = ref<string[]>([]);
    const menuTreeData = ref<TreeRecord[]>([]);
    const permissionTreeData = ref<TreeRecord[]>([]);
    const authorizationSaveLoading = ref<boolean>(false);
    const onMenuCheck = (
      newCheckedKeys: any,
      event: {
        checkedNodes: Array<TreeRecord>;
        halfCheckedNodes: Array<TreeRecord>;
      }
    ) => {
      menuTreeCompleteCheckedRecords.value = event.checkedNodes.map((node) => ({
        ...node,
      }));
      menuTreeHalfCheckedRecords.value = event.halfCheckedNodes.map((node) => ({
        ...node,
      }));
    };
    const onPermissionCheck = (
      newCheckedKeys: any,
      event: {
        checkedNodes: Array<TreeRecord>;
        halfCheckedNodes: Array<TreeRecord>;
      }
    ) => {
      permissionTreeCompleteCheckedRecords.value = event.checkedNodes.map(
        (node) => ({ ...node })
      );
      permissionTreeHalfCheckedRecords.value = event.halfCheckedNodes.map(
        (node) => ({ ...node })
      );
    };
    const fetchAuthorizedMenu = async (params: AuthorizationRecord) => {
      try {
        const { data } = await remoteResourceCall<
          AuthorizationRecord,
          AuthorizedPair
        >(
          controlPoints['relationPermission.authorize'],
          resources['relationPermission.resources.authorizedMenuQuery'],
          params
        );
        menuTreeHalfCheckedRecordKeys.value = data.authorizedHalfKeys;
        menuTreeCompleteCheckedRecordKeys.value = data.authorizedCompleteKeys;
        menuTreeCompleteCheckedRecords.value = data.authorizedCompleteRecords;
        menuTreeHalfCheckedRecords.value = data.authorizedHalfRecords;
      } catch (err: any) {
        Message.error(err?.message);
      }
    };
    const fetchAuthorizedPermission = async (params: AuthorizationRecord) => {
      try {
        const { data } = await remoteResourceCall<
          AuthorizationRecord,
          AuthorizedPair
          >(
          controlPoints['relationPermission.authorize'],
          resources['relationPermission.resources.authorizedPermissionQuery'],
          params
        );
        permissionTreeHalfCheckedRecordKeys.value = data.authorizedHalfKeys;
        permissionTreeCompleteCheckedRecordKeys.value = data.authorizedCompleteKeys;
        permissionTreeCompleteCheckedRecords.value = data.authorizedCompleteRecords;
        permissionTreeHalfCheckedRecords.value = data.authorizedHalfRecords;
      } catch (err: any) {
        Message.error(err?.message);
      }
    };
    const fetchMenuTree = async (params: MenuRecord) => {
      try {
        const { data } = await remoteResourceCall<
          MenuRecord,
          Array<TreeRecord>
        >(
          controlPoints['relationPermission.authorize'],
          resources['relationPermission.resources.menuTreeQuery'],
          params
        );
        menuTreeData.value = data;
      } catch (err: any) {
        Message.error(err?.message);
      }
    };
    const fetchPermissionTree = async (params: MenuRecord) => {
      try {
        const { data } = await remoteResourceCall<
          MenuRecord,
          Array<TreeRecord>
        >(
          controlPoints['relationPermission.authorize'],
          resources['relationPermission.resources.permissionTreeQuery'],
          params
        );
        permissionTreeData.value = data;
      } catch (err: any) {
        Message.error(err?.message);
      }
    };
    const authorizationSave = async () => {
      if (authorizationActiveKey.value === 'menuAuthorization') {
        try {
          authorizationSaveLoading.value = true;
          const data: AuthorizationRecord = {
            authorizationType: authorizationRawRecord.value?.authorizationType,
            authorizationKey: authorizationRawRecord.value?.authorizationKey,
            systemId: systemInfo.systemId,
            lastModifiedUserId: userInfo.userCode,
            completeAuthorizationResources:
              menuTreeCompleteCheckedRecords.value.map((treeRecord) => ({
                resourceKey: treeRecord.key,
                resourceType: treeRecord.type,
              })),
            halfAuthorizationResources: menuTreeHalfCheckedRecords.value.map(
              (treeRecord) => ({
                resourceKey: treeRecord.key,
                resourceType: treeRecord.type,
              })
            ),
          };
          await remoteResourceCall<AuthorizationRecord, void>(
            controlPoints['relationPermission.authorize'],
            resources['relationPermission.resources.authorizationMenuSave'],
            data
          );
          Message.success(t('global.message.save.success'));
        } catch (err: any) {
          Message.error(err?.message);
        } finally {
          authorizationSaveLoading.value = false;
        }
      } else {
        try {
          authorizationSaveLoading.value = true;
          const data: AuthorizationRecord = {
            authorizationType: authorizationRawRecord.value?.authorizationType,
            authorizationKey: authorizationRawRecord.value?.authorizationKey,
            systemId: systemInfo.systemId,
            lastModifiedUserId: userInfo.userCode,
            completeAuthorizationResources:
              permissionTreeCompleteCheckedRecords.value.map((treeRecord) => ({
                resourceKey: treeRecord.key,
                resourceType: treeRecord.type,
              })),
            halfAuthorizationResources:
              permissionTreeHalfCheckedRecords.value.map((treeRecord) => ({
                resourceKey: treeRecord.key,
                resourceType: treeRecord.type,
              })),
          };
          await remoteResourceCall<AuthorizationRecord, void>(
            controlPoints['relationPermission.authorize'],
            resources[
              'relationPermission.resources.authorizationPermissionSave'
            ],
            data
          );
          Message.success(t('global.message.save.success'));
        } catch (err: any) {
          Message.error(err?.message);
        } finally {
          authorizationSaveLoading.value = false;
        }
      }
    };
    const authorizationButtonClick = (record: AuthorizationRawRecord) => {
      authorizationModalVisible.value = true;
      authorizationRawRecord.value = record;
      menuTreeHalfCheckedRecords.value = [];
      menuTreeCompleteCheckedRecords.value = [];
      menuTreeHalfCheckedRecordKeys.value = [];
      menuTreeCompleteCheckedRecordKeys.value = [];
      permissionTreeHalfCheckedRecords.value = [];
      permissionTreeCompleteCheckedRecords.value = [];
      permissionTreeHalfCheckedRecordKeys.value = [];
      permissionTreeCompleteCheckedRecordKeys.value = [];
      authorizationActiveKey.value = 'menuAuthorization';
      fetchMenuTree({
        systemId: systemInfo.systemId,
      });
      fetchAuthorizedMenu({
        authorizationType: authorizationRawRecord.value?.authorizationType,
        authorizationKey: authorizationRawRecord.value?.authorizationKey,
        systemId: systemInfo.systemId,
      });
    };
    const authorizationTabChange = (key: string) => {
      if (key === 'menuAuthorization') {
        fetchMenuTree({
          systemId: systemInfo.systemId,
        });
        fetchAuthorizedMenu({
          authorizationType: authorizationRawRecord.value?.authorizationType,
          authorizationKey: authorizationRawRecord.value?.authorizationKey,
          systemId: systemInfo.systemId,
        });
      } else {
        fetchPermissionTree({
          systemId: systemInfo.systemId,
        });
        fetchAuthorizedPermission({
          authorizationType: authorizationRawRecord.value?.authorizationType,
          authorizationKey: authorizationRawRecord.value?.authorizationKey,
          systemId: systemInfo.systemId,
        });
      }
    };
    const t = (key:string)=>{
      const map = {
        'menu.authority-management.user': '用户管理',
        'menu.authority-management.role': '角色管理',
        'menu.authority-management.organization': '机构管理',
        'menu.authority-management.group': '岗位管理',
        'menu.system-management.relation-permission': '组织权限配置',
        'menu.authority-management.menuAuthorization': '菜单授权',
        'menu.authority-management.permissionAuthorization': '权限授权',
        'menu.authority-management.title.authorization': '授权',
      }
      return map[key]
    }
    return {
      t,
      authorizationModalVisible,
      menuTreeData,
      permissionTreeData,
      authorizationActiveKey,
      onMenuCheck,
      onPermissionCheck,
      authorizationSave,
      authorizationButtonClick,
      menuTreeHalfCheckedRecordKeys,
      menuTreeCompleteCheckedRecordKeys,
      permissionTreeHalfCheckedRecordKeys,
      permissionTreeCompleteCheckedRecordKeys,
      authorizationTabChange,
      authorizationSaveLoading
    };
  },
});
</script>
