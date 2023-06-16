
export interface UserRecord {
  systemId?:string;
  userId?: string;
  organizationId?: string;
  userCode?: string;
  userName?: string;
  userStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
  organizationIds?:Array<string>;
}

export type users = Array<UserRecord>;

export interface UserOrganizationRoleRecord{
  orgRoleRelId?: string;
  userId?: string;
  userCode?: string;
  userName?: string;
  organizationId?: string;
  organizationCode?: string;
  organizationName?: string;
  roleId?:string;
  roleName?:string;
  roleCode?:string;
  disabled?:boolean;
}
export interface UserOrganizationGroupRecord{
  orgRoleRelId?: string;
  userId?: string;
  userCode?: string;
  userName?: string;
  organizationId?: string;
  organizationCode?: string;
  organizationName?: string;
  groupId?:string;
  groupName?:string;
  groupCode?:string;
  disabled?:boolean;
}
export interface UserRolePair{
  authorizedOrganizationRoleRelIds:Array<string>;

  userOrganizationRoleList:Array<UserOrganizationRoleRecord>;
}
export interface UserGroupPair{
  authorizedOrganizationGroupRelIds:Array<string>;

  userOrganizationGroupList:Array<UserOrganizationGroupRecord>;
}
export interface UserRoleSave{
  systemId?: string;
  userId?: string;
  lastModifiedUserId?: string;
  authorizedOrgRoleRelIds:Array<string>;
}

export interface UserGroupSave{
  systemId?: string;
  userId?: string;
  lastModifiedUserId?: string;
  authorizedOrgGroupRelIds:Array<string>;
}

export const generateFormModel = (): UserRecord => {
  return {
    organizationId: '',
    userId: '',
    userCode: '',
    userName: '',
    userStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
    organizationIds:[]
  };
};
