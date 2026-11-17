import type { KernbohrungSchicht } from '@/types/kernbohrung';

let nextLocalLayerId = 0;

/** Suggest a start depth, but never copy or change any existing layer. */
export function appendKernbohrungLayer(layers: KernbohrungSchicht[]): KernbohrungSchicht[] {
  const last = layers.at(-1);
  const proposedStart = last?.tiefe_bis ?? (layers.length === 0 ? 0 : null);
  return [...layers, {
    id: `new-layer-${++nextLocalLayerId}`,
    bezeichnung: '',
    beschreibung: '',
    tiefe_von: proposedStart,
    tiefe_bis: null,
  }];
}

/** Deleting a layer does not silently rewrite measurements in other layers. */
export function removeKernbohrungLayer(layers: KernbohrungSchicht[], index: number): KernbohrungSchicht[] {
  return layers.filter((_, position) => position !== index);
}
