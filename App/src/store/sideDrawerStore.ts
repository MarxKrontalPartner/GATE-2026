// store/DrawerStore.ts
import { defineStore } from 'pinia';
import { ref, shallowRef, watch, type Ref } from 'vue';

import SideDrawer from '@/components/sidedrawer/SideDrawer.vue';
import router from '@/router/index';


export const useDrawerStore = defineStore('drawer', () => {
  const showDrawer = ref(false);
  const drawerComponent: Ref<any> = shallowRef(getDrawerComponent());
  
  const toggleDrawer = () => {
    showDrawer.value = !showDrawer.value;
  };

  function getDrawerComponent() {
    return router.resolve(window.location.pathname)
    .matched.slice(-1)[0]?.components?.sidedrawer ?? SideDrawer
  }

  watch(router.currentRoute, () => {
    drawerComponent.value = getDrawerComponent();
  })

  return { showDrawer, toggleDrawer, drawerComponent };
});
