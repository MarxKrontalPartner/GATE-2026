/**
 * Vitest shares the production Vite setup, but only runs Vitest tests.
 * tests/release uses Node's built-in test runner (node --test) and Python
 * unittest, so those suites must not be collected by Vitest.
 */
import { configDefaults, defineConfig, mergeConfig } from 'vitest/config';
import viteConfig from './vite.config';

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      exclude: [...configDefaults.exclude, '**/tests/release/**'],
    },
  }),
);
