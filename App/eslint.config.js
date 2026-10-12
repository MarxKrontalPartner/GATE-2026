import pluginVue from 'eslint-plugin-vue';
import {
  defineConfigWithVueTs,
  vueTsConfigs,
} from '@vue/eslint-config-typescript';


export default defineConfigWithVueTs(
  {
    ignores: ['dist/**', 'src-tauri/**', 'node_modules/**'],
  },
  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommended,
  {
    extends: [
        {
            rules: {
                "@typescript-eslint/no-explicit-any": "off",
                "@typescript-eslint/no-unused-vars": "warn",
                "@typescript-eslint/no-empty-object-type": ["error", { allowInterfaces: 'with-single-extends' }],
            }
        }
    ],
  }
);
