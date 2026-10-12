import { ref } from 'vue'
import { apiConnector } from '@/utils/ApiConnector'

export interface Probe {
  id?: number
  probenbezeichnung: string
  bemerkung: string | null
  material: number[]
  bohrkern?: null | number
  entnahmetiefeVon?: number | null
  entnahmetiefeBis?: number | null
  projekt_id?: number
  bauwerk_id?: number
  [key: string]: any
}
export interface Bohrkern {
  id?: number
  name: string
  anzahlProben?: number | null
  material?: number[]
  [key: string]: any
}


export function useProbe(id: number | null = null) {
  const probe = ref<Probe | null>(null)
  const relatedBohrkern = ref<Bohrkern | null>(null)
  const probenList = ref<Probe[]>([]);

  // Constructor-like initialization
  if (id) {
    loadProbe(id)
  }

  async function loadProbesOptions(projekt?: string, bauwerk?: string) {
    const response = await apiConnector.getXResourceList<Probe>('bwd', 'Probe', projekt, bauwerk)
    return response
  }

  async function refreshProbenList(projekt?: string, bauwerk?: string) {
    probenList.value = await loadProbesOptions(projekt, bauwerk);
  }

  async function loadProbe(probeId: number) {
    try {
      const response = await apiConnector.getXResource<Probe>('bwd', 'Probe', probeId)
      probe.value = response
      
      if (response.bohrkern) {
        try {
          // Fetch the related Bohrkern using the bohrkern ID
          relatedBohrkern.value = await apiConnector.getXResource<Bohrkern>('bwd', 'Bohrkern', response.bohrkern)
        } catch (error) {
          console.error('Failed to load related Bohrkern:', error)
          relatedBohrkern.value = null
        }
      } else {
        relatedBohrkern.value = null
      }
      return response
    } catch (error) {
      console.error('Failed to load Probe:', error)
      probe.value = null
      relatedBohrkern.value = null
      throw error
    }
  }

  return {
    probe,
    relatedBohrkern,
    probenList,
    loadProbesOptions,
    refreshProbenList,
    loadProbe,
  }
}
