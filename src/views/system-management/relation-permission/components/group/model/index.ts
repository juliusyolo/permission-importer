
export interface GroupRecord {
  systemId?:string;
  groupId?: string;
  groupCode?: string;
  groupName?: string;
  groupStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
}

export type groups = Array<GroupRecord>;
export interface GroupUserRecord {
  systemId?:string;
  roleId?:string;
  roleCode?:string;
  roleName?:string;
  userCode?:string;
  userId?:string;
  userName?:string;
  organizationId?: string;
  organizationCode?: string;
  organizationName?: string;
}
export const generateFormModel = (): GroupRecord => {
  return {
    groupId: '',
    groupCode: '',
    groupName: '',
    groupStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};
