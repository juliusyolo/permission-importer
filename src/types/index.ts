export interface AnyObject {
  [key: string]: unknown;
}

export interface Options {
  value: unknown;
  label: string;
}

export interface NodeOptions extends Options {
  children?: NodeOptions[];
}

export interface GetParams {
  body: null;
  type: string;
  url: string;
}

export interface PostData {
  body: string;
  type: string;
  url: string;
}

export interface Pagination {
  current: number;
  pageSize: number;
  total?: number;
}

export type TimeRanger = [string, string];

export interface GeneralChart {
  xAxis: string[];
  data: Array<{ name: string; value: number[] }>;
}

export interface PaginationResult<T> {
  total: number;
  totalPage: number;
  pageSize: number;
  currentPage: number;
  size: number;
  dataList: Array<T>;
}
export interface PaginationQuery<T> {
  condition: T;
  currentPage: number;
  pageSize: number;
}

export interface Done {
  (closed?: boolean): void;
}

export interface GUIDGenerator {
  (args?: any): string;
}

export interface SystemInfo{
  systemId: string;
  systemName: string;
}
