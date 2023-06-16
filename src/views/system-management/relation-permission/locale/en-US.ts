import localeGroup from '@/views/system-management/relation-permission/components/group/locale/en-US';
import localeRole from '@/views/system-management/relation-permission/components/role/locale/en-US';
import localeUser from '@/views/system-management/relation-permission/components/user/locale/en-US';
import localeOrganization from '@/views/system-management/relation-permission/components/organization/locale/en-US';

export default {
  ...localeOrganization,
  ...localeGroup,
  ...localeUser,
  ...localeRole,
  'menu.system-management.relation-permission': 'Relation Permission Configuration',
  'menu.authority-management.menuAuthorization': 'Menu Authorization',
  'menu.authority-management.permissionAuthorization': 'Permission Authorization',
  'menu.authority-management.title.authorization': 'Authorization',
};
