import { computed, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useDashboardStore } from '@/store/DashboardStore';
import type { DashboardBauwerk } from '@/repositories/dashboardRepository';
import type { StatusFilter } from '@/components/dashboard/statusTypes';

export function useDashboardFilters() {
  const store = useDashboardStore();
  const { bauwerke, available } = storeToRefs(store);
  const archiveFilter = ref<'active' | 'archived'>('active');
  const statusFilter = ref<StatusFilter | null>(null);
  const search = ref('');
  const archived = computed(() => archiveFilter.value === 'archived');
  const visibleLocal = computed(() => bauwerke.value.filter(row => row.archived === archived.value));
  const visibleServer = computed(() => available.value.filter(row => row.archived === archived.value));
  const archiveCounts = computed(() => ({
    active: bauwerke.value.filter(b => !b.archived).length + available.value.filter(b => !b.archived).length,
    archived: bauwerke.value.filter(b => b.archived).length + available.value.filter(b => b.archived).length,
  }));
  const counts = computed(() => ({
    synced: visibleLocal.value.filter(b => b.syncStatus === 'synced' && !b.serverUpdateAvailable).length,
    unsynced: visibleLocal.value.filter(b => b.syncStatus === 'unsynced').length,
    conflict: visibleLocal.value.filter(b => b.syncStatus === 'conflict').length,
    updates: visibleLocal.value.filter(b => b.serverUpdateAvailable).length,
    available: visibleServer.value.length,
  }));
  function matchesStatus(row: DashboardBauwerk): boolean {
    switch (statusFilter.value) {
      case 'synced': return row.syncStatus === 'synced' && !row.serverUpdateAvailable;
      case 'unsynced': return row.syncStatus === 'unsynced';
      case 'conflict': return row.syncStatus === 'conflict';
      case 'updates': return row.serverUpdateAvailable;
      case 'available': return false;
      default: return true;
    }
  }
  const matchesSearch = (name: string, number: string) =>
    `${name} ${number}`.toLocaleLowerCase('de').includes((search.value ?? '').toLocaleLowerCase('de').trim());
  const filteredLocal = computed(() => visibleLocal.value.filter(b => matchesStatus(b) &&
    matchesSearch(b.bezeichnung, b.bauwerksnummer)));
  const filteredServer = computed(() => statusFilter.value && statusFilter.value !== 'available' ? [] :
    visibleServer.value.filter(b => matchesSearch(b.bezeichnung, b.bauwerksnummer)));
  const lastOpened = computed(() => bauwerke.value.filter(b => b.lastOpenedAt)
    .sort((a, b) => (b.lastOpenedAt ?? '').localeCompare(a.lastOpenedAt ?? ''))[0] ?? null);
  function clearFilters() { search.value = ''; statusFilter.value = null; }
  return { archiveFilter, statusFilter, search, counts, archiveCounts, filteredLocal,
    filteredServer, lastOpened, clearFilters };
}
