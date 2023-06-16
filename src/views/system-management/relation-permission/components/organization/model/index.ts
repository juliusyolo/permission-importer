import { RoleRecord } from "@/views/system-management/relation-permission/components/role/model";

export interface OrganizationRecord {
  systemId?: string;
  organizationId?: string;
  organizationCode?: string;
  organizationName?: string;
  organizationLevel?: string;
  upOrganizationName?: string;
  upOrganizationId?: string;
  organizationStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
  isLeaf?: boolean;
}

export type organizations = Array<OrganizationRecord>;
export interface OrganizationRolePair{

  authorizedRoleIds:Array<string>;
  roleList:Array<RoleRecord>;

}
export interface OrganizationGroupPair{

  authorizedGroupIds:Array<string>;
  groupList:Array<RoleRecord>;

}
export interface OrganizationRoleRecord{
  userOrgRelId?:string;
  systemId?: string;
  organizationId?: string;
  organizationCode?: string;
  organizationName?: string;
  userId?: string;
  userCode?: string;
  userName?: string;
}

export interface OrganizationRoleSave{
  systemId?: string;
  organizationId?: string;
  lastModifiedUserId?: string;
  authorizedRoleIds:Array<string>;
}
export interface OrganizationGroupSave{
  systemId?: string;
  organizationId?: string;
  lastModifiedUserId?: string;
  authorizedGroupIds:Array<string>;
}
export const generateFormModel = (): OrganizationRecord => {
  return {
    systemId: '',
    organizationId: '',
    organizationCode: '',
    organizationName: '',
    organizationLevel: '',
    upOrganizationName: '',
    upOrganizationId: '',
    organizationStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
    isLeaf: false,
  };
};


