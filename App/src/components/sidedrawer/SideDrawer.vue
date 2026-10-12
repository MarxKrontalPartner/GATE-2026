<template>
  <v-navigation-drawer v-model="drawerRef">
    <!-- content for the drawer -->
    <v-list density="default" nav>
      <slot name="default"></slot>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
import { ref } from 'vue';

/*  
  Example usage:
    <SideDrawer>
      <NavCategory>
        <NavLink>...</NavLink>
        <NavLink>...</NavLink>
      </NavCategory>
      <v-divider class="my-2"></v-divider>
      <NavCategory>...</NavCategory>
      <NavLink>...</NavLink>
    </SideDrawer>
*/

const props = defineProps({
    drawer: Boolean
});

// this is required, because props can't be mutated
const drawerRef = ref(props.drawer);
</script>
