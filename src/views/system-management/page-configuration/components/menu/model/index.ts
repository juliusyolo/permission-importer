export interface MenuRecord {
  systemId?: string;
  functionId?: string;
  functionName?: string;
  menuId?: string;
  menuIcon?: string;
  menuName?: string;
  menuOrder?: number;
  upMenuName?: string;
  upMenuId?: string;
  menuStatus?: string;
  lastModifiedUserId?: string;
  updateTime?: string;
  i18nKey?: string;
  menuComponent?: string;
  menuPath?: string;
  menuRemark?: string;
  isLeaf?: boolean;
}

export type menus = Array<MenuRecord>;

export const generateFormModel = (): MenuRecord => {
  return {
    systemId: '',
    functionId: '',
    functionName: '',
    menuId: '',
    menuIcon: '',
    menuName: '',
    menuOrder: undefined,
    upMenuName: '',
    upMenuId: '',
    menuStatus: '',
    lastModifiedUserId: '',
    updateTime: '',
    i18nKey: '',
    menuComponent: '',
    menuPath: '',
    menuRemark: '',
    isLeaf: false,
  };
};
