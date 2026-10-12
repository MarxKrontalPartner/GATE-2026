// stores/UntersuchungenStore.ts
import { acceptHMRUpdate, defineStore } from 'pinia'
import { apiConnector } from '@/utils/ApiConnector';
import { normalizeMethodKey } from '@/utils/UntersuchungsverfahrenRegistry';



/**
 * Minimale Infos für die Übersicht pro Untersuchungseintrag.
 * Du kannst dieses Interface bei Bedarf anpassen/erweitern.
 */
export interface UntersuchungListItem {
  id: number
  name: string
  bauwerk: number | null
  projekt: number | null
  titelBild: string | number
  untersuchungsstelle: number | null
  type: string //todo enum
}

/**
 * Detaillierte Infos für eine einzelne Untersuchung.
 * Momentan als "any" getypt, da sich die Felder je nach Untersuchungs-Typ
 * stark unterscheiden können. 
 * TODO
 */
export interface UntersuchungDetail {
  type: string
  id: number
  projekt: number | string | null
  bauwerk: number | string | null
  untersuchungsstelle: number | null
  bilder: number[] | [],
  titelBild: string | number | null
  [key: string]: any

}




export const useUntersuchungenStore = defineStore('untersuchungen', {
  state: () => ({
    vorOrtUntersuchungen: [] as UntersuchungListItem[],

    laborUntersuchungen: [] as UntersuchungListItem[],

    /**
     * Detaildaten der aktuell ausgewählten Untersuchung.
     * (Reaktiv, wird angezeigt, wenn man auf Details geht)
     */
    currentUntersuchung: {} as UntersuchungDetail ,
    
    /**
     * Backup of the current investigation for undo functionality
     */
    currentUntersuchungBackup: {} as UntersuchungDetail,
    currentUntersuchungRequestId: 0,
    
    doSaveCurrentUntersuchung: false,
    }),


    actions: {
    async fetchVorOrtUntersuchung(bauwerk?: string, projekt?: string, selectedUntersuchungsverfahrenOptions?: string[]) {
      try {
        const data = await apiConnector.BWD.fetchVorOrtUntersuchungData(bauwerk, projekt, selectedUntersuchungsverfahrenOptions);
        const newVorOrtItems: UntersuchungListItem[] = [];

        if (data.VorOrtUntersuchung) {
          for (const untersuchungTyp in data.VorOrtUntersuchung) {
            const items = data.VorOrtUntersuchung[untersuchungTyp];
            for (const item of items) {
              newVorOrtItems.push({
                id: item.id,
                name: item.name ?? '',
                bauwerk: item.bauwerk ?? null,
                projekt: item.projekt ?? null,
                titelBild: item.titelBild ?? '',
                untersuchungsstelle: item.untersuchungsstelle ?? null,
                type: normalizeMethodKey(untersuchungTyp),
              });
            }
          }
        }

        this.vorOrtUntersuchungen = newVorOrtItems;
      } catch (error) {
        console.error('fetchVorOrtUntersuchung error:', error);
        throw error;
      }
    },
    
    async fetchLaboruntersuchung(bauwerk?: string, projekt?: string, selectedUntersuchungsverfahrenOptions?: string[]) {
      try {
        const data = await apiConnector.BWD.fetchLaboruntersuchungData(bauwerk, projekt, selectedUntersuchungsverfahrenOptions);
        const newLaboruntersuchungItems: UntersuchungListItem[] = [];

        if (data.Laboruntersuchung) {
          for (const untersuchungTyp in data.Laboruntersuchung) {
            const items = data.Laboruntersuchung[untersuchungTyp];
            for (const item of items) {
              newLaboruntersuchungItems.push({
                id: item.id,
                name: item.name ?? '',
                bauwerk: item.bauwerk ?? null,
                projekt: item.projekt ?? null,
                titelBild: item.titelBild ?? '',
                untersuchungsstelle: item.untersuchungsstelle ?? null,
                type: normalizeMethodKey(untersuchungTyp),
              });
            }
          }
        }

        this.laborUntersuchungen = newLaboruntersuchungItems;
      } catch (error) {
        console.error('fetchLaboruntersuchung error:', error);
        throw error;
      }
    },

    clearCurrentUntersuchung() {
      this.currentUntersuchung = {} as UntersuchungDetail;
      this.currentUntersuchungBackup = {} as UntersuchungDetail;
    },

    /**
    * Lädt die Detaildaten einer bestimmten Untersuchung.
     * und speichert sie in currentUntersuchung.
     * @param id            ID der Untersuchung
    * @param art           'VorOrtUntersuchung' oder 'Laboruntersuchung'
     * @param untersuchung  z.B. 'kernbohrung', 'rueckprallhammer', etc.
     */
  async fetchUntersuchungDetail(id: number, art: string | undefined, untersuchung: string) {
      try {
        const requestId = ++this.currentUntersuchungRequestId;
        this.currentUntersuchung = {} as UntersuchungDetail;
        this.currentUntersuchungBackup = {} as UntersuchungDetail;

    const data = await apiConnector.BWD.fetchUntersuchungDetailData(id, untersuchung, art);
        if (requestId !== this.currentUntersuchungRequestId) {
          return;
        }

        data.type = normalizeMethodKey(untersuchung);
        this.currentUntersuchung = data;
        // Clear backup when loading new data
        this.currentUntersuchungBackup = {} as UntersuchungDetail;
      } catch (error) {
        console.error('fetchUntersuchungDetail error:', error);
        throw error;
      }
    },

    /**
     * Erstellt eine neue Untersuchung (POST).
     * Anschließend wird sie direkt in der jeweiligen Liste gespeichert.
    * @param art           'VorOrtUntersuchung' oder 'Laboruntersuchung'
     * @param untersuchung  z.B. 'kernbohrung'
     * @param payload       Body-Daten für den POST
     */
      async createUntersuchung(art: 'VorOrtUntersuchung' | 'Laboruntersuchung', untersuchung: string, payload: any) {
      try {
        const createdData = await apiConnector.BWD.createUntersuchung(untersuchung, payload);

        const newItem: UntersuchungListItem = {
          id: createdData.id,
          name: createdData.name ?? '',
          bauwerk: createdData.bauwerk ?? null,
          projekt: createdData.projekt ?? null,
          titelBild: createdData.titelBild ?? '',
          untersuchungsstelle: createdData.untersuchungsstelle ?? null,
          type: normalizeMethodKey(untersuchung),
        };

        if (art === 'VorOrtUntersuchung') {
          this.vorOrtUntersuchungen.push(newItem);
        } else if (art === 'Laboruntersuchung') {
          this.laborUntersuchungen.push(newItem);
        }

        return createdData;
      } catch (error) {
        console.error('createUntersuchung error:', error);
        throw error;
      }
    },

    /**
     * Aktualisiert eine bestehende Untersuchung (PATCH).
     * Anschließend wird sie in der jeweiligen Liste aktualisiert.
     * @param id            ID der Untersuchung
     * @param untersuchung  z.B. 'kernbohrung'
     * @param payload       Body-Daten für das Update
     */
    async updateUntersuchung(id: number, untersuchung: string, payload: any) {
      try {
        this.doSaveCurrentUntersuchung = true;
        const updatedData = await apiConnector.BWD.updateUntersuchung(id, untersuchung, payload);
        this.currentUntersuchung = updatedData;
        if (this.currentUntersuchung) {
          this.currentUntersuchung.type = normalizeMethodKey(untersuchung);
        }
      } catch (error) {
        console.error('updateUntersuchung error:', error);
        throw error;
      } finally {
        this.doSaveCurrentUntersuchung = false;
      }
    },

    /**
     * Löscht eine bestehende Untersuchung (DELETE).
     * Entfernt den Eintrag aus der jeweiligen Liste.
     * @param id            ID der Untersuchung
     * @param art           'VorOrtUntersuchung' oder 'Laboruntersuchung'
     * @param untersuchung  z.B. 'kernbohrung'
     */
    async deleteUntersuchung(id: number, art: 'VorOrtUntersuchung' | 'Laboruntersuchung', untersuchung: string) {
      try {
        await apiConnector.BWD.deleteUntersuchungData(id, untersuchung);

        if (art === 'VorOrtUntersuchung') {
          this.vorOrtUntersuchungen = this.vorOrtUntersuchungen.filter((item) => item.id !== id);
        } else if (art === 'Laboruntersuchung') {
          this.laborUntersuchungen = this.laborUntersuchungen.filter((item) => item.id !== id);
        }
      } catch (error) {
        console.error('deleteUntersuchung error:', error);
        throw error;
      }
    },
    // updates the current Untersuchung
    async saveCurrentUntersuchung(): Promise<void> {
      if (this.currentUntersuchung) {
        const id = this.currentUntersuchung.id;
        const untersuchung = this.currentUntersuchung.type;
        this.doSaveCurrentUntersuchung = true;
        return await this.updateUntersuchung(id, untersuchung, this.currentUntersuchung).then(() => {
          this.doSaveCurrentUntersuchung = false;
          // Clear backup after successful save
          this.clearCurrentUntersuchungBackup();
        });
      }
    },

    /**
     * Creates a backup of the current investigation state.
     * Call this when entering edit mode.
     */
    createCurrentUntersuchungBackup(): void {
      if (this.currentUntersuchung && Object.keys(this.currentUntersuchung).length > 0) {
        // Deep copy the current investigation to backup
        this.currentUntersuchungBackup = JSON.parse(JSON.stringify(this.currentUntersuchung));
      }
    },

    /**
     * Restores the current investigation from backup.
     * Call this when clicking undo.
     */
    restoreCurrentUntersuchungFromBackup(): void {
      if (this.currentUntersuchungBackup && Object.keys(this.currentUntersuchungBackup).length > 0) {
        // Deep copy the backup back to current
        this.currentUntersuchung = JSON.parse(JSON.stringify(this.currentUntersuchungBackup));
      }
    },

    /**
     * Clears the backup. Call this after successful save.
     */
    clearCurrentUntersuchungBackup(): void {
      this.currentUntersuchungBackup = {} as UntersuchungDetail;
    },

    //  fk felder aufloesen
    async fetchMaterial() {
      try {
        return await apiConnector.BWD.fetchMaterialData();
      } catch (error) {
        console.error('fetchMaterial error:', error);
        throw error;
      }
    },

    async fetchBearbeiter() {
      try {
        return await apiConnector.BWD.fetchBearbeiterData();
      } catch (error) {
        console.error('fetchBearbeiter error:', error);
        throw error;
      }
    },

    // fetchBohrkern
    async fetchBohrkern(id: number) {
      try {
        return await apiConnector.BWD.fetchBohrkernData(id);
      } catch (error) {
        console.error('fetchBohrkern error:', error);
        throw error;
      }
    },

    // createBohrkern
    async createBohrkern(name: string) {
      try {
        const payload = {
          name: name,
        };
        
        return await apiConnector.postxResource('bwd', 'Bohrkern', payload);
      } catch (error) {
        console.error('createBohrkern error:', error);
        throw error;
      }
    },

    // probe
    async fetchProbe(id: number) {
      try {
        return await apiConnector.BWD.fetchProbeData(id);
      } catch (error) {
        console.error('fetchProbe error:', error);
        throw error;
      }
    },
}
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useUntersuchungenStore, import.meta.hot))
}
