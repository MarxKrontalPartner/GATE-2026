<template> 
  <!-- 'text-caption' sets font-size for icon. Text font-weight get's set in v-list-item-title -->
  <v-list-group class="text-caption">
    <template v-slot:activator="{ props }">
      <v-hover>
        <template v-slot:default="hover">
          <!-- v-list-group activator props enable collapse, hover.props enable hover effect  -->
          <v-list-item 
            v-bind="{...props, ...hover.props}"
            :class="hover.isHovering ? 'text-primary': ''"
          >
            <v-list-item-title class="font-weight-regular text-body-1">
              {{ title }}
            </v-list-item-title>
          </v-list-item>
        </template>
      </v-hover>
    </template>
    <!-- slot for NavLinks or NavCategories -->
    <slot name="default" />
  </v-list-group>
</template>

<script setup lang="ts">
/*
  This component is mostly a wrapper for v-list-group.
  It has a custom hover effect and a slot for NavLinks or other NavCategories.

  Example usage:
    <NavCategory title="Bauwerk" :prepend-icon="mdiBridge" value="bauwerk">
      <NavLink>...</NavLink>
      <NavLink>...</NavLink>
    </NavCategory>

    or:
    <NavCategory title="Monitoring" :prepend-icon="mdiPulse" value="monitoring">
      <NavCategory :prepend-icon="mdiClipboardTextOutline" title="Grundlagen" value="grundlagen">
        <NavLink>...</NavLink>
      </NavCategory>
      <NavLink>...</NavLink>
    </NavCategory>
*/

defineProps<{
  title: string
}>();
</script>

<style scoped>
/* Default padding is way to big */
.v-list-group {
  --indent-padding: 0px;
}
/* thinner list icon */
.v-list-item {
  --custom-list-icon-width: 2rem;
  grid-template-columns: var(--custom-list-icon-width) 1fr auto;
}
</style>