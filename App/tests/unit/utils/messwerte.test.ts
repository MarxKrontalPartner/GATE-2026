import { describe, expect, test } from 'vitest';
import { maximum, median, toNumberOrNull } from '@/utils/messwerte';


describe('toNumberOrNull', () => {
  test('converts numeric input and maps empty or invalid input to null', () => {
    expect(toNumberOrNull('12.5')).toBe(12.5);
    expect(toNumberOrNull('0,3')).toBe(0.3);
    expect(toNumberOrNull(7)).toBe(7);
    expect(toNumberOrNull('')).toBeNull();
    expect(toNumberOrNull('  ')).toBeNull();
    expect(toNumberOrNull(null)).toBeNull();
    expect(toNumberOrNull(undefined)).toBeNull();
    expect(toNumberOrNull('abc')).toBeNull();
  });
});


describe('median', () => {
  test('uses the middle value of an odd count', () => {
    expect(median([40, 41, 39, 42, 40])).toBe(40);
  });

  test('averages the two middle values of an even count', () => {
    expect(median([40, 42, 38, 44])).toBe(41);
  });

  test('ignores empty values and returns null without values', () => {
    expect(median([null, 30, '', 32])).toBe(31);
    expect(median([])).toBeNull();
    expect(median(null)).toBeNull();
  });
});


describe('maximum', () => {
  test('returns the largest numeric value or null', () => {
    expect(maximum([0.2, null, 0.45, 0.1])).toBe(0.45);
    expect(maximum([null, undefined])).toBeNull();
    expect(maximum(undefined)).toBeNull();
  });
});
