<template>
  <section class="gate-workspace-section" aria-labelledby="gate-workspace-stellen-heading">
    <div class="gate-workspace-section__heading">
      <div>
        <h2 id="gate-workspace-stellen-heading">Untersuchungsstellen ({{ stellen.length }})</h2>
        <p>Gespeicherte Untersuchungsstellen dieses {{ projektId === null ? 'Bauwerks' : 'Projekts' }}</p>
      </div>
      <GateActionButton intent="secondary" :prepend-icon="mdiArrowRight" @click="openAll">
        Zum Arbeitsbereich
      </GateActionButton>
    </div>
    <v-alert v-if="stellen.length === 0" type="info" variant="tonal">
      Hier sind noch keine Untersuchungsstellen gespeichert. Im Arbeitsbereich können Sie neue anlegen.
    </v-alert>
    <div v-else class="gate-workspace-stellen-grid">
      <v-card v-for="stelle in stellen" :key="stelle.id" class="gate-card gate-workspace-stelle" variant="flat">
        <v-card-text>
          <div class="gate-workspace-stelle__header">
            <v-icon :icon="mdiMapMarkerOutline" color="primary" size="28" />
            <div class="gate-workspace-stelle__title">
              <h3>{{ stelle.name }}</h3>
              <p v-if="stelle.projekt" class="gate-workspace-muted">{{ projectLabel(stelle.projekt) }}</p>
            </div>
          </div>
          <p v-if="stelle.bemerkung" class="gate-workspace-stelle__note">{{ stelle.bemerkung }}</p>
          <GateActionButton intent="secondary" block @click="openStelle(stelle.id)">
            Untersuchungen ansehen
          </GateActionButton>
        </v-card-text>
      </v-card>
    </div>
  </section>
</template>

<script setup lang="ts">
import { mdiArrowRight, mdiMapMarkerOutline } from '@mdi/js';
import { useRouter } from 'vue-router';
import { GateActionButton } from '@/components/common';
import type { IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces';
const props = withDefaults(defineProps<{
  bauwerkId: number;
  stellen: IUntersuchungsstelle[];
  projekte: IProjekt[];
  projektId?: number | null;
}>(), { projektId: null });
const router = useRouter();
function projectLabel(id: number): string {
  const project = props.projekte.find(p => p.id === id);
  return project ? `${project.projektnummer} · ${project.name}` : `Projekt #${id}`;
}
function goToWorkspace(stelleId?: number) {
  return router.push({
    name: 'Untersuchungsstellen',
    params: { bauwerkId: String(props.bauwerkId) },
    query: {
      ...(props.projektId !== null ? { projekt: String(props.projektId) } : {}),
      ...(stelleId !== undefined ? { stelle: String(stelleId) } : {}),
    },
  });
}
function openAll() { void goToWorkspace(); }
function openStelle(id: number) { void goToWorkspace(id); }
</script>
