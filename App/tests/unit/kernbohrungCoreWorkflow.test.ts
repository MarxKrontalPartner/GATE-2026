import { describe, expect, it } from 'vitest';
import { coreCreationState } from '@/utils/kernbohrungCoreWorkflow';

describe('Kernbohrung Bohrkern creation preconditions', () => {
  it('distinguishes a new offline Kernbohrung from an unsaved form', () => {
    expect(coreCreationState(-1)).toBe('needs-sync');
    expect(coreCreationState(-23)).toBe('needs-sync');
    expect(coreCreationState(0)).toBe('missing-investigation');
    expect(coreCreationState(undefined)).toBe('missing-investigation');
    expect(coreCreationState('')).toBe('missing-investigation');
  });
  it('accepts a persisted positive server identifier', () => {
    expect(coreCreationState(4)).toBe('ready');
    expect(coreCreationState('27')).toBe('ready');
    expect(coreCreationState(2.5)).toBe('missing-investigation');
  });
});
