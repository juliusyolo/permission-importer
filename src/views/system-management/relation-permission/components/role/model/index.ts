export interface RoleRecord {
  systemId?:string;
  roleId?:string;
  roleCode?:string;
  roleName?:string;
  roleStatus?:string;
  lastModifiedUserId?:string;
  updateTime?:string;
}
export type role = {
  roleId: string;
  roleCode: string;
  roleName: string;
  roleStatus: string;
  lastModifiedUserId: string;
  updateTime: string;
};

export type roles = Array<RoleRecord>;
export interface RoleUserRecord {
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
export const generateFormModel = (): RoleRecord => {
  return {
    roleId: '',
    roleCode: '',
    roleName: '',
    roleStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};
