<template>
  <v-card class="gate-card gate-workspace-project" variant="flat">
    <v-card-text>
      <div class="gate-workspace-project__content">
        <v-icon :icon="mdiBriefcaseOutline" size="32" color="primary" />
        <div>
          <p class="gate-workspace-project__number">Projekt {{ projekt.projektnummer }}</p>
          <h3>{{ projekt.name || projekt.bezeichnung || 'Unbenanntes Projekt' }}</h3>
          <p class="gate-workspace-muted">
            {{ stelleCount }} {{ stelleCount === 1 ? 'Untersuchungsstelle' : 'Untersuchungsstellen' }}
          </p>
        </div>
      </div>
      <GateActionButton block intent="secondary" :prepend-icon="mdiArrowRight" @click="emit('open', projekt.id)">
        Projekt ansehen
      </GateActionButton>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { mdiBriefcaseOutline, mdiArrowRight } from '@mdi/js';
import { GateActionButton } from '@/components/common';
import type { IProjekt } from '@/types/apiInterfaces';
defineProps<{ projekt: IProjekt; stelleCount: number }>();
const emit = defineEmits<{ open: [id: number] }>();
</script>
