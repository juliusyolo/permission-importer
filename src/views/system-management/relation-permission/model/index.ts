import { MenuRecord } from '@/views/system-management/page-configuration/components/menu/model';

export interface TreeRecord {
  title?: string;
  key?: string;
  type?: string;
  origin?: MenuRecord;
  children?: Array<TreeRecord>;
}


export interface ResourceRecord{
  resourceType?:string;
  resourceKey?:string;
}
export interface AuthorizationRawRecord{
  authorizationType?:string;
  authorizationKey?:string;
}

export interface AuthorizedPair{
  authorizedHalfKeys:Array<string>;
  authorizedCompleteKeys:Array<string>;
  authorizedHalfRecords:Array<TreeRecord>;
  authorizedCompleteRecords:Array<TreeRecord>;
}

export interface AuthorizationRecord{
  systemId?:string;
  lastModifiedUserId?:string;
  authorizationType?:string;
  authorizationKey?:string;
  completeAuthorizationResources?:Array<ResourceRecord>;
  halfAuthorizationResources?:Array<ResourceRecord>;
}

