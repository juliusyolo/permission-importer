export interface ModuleRecord {
  systemId?: string;
  moduleId?: string;
  moduleName?: string;
  moduleStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
}
export interface FunctionRecord {
  systemId?: string;
  moduleId?: string;
  functionId?: string;
  functionUrl?: string;
  functionName?: string;
  functionStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
}
export interface ResourceRecord {
  resourceId?: string;
  controlId?: string;
  resourceCode?: string;
  resourceName?: string;
  resourceStatus?: string;
  resourceUrl?:string;
  resourceMethodType?:string;
  resourceRemark?:string;
  resourceQueryFlag?:string;
}
export interface ControlPointRecord {
  functionId?: string;
  controlId?: string;
  controlCode?: string;
  controlName?: string;
  controlStatus?: string;
  lastModifiedUserId?: string;
  resources?:Array<ResourceRecord>;
  updateTime?: string;
}

export const generateFunctionFormModel = (): FunctionRecord => {
  return {
    systemId: '',
    moduleId: '',
    functionId: '',
    functionUrl: '',
    functionName: '',
    functionStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};

export const generateModuleFormModel = (): ModuleRecord => {
  return {
    systemId: '',
    moduleId: '',
    moduleName: '',
    moduleStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};

export const generateControlPointFormModel = (): ControlPointRecord => {
  return {
    functionId: '',
    controlId: '',
    controlCode: '',
    controlName: '',
    controlStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};
