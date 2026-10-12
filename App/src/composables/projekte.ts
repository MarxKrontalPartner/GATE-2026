import { ref, type Ref } from "vue";

import { apiConnector } from "@/utils/ApiConnector";
import { useFeedbackStore, FeedbackType } from "@/store/FeedbackStore";
import type { IProjekt } from "@/types/apiInterfaces";


export function useProjekte({ all = false } = {}) {
  const projektList: Ref<IProjekt[]> = ref([]);

  
  async function refreshProjektList({ errorFeedback = false } = {}) {
    try {
      const projekte = await apiConnector.getProjekte(all);
      // Sort the data by projektnummer
      projekte.sort((a, b) => {
        return projektnummerToNumber(a.projektnummer) - projektnummerToNumber(b.projektnummer);
      });
      projektList.value = projekte;
    } catch {
      if (errorFeedback) {
        useFeedbackStore().addFeedback({
          message: "Projekte konnten nicht geladen werden",
          type: FeedbackType.ERROR,
          action: "Projekte laden"
        });
      }
    }
  }

  function projektnummerToNumber(projektnummer: string) {
    return Number(projektnummer.replace(".", ""));
  }

  return { projektList, refreshProjektList };
}
