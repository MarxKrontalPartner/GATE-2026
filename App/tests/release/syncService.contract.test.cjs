/**
 * Dependency-light regression tests that execute the actual src/services/syncService.ts
 * after TypeScript transpilation. No Tauri runtime, Django server or SQLite data
 * is modified. Run from the repository root:
 *   node --test App/tests/release/syncService.contract.test.cjs
 *
 * Requires the normal project's TypeScript dev dependency (npm install), or a
 * globally installed TypeScript compiler.
 */
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { execFileSync } = require('node:child_process');

function typescript() {
  try { return require('typescript'); } catch {}
  const root = execFileSync('npm', ['root', '-g'], { encoding: 'utf8' }).trim();
  return require(path.join(root, 'typescript'));
}
const ts = typescript();
const source = path.join(__dirname, '../../src/services/syncService.ts');

function subject({ creates = [], pending = [], serverResponse, networkError = false } = {}) {
  const actions = [];
  const requests = [];
  let connectionFailures = 0;
  const invoke = async (command, args) => {
    actions.push({ command, args });
    if (command === 'sync_take_creates') return {
      endpoint: `bwd/${args.resource}/`, localIds: creates.map(r => r.localId),
      records: creates.map(r => r.data),
    };
    if (command === 'sync_take_pending') return {
      endpoint: `bwd/${args.resource}/sync/`, body: pending,
    };
    return undefined;
  };
  const fetch = async (url, opts) => {
    requests.push({ url, opts });
    if (networkError) throw Error('network disconnected');
    return serverResponse ?? { status: 201, json: async () => [{ id: 100 }], text: async () => '' };
  };
  const exports = {};
  const code = ts.transpileModule(fs.readFileSync(source, 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
    fileName: source,
  }).outputText;
  const requireMock = (moduleName) => {
    if (moduleName === '@tauri-apps/api/core') return { invoke, isTauri: () => true };
    if (moduleName === '@/config/apiBase') return { API_BASE_URL: 'http://example.test/api' };
    if (moduleName === '@/services/connectivity') return { markServerUnreachable: () => { connectionFailures++; } };
    throw Error(`Unexpected dependency ${moduleName}`);
  };
  vm.runInNewContext(code, { exports, require: requireMock, window: { fetch } }, { filename: source });
  return {
    api: exports, actions, requests,
    getConnectionFailures: () => connectionFailures,
  };
}

const create = [{ localId: -7, data: { projekt: 1, bauwerk: 1, untersuchungsstelle: 1, datum: '2026-11-27', material: [2] } }];

test('HTTP 400 Invalid material pk remains failed and is not acknowledged', async () => {
  const runtime = subject({
    creates: create,
    serverResponse: { status: 400, text: async () => '{"material":["Invalid pk 2"]}' },
  });
  const result = await runtime.api.syncResource('Kernbohrung');
  assert.equal(result.failed, true);
  assert.equal(result.created, 0);
  const applied = runtime.actions.find(a => a.command === 'sync_apply_creates');
  assert.equal(applied.args.status, 'FAILED');
  assert.match(applied.args.error, /Invalid pk 2/);
  assert.equal(runtime.actions.some(a => a.command === 'sync_take_pending'), false, 'Do not patch child data after create failed');
});

test('successful create acknowledges server-generated ID, never local negative ID', async () => {
  const runtime = subject({
    creates: create,
    serverResponse: { status: 201, json: async () => [{ id: 41, version: 1 }] },
  });
  const result = await runtime.api.syncResource('Kernbohrung');
  assert.equal(result.failed, false);
  assert.equal(result.created, 1);
  const applied = runtime.actions.find(a => a.command === 'sync_apply_creates');
  assert.equal(applied.args.status, 'OK');
  assert.deepEqual(Array.from(applied.args.localIds), [-7]);
  assert.equal(applied.args.created[0].id, 41);
});

test('network failure leaves offline create pending and flags disconnected server', async () => {
  const runtime = subject({ creates: create, networkError: true });
  const result = await runtime.api.syncResource('Kernbohrung');
  assert.equal(result.failed, true);
  assert.equal(runtime.getConnectionFailures(), 1);
  assert.equal(runtime.actions.find(a => a.command === 'sync_apply_creates').args.status, 'FAILED');
});

test('201 with incorrect created-record count must NOT be marked successful', async () => {
  const runtime = subject({
    creates: create,
    serverResponse: { status: 201, json: async () => [] },
  });
  const result = await runtime.api.syncResource('Kernbohrung');
  assert.equal(result.failed, true, 'A partial/malformed server response must remain pending');
  assert.equal(result.created, 0);
  assert.equal(runtime.actions.find(a => a.command === 'sync_apply_creates').args.status, 'FAILED');
});

test('201 with invalid JSON does not acknowledge create', async () => {
  const runtime = subject({
    creates: create,
    serverResponse: { status: 201, json: async () => { throw Error('invalid JSON'); } },
  });
  const result = await runtime.api.syncResource('Kernbohrung');
  assert.equal(result.failed, true);
  assert.equal(runtime.actions.find(a => a.command === 'sync_apply_creates').args.status, 'FAILED');
});

test('201 with an invalid server ID does not acknowledge create', async () => {
  const runtime = subject({
    creates: create,
    serverResponse: { status: 201, json: async () => [{ id: null }] },
  });
  const result = await runtime.api.syncResource('Kernbohrung');
  assert.equal(result.failed, true);
  assert.equal(runtime.actions.find(a => a.command === 'sync_apply_creates').args.status, 'FAILED');
});

test('Material is not an automatically synced resource (release limitation)', () => {
  const runtime = subject();
  assert.equal(runtime.api.SYNCED_RESOURCES.includes('Material'), false);
  assert.equal(runtime.api.SYNCED_RESOURCES.includes('Kernbohrung'), true);
});
