<template>
  <v-row>
    <v-col :cols="6" :xs="12" :sm="6" :md="8" :lg="9">
      <h1 class="mt-3 ms-5 text-h6 font-weight-regular text-secondary">Bauwerkswahl</h1>
    </v-col>
    <v-col :cols="6" :xs="12" :sm="6" :md="4" :lg="3">
      <v-autocomplete
        v-model="selectedItems"
        :prepend-inner-icon="mdiMagnify"
        :clear-icon="mdiClose"
        label="Suche"
        :items="searchItems"
        variant="outlined"
        clearable
        multiple
        class="mt-3 me-3"
        @update:modelValue="filterSelection()"
      >
      </v-autocomplete>
    </v-col>
  </v-row>
  <FeedbackFeedComponent />
  <v-container class="w-75">
    <v-row>
      <v-col 
        :cols="12" :sm="12" :md="6" :lg="6" :xl="6"
        @click="selectBauwerk(bauwerk.id)"
        v-for="(bauwerk, index) in filteredItems" :key="index" 
      >
        <BauwerkCard 
          :bauwerk="bauwerk" 
          :projekte="getProjekteForBauwerk(bauwerk)"
        />
      </v-col>
      <v-col :cols="12" :sm="12" :md="6" :lg="6" :xl="6">
        <BauwerkCreateCard />
      </v-col>
    </v-row>
  </v-container>

</template>

<script setup lang="ts">
import { mdiMagnify, mdiClose } from '@mdi/js';
import { storeToRefs } from 'pinia';
import { ref, onMounted, watch, type Ref } from "vue";

import type { IBauwerk } from "@/types/apiInterfaces";
import { BauwerkCard, BauwerkCreateCard } from "@/components/bauwerkCards"
import FeedbackFeedComponent from "@/components/FeedbackFeedComponent.vue";
import router from '@/router';
import { getBauwerkeListWithFeedback } from '@/utils/FeedbackHooks';
import { useProjekte } from '@/composables/projekte';
import { useBauwerkStore } from '@/store/BauwerkStore';

const { projektList, refreshProjektList } = useProjekte();
const bauwerkStore = useBauwerkStore();
const { bauwerke } = storeToRefs(bauwerkStore);  // necessary for watcher

// get bauwerke when loading this page
onMounted(async () => {
  await refreshProjektList();
  await getBauwerkeListWithFeedback();
});

const searchItems = ref(bauwerke.value.map((bauwerk) => bauwerk.bezeichnung) ?? []);
const selectedItems: Ref<Array<string>> = ref([]);

const filteredItems = ref(bauwerke.value);

function filterSelection(){
  const bauwerkeInSelectedItems = bauwerke.value.filter((bauwerk) => {
    return selectedItems.value.includes(bauwerk.bezeichnung);
  });
  filteredItems.value = bauwerkeInSelectedItems ?? [];

  if(selectedItems.value.length === 0){
    filteredItems.value = bauwerke.value;
  }
}

watch(bauwerke, () => {
  if (bauwerke.value) {
    filterSelection();
    searchItems.value = bauwerke.value.map((bauwerk) => bauwerk.bezeichnung) ?? [];
  }
}, { immediate: true});

function selectBauwerk(bauwerkId: number) {
  // The Untersuchungsstellen are the entry point of a Bauwerk.
  router.push({ name: 'Untersuchungsstellen', params: { bauwerkId: bauwerkId.toString() } });
}

function getProjekteForBauwerk(bauwerk: IBauwerk) {
  if (!projektList.value) return "";
  const projekte = projektList.value.filter(projekt => bauwerk.projekt.includes(projekt.id));
  const projekNummern = projekte.map(projekt => projekt.projektnummer);
  return projekNummern.join(", ");
}
</script>
