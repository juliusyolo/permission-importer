import {Message} from "@arco-design/web-vue";
import {invoke} from "@tauri-apps/api/tauri";

export const remoteResourceCall = <T, D>(
  controlCode: string,
  resourceCode: string,
  data: T
): Promise<any> => new Promise((resolve, reject) => {
  console.log("resourceCode:",resourceCode,"data:",data)
  invoke(resourceCode, {
    ...data
  }).then((result) => {
    console.log(result)
    resolve({data: result})
  }).catch((errorMsg) => {
    Message.error({content: errorMsg, duration: 1000})
    reject(errorMsg)
  })
})

export const statusGetter = () => {
  return [
    {
      label: '生效',
      value: '1',
    },
    {
      label: '失效',
      value: '0',
    },
  ];
};

export const yesOrNoGetter = () => {
  return [
    {
      label: '是',
      value: '1',
    },
    {
      label: '否',
      value: '0',
    },
  ];
};

export const methodGetter = () => {
  return [
    {
      label: 'GET',
      value: 'GET',
    },
    {
      label: 'POST',
      value: 'POST',
    },
  ];
};

export const dataAuthorizationGetter = () => {
  return [
    {
      label: '本机构',
      value: '1',
    },
    {
      label: '本机构及下级机构',
      value: '2',
    },
    {
      label: '本人及本机构',
      value: '3',
    },
  ];
};

export const clipRuleGetter = () => {
  return [
    {
      label: '客户姓名',
      value: '1',
    },
    {
      label: '银行卡号',
      value: '2',
    },
    {
      label: '身份证件',
      value: '3',
    },
    {
      label: '电子邮件',
      value: '4',
    },
    {
      label: '电话手机c',
      value: '5',
    },
    {
      label: '电话手机',
      value: '6',
    },
  ];
};

export const iconGetter = () => {
  return [
    {
      label: 'icon-dashboard',
      value: 'icon-dashboard',
    },
    {
      label: 'icon-branch',
      value: 'icon-branch',
    },
    {
      label: 'icon-drag-dot',
      value: 'icon-drag-dot',
    },
    {
      label: 'icon-drag-dot-vertical',
      value: 'icon-drag-dot-vertical',
    },
    {
      label: 'icon-settings',
      value: 'icon-settings',
    },
    {
      label: 'icon-computer',
      value: 'icon-computer',
    },
    {
      label: 'icon-bar-chart',
      value: 'icon-bar-chart',
    },
    {
      label: 'icon-bookmark',
      value: 'icon-bookmark',
    },
    {
      label: 'icon-desktop',
      value: 'icon-desktop',
    },
  ];
};
export const paramWrapper = (arg0: object) => {
  const obj = {};
  Object.keys(arg0).forEach((key: string) => {
    // @ts-ignore
    obj[key] = arg0[key] || undefined;
  });
  return obj;
};

