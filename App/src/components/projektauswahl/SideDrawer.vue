<template>
  <SideDrawer>
    <!-- Bauwerke List -->
    <NavCategory title="Bauwerke" :prepend-icon="mdiBridge" value="bauwerksverwaltung">
    <!-- Listenansicht Button -->
      <NavLink 
        :class="{ active: activeButton === 'bauwerkswahl' }" 
        @click="setActiveButton('bauwerkswahl')"
        :to="{ name: 'bauwerkswahl' }" 
        :prepend-icon="mdiListBoxOutline"
      >Listenansicht</NavLink>
    </NavCategory>
  </SideDrawer>
</template>

<script setup lang="ts">
import { mdiBridge, mdiListBoxOutline } from '@mdi/js';
import { ref } from 'vue';
import { SideDrawer, NavCategory, NavLink } from '@/components/sidedrawer';

const activeButton = ref('');

function setActiveButton(buttonName: string) {
  activeButton.value = buttonName;
}
</script>
