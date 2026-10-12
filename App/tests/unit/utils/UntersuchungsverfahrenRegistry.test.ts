import { expect, test, describe } from 'vitest';
import {
  getAllKnownMethods,
  getMethodApiName,
  getMethodArt,
  getMethodDisplayName,
  normalizeMethodKey,
} from '@/utils/UntersuchungsverfahrenRegistry';


describe('normalizeMethodKey', () => {
  test('maps api names and list keys to the route slug', () => {
    expect(normalizeMethodKey('Kernbohrung')).toBe('kernbohrung');
    expect(normalizeMethodKey('druckfestigkeitspruefung')).toBe('druckfestigkeitspruefung');
    expect(normalizeMethodKey('  Rueckprallhammer ')).toBe('rueckprallhammer');
  });

  test('returns unknown methods lowercased', () => {
    expect(normalizeMethodKey('Spiralbohrung')).toBe('spiralbohrung');
  });
});


describe('getMethodApiName', () => {
  test('returns the backend class name for a slug', () => {
    expect(getMethodApiName('kernbohrung')).toBe('Kernbohrung');
    expect(getMethodApiName('rissbreitenmessung')).toBe('Rissbreitenmessung');
    expect(getMethodApiName('druckfestigkeitspruefung')).toBe('Druckfestigkeitspruefung');
  });

  test('passes unknown methods through unchanged', () => {
    expect(getMethodApiName('Unbekannt')).toBe('Unbekannt');
  });
});


describe('registry metadata', () => {
  test('contains exactly the five supported methods', () => {
    expect(getAllKnownMethods().map((method) => method.slug)).toEqual([
      'kernbohrung',
      'chloridanalyse',
      'druckfestigkeitspruefung',
      'rueckprallhammer',
      'rissbreitenmessung',
    ]);
  });

  test('resolves art and display name', () => {
    expect(getMethodArt('Druckfestigkeitspruefung')).toBe('Laboruntersuchung');
    expect(getMethodArt('chloridanalyse')).toBe('VorOrtUntersuchung');
    expect(getMethodDisplayName('rueckprallhammer')).toBe('Rückprallhammer');
    expect(getMethodArt('Rissbreitenmessung')).toBe('VorOrtUntersuchung');
    expect(getMethodDisplayName('rissbreitenmessung')).toBe('Rissbreitenmessung');
  });
});
