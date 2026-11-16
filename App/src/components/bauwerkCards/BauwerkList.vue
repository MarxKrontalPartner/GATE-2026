<template>
  <v-row class="gate-card-grid">
    <v-col v-for="bauwerk in bauwerke" :key="bauwerk.id" cols="12" md="6">
      <BauwerkCard
        class="gate-card-interactive"
        :bauwerk="bauwerk"
        :projekte="projectNumbers(bauwerk)"
        role="button"
        tabindex="0"
        :aria-label="`Bauwerk ${bauwerk.bezeichnung} öffnen`"
        @click="emit('open', bauwerk.id)"
        @keydown.enter.prevent="emit('open', bauwerk.id)"
        @keydown.space.prevent="emit('open', bauwerk.id)"
      />
    </v-col>
    <v-col cols="12" md="6">
      <BauwerkCreateCard />
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import type { IBauwerk, IProjekt } from '@/types/apiInterfaces';
import BauwerkCard from './BauwerkCard.vue';
import BauwerkCreateCard from './BauwerkCreateCard.vue';

const props = defineProps<{ bauwerke: IBauwerk[]; projekte: IProjekt[] }>();
const emit = defineEmits<{ open: [bauwerkId: number] }>();

function projectNumbers(bauwerk: IBauwerk): string {
  return props.projekte
    .filter(projekt => bauwerk.projekt.includes(projekt.id))
    .map(projekt => projekt.projektnummer)
    .join(', ');
}
</script>
