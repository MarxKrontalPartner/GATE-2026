/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_VARS_API_BASE_URL: string;
  }
  
  interface ImportMeta {
    readonly env: ImportMetaEnv;
  }

  // Vue module shim (ensure .vue files resolve for TS)
  declare module '*.vue' {
    import { DefineComponent } from 'vue';
    // Generic shim for .vue files
    const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, any>;
    export default component;
  }
