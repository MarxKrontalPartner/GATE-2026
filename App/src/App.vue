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
    <!-- No top bar or drawer on the login page -->
    <template v-if="!route.meta.public">
      <TopBar :toggleDrawer="drawerStore.toggleDrawer"/>
      <component :is="drawerStore.drawerComponent" v-model="drawerStore.showDrawer"/>
    </template>
    <v-main>
      <RouterView></RouterView>
    </v-main>
  </v-app>
</template>


<style lang="scss">
@import '@/assets/base.css';
</style>
