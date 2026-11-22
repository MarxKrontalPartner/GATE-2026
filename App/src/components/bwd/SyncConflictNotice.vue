<template>
  <v-row v-if="conflicts.length">
    <v-col cols="12" md="12">
      <v-alert type="warning" variant="tonal" border="start">
        <div class="text-subtitle-1 mb-1">Konflikt beim Synchronisieren</div>
        <div class="mb-2">
          Diese Felder wurden auf dem Server geändert, während Sie offline gearbeitet haben.
          Beim nächsten Speichern wird der neue Serverwert als Ausgangswert übernommen.
        </div>
        <v-table density="compact">
          <thead>
            <tr><th>Feld</th><th>Neuer Serverwert</th></tr>
          </thead>
          <tbody>
            <tr v-for="row in conflicts" :key="row.field">
              <td>{{ fieldLabel(row.field) }}</td>
              <td>{{ formatValue(row.serverValue) }}</td>
            </tr>
          </tbody>
        </v-table>
      </v-alert>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
/*
  Shows the open sync conflicts (client_sync_conflicts) of one record at the
  bottom of its form: the conflicting fields with the new server value.
  Read only. Saving the form makes the server value the base of these fields.
*/
import { ref, watch } from 'vue';
import { isTauri } from '@tauri-apps/api/core';
import { listConflicts } from '@/services/syncService';

interface FieldConflict { field: string; serverValue: unknown }

const props = defineProps<{
  /** Table of the record, e.g. "bauwerksdiagnostik_rueckprallhammer". */
  table: string;
  recordId: number | string | null | undefined;
  /** Changes whenever the form loads or saves the record, to reload the conflicts. */
  version?: unknown;
}>();

const conflicts = ref<FieldConflict[]>([]);

const FIELD_LABELS: Record<string, string> = {
  hammertyp: 'Hammertyp', schlagrichtung: 'Schlagrichtung',
  oberflaecheGeschliffen: 'Oberfläche geschliffen', kalibriertAm: 'Kalibriert am',
  messreihe: 'Messreihe', medianRueckprallwert: 'Median Rückprallwert',
  datum: 'Datum', zeit: 'Zeit', bemerkung: 'Bemerkung',
  lufttemperatur: 'Lufttemperatur', luftfeuchte: 'Luftfeuchte', abgeschlossen: 'Abgeschlossen',
  untersuchungsstelle: 'Untersuchungsstelle', projekt: 'Projekt', bauwerk: 'Bauwerk',
  // Kernbohrung
  durchmesser: 'Durchmesser', bohrlaenge: 'Bohrlänge', neigungswinkel: 'Neigungswinkel',
  kuehlung: 'Kühlung', verschlussmaterial: 'Verschlussmaterial', bohrkern: 'Bohrkern',
  schichtaufbau: 'Schichtaufbau', bewehrungAngeschnitten: 'Bewehrung angeschnitten',
  kernVollstaendig: 'Kern vollständig',
  // Rissbreitenmessung
  messmittel: 'Messmittel', risse: 'Risse', maxRissbreite: 'Max. Rissbreite',
  monitoringEmpfohlen: 'Monitoring empfohlen',
  // Chloridanalyse
  probe: 'Probe', pruefstelle: 'Prüfstelle', analyseverfahren: 'Analyseverfahren',
  chloridgehaltZement: 'Chloridgehalt (Zement)', zementgehaltAnnahme: 'Zementgehalt (Annahme)',
  grenzwertUeberschritten: 'Grenzwert überschritten',
  // Druckfestigkeitsprüfung
  pruefalter: 'Prüfalter', lagerung: 'Lagerung', bruchlast: 'Bruchlast',
  druckfestigkeit: 'Druckfestigkeit', bruchbild: 'Bruchbild',
};
const fieldLabel = (field: string) => FIELD_LABELS[field] ?? field;

function formatValue(value: unknown): string {
  if (value === null || value === undefined || value === '') return '–';
  if (typeof value === 'boolean') return value ? 'Ja' : 'Nein';
  if (Array.isArray(value)) return value.map((v) => (v && typeof v === 'object' ? JSON.stringify(v) : String(v))).join(', ');
  if (typeof value === 'object') return JSON.stringify(value);
  return String(value);
}

async function load() {
  const id = Number(props.recordId);
  if (!isTauri() || !Number.isSafeInteger(id) || id <= 0) {
    conflicts.value = [];
    return;
  }
  try {
    const all = await listConflicts() as Array<Record<string, unknown>>;
    conflicts.value = all
      .filter((c) => c.table === props.table && Number(c.id) === id)
      .map((c) => ({ field: String(c.field), serverValue: c.serverValue }));
  } catch (error) {
    console.warn('Konflikte konnten nicht geladen werden', error);
    conflicts.value = [];
  }
}

watch(() => [props.table, props.recordId, props.version], () => void load(), { immediate: true });
</script>
