<template>
  <section class="gate-workspace-section" aria-labelledby="gate-workspace-stellen-heading">
    <div class="gate-workspace-section__heading">
      <div>
        <h2 id="gate-workspace-stellen-heading">Untersuchungsstellen ({{ stellen.length }})</h2>
        <p>Alle gespeicherten Untersuchungsstellen dieses {{ projektId === null ? 'Bauwerks' : 'Projekts' }}</p>
      </div>
      <GateActionButton intent="primary" :prepend-icon="mdiPlus" :disabled="projekte.length === 0" @click="createDialogOpen = true">
        Untersuchungsstelle hinzufügen
      </GateActionButton>
    </div>
    <v-alert v-if="projekte.length === 0" type="warning" variant="tonal">
      Vor dem Anlegen einer Untersuchungsstelle muss diesem Bauwerk ein Projekt zugeordnet sein.
    </v-alert>
    <v-alert v-if="stellen.length === 0" type="info" variant="tonal">
      Hier sind noch keine Untersuchungsstellen gespeichert. Wählen Sie „Untersuchungsstelle hinzufügen“.
    </v-alert>
    <div v-else class="gate-workspace-stellen-grid">
      <WorkspaceStelleCard v-for="stelle in stellen" :key="stelle.id"
        :stelle="stelle" :project-name="projectLabel(stelle.projekt)"
        :methods="methodsByStelle.get(stelle.id) ?? []"
        :methods-loading="methodsLoading" :methods-error="Boolean(methodsError)"
        @open="openStelle" />
    </div>
    <v-alert v-if="methodsError && stellen.length" type="warning" variant="tonal" class="mt-3">
      {{ methodsError }}
      <GateActionButton intent="secondary" class="mt-2" @click="refreshMethods">Untersuchungen erneut laden</GateActionButton>
    </v-alert>
    <WorkspaceStelleCreateDialog :open="createDialogOpen" :bauwerk-id="bauwerkId"
      :projekte="projekte" :projekt-id="projektId"
      @close="createDialogOpen = false" @created="onCreated" />
  </section>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue';
import { mdiPlus } from '@mdi/js';
import { useRouter } from 'vue-router';
import { GateActionButton } from '@/components/common';
import type { IProjekt, IUntersuchungsstelle } from '@/types/apiInterfaces';
import { useWorkspaceStelleMethods } from '@/composables/bwd/useWorkspaceStelleMethods';
import WorkspaceStelleCard from './WorkspaceStelleCard.vue';
import WorkspaceStelleCreateDialog from './WorkspaceStelleCreateDialog.vue';

const props = withDefaults(defineProps<{
  bauwerkId: number;
  stellen: IUntersuchungsstelle[];
  projekte: IProjekt[];
  projektId?: number | null;
}>(), { projektId: null });
const emit = defineEmits<{ created: [] }>();
const router = useRouter();
const createDialogOpen = ref(false);
const { methodsByStelle, loading: methodsLoading, error: methodsError, refresh: refreshMethods } =
  useWorkspaceStelleMethods(computed(() => props.bauwerkId));
function projectLabel(id: number | null): string {
  if (id === null) return 'Kein Projekt zugeordnet';
  const project = props.projekte.find(p => p.id === id);
  return project ? `${project.projektnummer} · ${project.name}` : `Projekt #${id}`;
}
function onCreated() {
  emit('created');
}
function openStelle(id: number) {
  void router.push({ name: 'UntersuchungsstelleDetail',
    params: { bauwerkId: String(props.bauwerkId), stelleId: String(id) },
    query: props.projektId !== null ? { projekt: String(props.projektId) } : {},
  });
}
</script>
