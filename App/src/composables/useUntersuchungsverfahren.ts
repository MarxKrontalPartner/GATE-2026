import { ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useUntersuchungenStore } from '@/store/bwd/UntersuchungsverfahrenStore';

export function useUntersuchungsverfahren(art: 'VorOrtUntersuchung' | 'Laboruntersuchung') {
  const store = useUntersuchungenStore();
  const { currentUntersuchung } = storeToRefs(store);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Fetch detail and populate store.currentUntersuchung
  async function fetchDetail(id: number, untersuchung: string) {
    loading.value = true;
    error.value = null;
    try {
      await store.fetchUntersuchungDetail(id, art, untersuchung);
    } catch (err: any) {
      error.value = err?.message || 'Fehler beim Laden der Daten';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  // Create a new entry
  async function create(untersuchung: string, payload: any) {
    loading.value = true;
    error.value = null;
    try {
      const created = await store.createUntersuchung(art, untersuchung, payload);
      return created;
    } catch (err: any) {
      error.value = err?.message || 'Fehler beim Erstellen';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  // Update existing entry
  async function update(untersuchung: string, payload: any) {
    if (!currentUntersuchung.value?.id) {
      throw new Error('Keine aktuelle Untersuchung ausgewählt');
    }
    loading.value = true;
    error.value = null;
    try {
      await store.updateUntersuchung(
        currentUntersuchung.value.id,
        untersuchung,
        payload
      );
    } catch (err: any) {
      error.value = err?.message || 'Fehler beim Aktualisieren';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  // Delete entry
  async function remove(id: number, untersuchung: string) {
    loading.value = true;
    error.value = null;
    try {
      await store.deleteUntersuchung(id, art, untersuchung);
    } catch (err: any) {
      error.value = err?.message || 'Fehler beim Löschen';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  return {
    currentUntersuchung,
    loading,
    error,
    fetchDetail,
    create,
    update,
    remove
  };
}
