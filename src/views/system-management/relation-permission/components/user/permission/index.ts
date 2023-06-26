export default {
  controlPoints:  {
    'user.list': 'userList',
    'user.add': 'userAdd',
    'user.edit': 'userEdit',
    'user.view': 'userView',
    'user.delete': 'userDelete',
    'user.enable': 'userEnable',
    'user.disable': 'userDisable',
    'user.userRoleGroup': 'userRoleGroup',
  },
  resources:{
    'user.resources.list': 'get_user_list_by_pagination',
    'user.resources.save': 'add_user',
    'user.resources.edit': 'edit_user',
    'user.resources.view': 'view',
    'user.resources.delete': 'delete_user',
    'user.resources.enable': 'enable_user',
    'user.resources.disable': 'disable_user',
    'user.resources.organizationList':'get_all_organization_list',
    'user.resources.userRoleList': 'get_user_role_list',
    'user.resources.userRoleSave': 'add_user_role',
    'user.resources.userGroupList': 'get_user_group_list',
    'user.resources.userGroupSave': 'add_user_group',
  }
}
