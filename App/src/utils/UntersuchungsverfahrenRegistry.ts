export type UntersuchungsverfahrenArt = 'VorOrtUntersuchung' | 'Laboruntersuchung'

export interface UntersuchungsverfahrenMetadata {
  slug: string
  apiName: string
  displayName: string
  art: UntersuchungsverfahrenArt
}

const methodDefinitions: UntersuchungsverfahrenMetadata[] = [
  { slug: 'kernbohrung', apiName: 'Kernbohrung', displayName: 'Kernbohrung', art: 'VorOrtUntersuchung' },
  { slug: 'chloridanalyse', apiName: 'Chloridanalyse', displayName: 'Chloridanalyse', art: 'VorOrtUntersuchung' },
  { slug: 'druckfestigkeitspruefung', apiName: 'Druckfestigkeitspruefung', displayName: 'Druckfestigkeitsprüfung', art: 'Laboruntersuchung' },
  { slug: 'rueckprallhammer', apiName: 'Rueckprallhammer', displayName: 'Rückprallhammer', art: 'VorOrtUntersuchung' },
  { slug: 'rissbreitenmessung', apiName: 'Rissbreitenmessung', displayName: 'Rissbreitenmessung', art: 'VorOrtUntersuchung' },
]

const methodsBySlug = Object.fromEntries(methodDefinitions.map((method) => [method.slug, method])) as Record<string, UntersuchungsverfahrenMetadata>
const methodsByApiName = Object.fromEntries(methodDefinitions.map((method) => [method.apiName.toLowerCase(), method])) as Record<string, UntersuchungsverfahrenMetadata>

export function normalizeMethodKey(methodName: string): string {
  const normalized = methodName.trim().toLowerCase()
  return methodsBySlug[normalized]?.slug ?? methodsByApiName[normalized]?.slug ?? normalized
}

export function getMethodMetadata(methodName: string): UntersuchungsverfahrenMetadata | undefined {
  return methodsBySlug[normalizeMethodKey(methodName)]
}

export function getMethodRouteSlug(methodName: string): string {
  return getMethodMetadata(methodName)?.slug ?? normalizeMethodKey(methodName)
}

export function getMethodApiName(methodName: string): string {
  return getMethodMetadata(methodName)?.apiName ?? methodName
}

export function getMethodDisplayName(methodName: string): string {
  return getMethodMetadata(methodName)?.displayName ?? methodName
}

export function getMethodArt(methodName: string): UntersuchungsverfahrenArt | undefined {
  return getMethodMetadata(methodName)?.art
}

export function getAllKnownMethods(): UntersuchungsverfahrenMetadata[] {
  return methodDefinitions
}
