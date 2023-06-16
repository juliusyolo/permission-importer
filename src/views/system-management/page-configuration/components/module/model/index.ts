export interface ModuleRecord {
  systemId?: string;
  moduleId?: string;
  moduleName?: string;
  moduleRemark?: string;
  moduleStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
}
export interface FunctionRecord {
  systemId?: string;
  moduleId?: string;
  moduleName?: string;
  functionId?: string;
  functionUrl?: string;
  functionName?: string;
  functionStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
}

export interface ClipRule{
  type?:string;
  keyword?:string;
  regex?:string;
  replacement?:string;
}

export interface ResourceRecord {
  resourceId?: string;
  controlId?: string;
  resourceCode?: string;
  resourceName?: string;
  resourceStatus?: string;
  resourceUrl?: string;
  resourceMethodType?: string;
  resourceRemark?: string;
  resourceQueryFlag?: string;
  resourceClipRules?: Array<ClipRule>;
  lastModifiedUserId?: string;
  authorizations?: Array<string>;
}

export interface ControlPointRecord {
  systemId?:string;
  functionId?: string;
  controlId?: string;
  controlCode?: string;
  controlName?: string;
  controlStatus?: string;
  lastModifiedUserId?: string;
  resources?: Array<ResourceRecord>;
  updateTime?: string;
}

export const generateClipRuleFormModel = ():ClipRule =>{
  return {
    type:'',
    keyword:'',
    regex:'',
    replacement:'',
  }
}

export const generateFunctionFormModel = (): FunctionRecord => {
  return {
    systemId: '',
    moduleId: '',
    moduleName:'',
    functionId: '',
    functionUrl: '',
    functionName: '',
    functionStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};
export const generateResourceFormModel = (): ResourceRecord => {
  return {
    resourceId: '',
    controlId: '',
    resourceCode: '',
    resourceName: '',
    resourceStatus: '',
    resourceUrl: '',
    resourceMethodType: '',
    resourceRemark: '',
    resourceQueryFlag: '',
    resourceClipRules: [],
    lastModifiedUserId: '',
    authorizations:[]
  };
};
export const generateModuleFormModel = (): ModuleRecord => {
  return {
    systemId: '',
    moduleId: '',
    moduleName: '',
    moduleStatus: '',
    moduleRemark:'',
    lastModifiedUserId: '',
    updateTime: '',
  };
};

export const generateControlPointFormModel = (): ControlPointRecord => {
  return {
    systemId: '',
    functionId: '',
    controlId: '',
    controlCode: '',
    controlName: '',
    controlStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
  };
};

