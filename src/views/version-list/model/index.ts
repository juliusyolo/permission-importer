export interface VersionControl {
  id?: number;
  version?: string;
  releaseTime?: string;
  createTime?: string;
  updateTime?: string
}

export interface PaginationResult<T> {
  total: number,
  current_page: number,
  page_size: number,
  data: Array<T>,
}


export type VersionControlPaginationResult = PaginationResult<VersionControl>;

