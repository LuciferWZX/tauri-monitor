import { defineConfig } from "umi";
import routes from "./config/route";

export default defineConfig({
  routes:routes,
  npmClient: 'pnpm',
  plugins: [
    '@umijs/plugins/dist/initial-state',
    '@umijs/plugins/dist/model',
    '@umijs/plugins/dist/styled-components',
    '@umijs/plugins/dist/valtio',
  ],
  history:{
    type:"hash"
  },
  hash:true,
  alias:{
    '@':'src',
  },
  model:{},
  initialState: {},
  styledComponents:{},
  valtio:{},
});
