import type { Component } from 'vue'

import fallback from '@/views/bwd/Untersuchungsverfahren/fallbackView.vue'
import ChloridanalyseView from '@/views/bwd/Untersuchungsverfahren/VorOrtUntersuchung/chloridanalyseView.vue'
import DruckfestigkeitspruefungView from '@/views/bwd/Untersuchungsverfahren/VorOrtUntersuchung/DruckfestigkeitspruefungView.vue'
import KernbohrungView from '@/views/bwd/Untersuchungsverfahren/VorOrtUntersuchung/kernbohrungView.vue'
import RissbreitenmessungView from '@/views/bwd/Untersuchungsverfahren/VorOrtUntersuchung/RissbreitenmessungView.vue'
import RueckprallhammerView from '@/views/bwd/Untersuchungsverfahren/VorOrtUntersuchung/rueckprallhammerView.vue'
import { normalizeMethodKey } from '@/utils/UntersuchungsverfahrenRegistry'

const detailComponents: Record<string, Component> = {
  kernbohrung: KernbohrungView,
  chloridanalyse: ChloridanalyseView,
  druckfestigkeitspruefung: DruckfestigkeitspruefungView,
  rueckprallhammer: RueckprallhammerView,
  rissbreitenmessung: RissbreitenmessungView,
}

export function getUntersuchungsverfahrenComponent(methodName: string): Component {
  return detailComponents[normalizeMethodKey(methodName)] ?? fallback
}
