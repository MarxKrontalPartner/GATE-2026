<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router';
import TopBar from './components/TopBar.vue';
import { useBauwerkStore } from '@/store/BauwerkStore';
import { useDrawerStore } from '@/store/sideDrawerStore';


const drawerStore = useDrawerStore();
const route = useRoute();

// A little bit of a dirty workaround to ensure a router beforeResolve hook 
// inside the store is completely set up before the store is used in any RouterView for the first time.
// Without this, the router hook might not be triggered when the page, thats initially loaded,
// is inside a bauwerk, resulting in some errors.
useBauwerkStore();
</script>

<template>
  <v-app>
    <TopBar v-if="route.name !== 'login'" :toggleDrawer="drawerStore.toggleDrawer"/>
    <component v-if="route.name !== 'login'" :is="drawerStore.drawerComponent" v-model="drawerStore.showDrawer"/>
    <v-main>
      <RouterView></RouterView>
    </v-main>
  </v-app>
</template>
