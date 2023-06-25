import {PaginationResult} from '../../../types';
export interface VersionControl {
  id?: number;
  version?: string;
  releaseTime?: string;
  createTime?: string;
  updateTime?: string
}

export type VersionControlPaginationResult = PaginationResult<VersionControl>;

