import { test } from 'node:test';
import assert from 'node:assert/strict';
import { coreCreationState } from '../../src/utils/kernbohrungCoreWorkflow.ts';

test('offline-created Kernbohrung must be synchronized before server Bohrkern creation', () => {
  assert.equal(coreCreationState(-1), 'needs-sync');
  assert.equal(coreCreationState(-27), 'needs-sync');
});
test('a positive server ID permits a Bohrkern', () => {
  assert.equal(coreCreationState(1), 'ready');
  assert.equal(coreCreationState('45'), 'ready');
});
test('missing or malformed IDs must not trigger Bohrkern creation', () => {
  for(const id of [null,undefined,'',0,NaN,1.5,'hello'])
    assert.equal(coreCreationState(id),'missing-investigation');
});
