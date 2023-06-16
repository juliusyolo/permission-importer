import localeGroup from '@/views/system-management/relation-permission/components/group/locale/zh-CN';
import localeRole from '@/views/system-management/relation-permission/components/role/locale/zh-CN';
import localeUser from '@/views/system-management/relation-permission/components/user/locale/zh-CN';
import localeOrganization from '@/views/system-management/relation-permission/components/organization/locale/zh-CN';

export default {
  ...localeOrganization,
  ...localeGroup,
  ...localeUser,
  ...localeRole,
  'menu.system-management.relation-permission': '组织权限配置',
  'menu.authority-management.menuAuthorization': '菜单授权',
  'menu.authority-management.permissionAuthorization': '权限授权',
  'menu.authority-management.title.authorization': '授权',
};
