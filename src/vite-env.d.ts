/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

// 路径别名类型声明
declare module "*/" {
  const content: any;
  export default content;
}
