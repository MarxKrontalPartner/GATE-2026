import { ref, watch } from "vue";
import { useRoute } from "vue-router";


export function useBauwerkIdRouteParam() {
  const route = useRoute();
  const bauwerkId = ref<string | string[]>(route.params.bauwerkId);
  
  // beim reload (f5) kann es sein, dass die ID noch nicht gesetzt ist, daher auf Änderungen warten
  watch(
    () => route.params.bauwerkId,
    (newId) => {
      if (newId) {
        bauwerkId.value = newId; // ID aktualisieren, wenn sie verfügbar ist
      }
    }
  );
  
  // wenn die ID noch nicht gesetzt ist, auf 0 setzen -> wird "instant" überschrieben
  if (!bauwerkId.value) {
    bauwerkId.value = '0';
  }
  
  return { bauwerkId }
}