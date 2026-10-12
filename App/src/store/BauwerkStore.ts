import { defineStore, acceptHMRUpdate } from 'pinia';
import { ref, computed, watch } from 'vue';
import { useRouter, type RouteLocationNormalized } from 'vue-router';

import type { IBauwerk } from '@/types/apiInterfaces';
import { apiConnector } from '@/utils/ApiConnector';
import { MisconfigurationError } from '@/utils/errors';


export const useBauwerkStore = defineStore('bauwerkStore', () => {
  const bauwerke = ref<IBauwerk[]>([]);
  const selectedBauwerk = ref<IBauwerk | null>(null);
  const lastKnownBauwerk = ref<IBauwerk | null>(null);

  const _router = useRouter();

  // keep current bauwerk up to date on route change 
  _router.beforeResolve(async to => {
    const futureRouteId = _getIdParamFromRoute(to);
    if (futureRouteId === null) {
      return;
    }
    selectedBauwerk.value = await findBauwerkById(futureRouteId);
  });

  watch(selectedBauwerk, (bauwerk) => {
    if (bauwerk !== null) {
      lastKnownBauwerk.value = bauwerk;
    }
  }, { immediate: true });

  const activeBauwerk = computed<IBauwerk>(() => {
    /*
      A handle to the currently selected bauwerk which doesn't need null checking to satisfy typescript.
      Though only use this if you are sure that a bauwerk is selected. Otherwise an Error will be thrown.
    */
    if (selectedBauwerk.value) {
      return selectedBauwerk.value;
    }
    if (lastKnownBauwerk.value) {
      return lastKnownBauwerk.value;
    }
    throw new Error('Using activeBauwerk without a selectedBauwerk is forbidden, as it cannot be null.');
  });

  // update selectedBauwerk when bauwerke list changes
  watch(bauwerke, () => {
    if (selectedBauwerk.value !== null) {
      selectedBauwerk.value = bauwerke.value.find(b => b.id === selectedBauwerk.value?.id) || null;
    }
  });

  async function setBauwerkList(): Promise<void> {
    /*
     fetch bauwerke from api.
    */
    const data = await apiConnector.getBauwerke();
    data.sort((a, b) => a.bezeichnung.localeCompare(b.bezeichnung));

    bauwerke.value = data;
  }

  async function findBauwerkById(bauwerkId: number): Promise<IBauwerk | null> {
    /*
      Find a bauwerk in the bauwerke list by its id.
      If the bauwerk is not found, refresh bauwerke from the api and retry once.

      Returns the bauwerk instance if found, otherwise null.
    */
    const findBauwerk = () => bauwerke.value.find((bauwerk: IBauwerk) => bauwerk.id === bauwerkId) || null;
    
    let bauwerk = findBauwerk();

    if (!bauwerk) {
      // request bauwerke again one time when bauwerk is not found 
      // mostly happens on page reload
      // or in case the bauwerk was added by someone else and this user's bauwerke list is outdated.
      await setBauwerkList();
      bauwerk = findBauwerk();
    }
    
    return bauwerk;
  }

  function _getIdParamFromRoute(route: RouteLocationNormalized): number | null {
    
    // safely check if (an object has a property in js) the route has a bauwerkId parameter
    const routeHasIdParam = Object.prototype.hasOwnProperty.call(route.params, 'bauwerkId');
    
    if (routeHasIdParam) {
      const bauwerkId = route.params.bauwerkId;
      
      // sanity check in case router configuration changes
      // ToDo: Should be covered by a test instead
      try {
        if (typeof bauwerkId === 'string') {
          return parseInt(bauwerkId);
        } else {
          throw new Error(`Expected bauwerkId to be of type string. Got ${typeof bauwerkId} instead.`);
        }
      } catch {
        throw new MisconfigurationError('The route parameter "bauwerkId" cannot be parsed to an integer.');
      }
    } else {
      return null;
    }
  }

  return {
    bauwerke,
    selectedBauwerk,
    activeBauwerk,
    setBauwerkList,
    findBauwerkById,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useBauwerkStore, import.meta.hot))
}