<template>
  <BauwerkCardBase class="px-3 py-2" :hover="false">
    <v-card-title class="px-1">
      <span class="text-body-1 font-weight-regular text-secondary">Neues Bauwerk anlegen</span>
    </v-card-title>

    <NeutralButton @click="openCreateNew()">Neu</NeutralButton>

    <DialogComponent max-width="500" :show="showDialog" @update:modelValue="closeCreateNew()">
      <template #title>
        <span class="font-weight-regular text-secondary">Neues Bauwerk</span>
      </template>
      <template #text>
        <v-text-field
          class="text-secondary"
          v-model="bauwerkBezeichnung"
          label="Bauwerk Name"
          required
          @keyup.enter="addBauwerk()"
        />
        <v-autocomplete
          v-model="selectedProjekte"
          :items="projektList"
          :item-title="(projekt: IProjekt) => `${projekt.projektnummer} ${projekt.name}`"
          item-value="id"
          label="Projekte"
          chips
          closable-chips
          multiple
          hint="Projekte werden im Django-Admin gepflegt"
          persistent-hint
        />
        <v-alert v-if="similarBauwerke.length > 0" type="warning" variant="tonal" density="compact" class="mt-4">
          {{ similarBauwerke.length > 1 ? "Ähnliche Bauwerke vorhanden" : "Ähnliches Bauwerk vorhanden" }}
          <ul>
            <li v-for="bauwerk in similarBauwerke" :key="bauwerk.id">
              • <strong>{{ bauwerk.bezeichnung }}</strong>
            </li>
          </ul>
        </v-alert>
      </template>
      <template #actions>
        <NeutralButton @click="addBauwerk()">Bauwerk hinzufügen</NeutralButton>
      </template>
    </DialogComponent>
  </BauwerkCardBase>
</template>
<script setup lang="ts">
import { BAUWERKE_ENDPOINT } from "@/config/endpoints";
import { computed, ref, type Ref } from "vue";

import BauwerkCardBase from "./BauwerkCardBase.vue";
import { NeutralButton } from "@/components/buttons";
import { DialogComponent } from "@/components/dialogs";
import { useProjekte } from "@/composables/projekte";
import { FeedbackType, useFeedbackStore } from "@/store/FeedbackStore";
import { apiConnector } from "@/utils/ApiConnector";
import { getErrorMessage } from "@/utils/errors";
import type { IBauwerk, IProjekt } from "@/types/apiInterfaces";
import { useBauwerkStore } from "@/store/BauwerkStore";


const { bauwerke, setBauwerkList } = useBauwerkStore();
const { projektList, refreshProjektList } = useProjekte();


const feedbackStore = useFeedbackStore();

const showDialog: Ref<boolean> = ref(false);
const bauwerkBezeichnung: Ref<string> = ref("");
const selectedProjekte: Ref<number[]> = ref([]);
const similarBauwerke = computed(() => searchSimilar(bauwerkBezeichnung.value));

async function openCreateNew() {
  showDialog.value = true;
  await refreshProjektList({ errorFeedback: true });
}

const addBauwerk = async () => {
  if(!bauwerkBezeichnung.value){
    feedbackStore.addFeedback({
      message: "Bauwerk konnte nicht gespeichert werden, da kein Name angegeben wurde",
      type: FeedbackType.ERROR,
      action: "Speichern"
    });
    return;
  }
  if(bauwerke.find(bauwerk => bauwerk.bezeichnung === bauwerkBezeichnung.value)){
    feedbackStore.addFeedback({
      message: "Bauwerk existiert bereits",
      type: FeedbackType.ERROR,
      action: "Speichern"
    });
    return;
  }
  
  let isSuccess;
  let errorMessage = '';
  try {
    const bauwerkData = { bezeichnung: bauwerkBezeichnung.value, projekt: selectedProjekte.value } as IBauwerk;
    await apiConnector.postResource(BAUWERKE_ENDPOINT, bauwerkData);
    await setBauwerkList();
    isSuccess = true;
  } catch (error) {
    isSuccess = false;
    errorMessage = getErrorMessage(error, '');
  } finally {
    feedbackStore.addFeedback({
      message: isSuccess 
        ? `Bauwerk „${bauwerkBezeichnung.value}“`
        : `Bauwerk „${bauwerkBezeichnung.value}“ konnte nicht gespeichert werden ${errorMessage}`.trim(),
      type: isSuccess ? FeedbackType.SUCCESS : FeedbackType.ERROR,
      action: "Speichern"
    });

    closeCreateNew();
  }
};

function closeCreateNew() {
  showDialog.value = false;
  bauwerkBezeichnung.value = "";
  selectedProjekte.value = [];
}

function simplifyString(characters: string) {
  // replace all non space and non alphabetic characters
  let simplified = characters.trim().toLowerCase().replaceAll(/[^a-z\s]/g, "")
  // normalize spaces to single space
  simplified = simplified.replaceAll(/\s+/g, " ");
  return simplified.trim();  // single space to empty string
}

function splitAtSpaces(str: string) {
  return str.split(/\s+/);
}

function searchSimilar(bezeichnung: string): IBauwerk[] {
  /**
   * Searches for bauwerke with similar names based on the user's input.
   * Returns an array of matching bauwerke.
   * 
   * Uses a simple string similarity check:
   * 1. string is normalized to only include lowercase alphabetic characters and spaces.
   * 2. string is split into parts based on spaces.
   * 3. each part is compared with parts of existing bauwerke to find matches.
   * 
   * Basically matches full words instead of partial matches.
   * 
   * Example:
   *   > searchSimilar("Neues Bauwerk");
   *   match: "Anderes Bauwerk", "bauwerk"
   *   no match: "Brückenbauwerk", "Neue Brücke"
   */
  if (!bezeichnung) return [];

  const userInputParts = new Set(splitAtSpaces(simplifyString(bezeichnung)));

  const matchingBauwerke = bauwerke.filter(bauwerk => {
    const simplified = simplifyString(bauwerk.bezeichnung);
    const existingBauwerkParts = splitAtSpaces(simplified);
    return existingBauwerkParts.some(part => part && userInputParts.has(part));
  });
  return matchingBauwerke ?? [];
}
</script>
