<template>
  <BauwerkCardBase class="pb-2">
    <v-img
      :src="PlaceholderImage"
      height="200px"
      alt="Bild des Bauwerks"
      cover
    />

    <v-card-title>
      <span class="font-weight-regular text-secondary">{{ bauwerk.bezeichnung }}</span>
    </v-card-title>

    <v-card-subtitle>
      Projekte: {{ projekte || 'Keine Projekte' }}
    </v-card-subtitle>
  </BauwerkCardBase>
</template>
<script setup lang="ts">
import BauwerkCardBase from "./BauwerkCardBase.vue";
import PlaceholderImage from "@/assets/mkp/stock/placeholder-image.png";
import type { IBauwerk } from "@/types/apiInterfaces";


defineProps<{
    bauwerk: IBauwerk,
    projekte?: string,
}>();
</script>
