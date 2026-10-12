/** Converts form input to a finite number, or null for empty/invalid input. */
export function toNumberOrNull(value: unknown): number | null {
  if (value === null || value === undefined) return null
  if (typeof value === 'string' && value.trim() === '') return null
  const parsed = typeof value === 'number' ? value : Number(String(value).replace(',', '.'))
  return Number.isFinite(parsed) ? parsed : null
}

function finiteValues(values: readonly unknown[] | null | undefined): number[] {
  return (values ?? []).map(toNumberOrNull).filter((value): value is number => value !== null)
}

/** Median of all numeric values, null if there are none. */
export function median(values: readonly unknown[] | null | undefined): number | null {
  const sorted = finiteValues(values).sort((a, b) => a - b)
  if (sorted.length === 0) return null
  const middle = Math.floor(sorted.length / 2)
  return sorted.length % 2 === 1 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2
}

/** Maximum of all numeric values, null if there are none. */
export function maximum(values: readonly unknown[] | null | undefined): number | null {
  const numbers = finiteValues(values)
  return numbers.length === 0 ? null : Math.max(...numbers)
}
