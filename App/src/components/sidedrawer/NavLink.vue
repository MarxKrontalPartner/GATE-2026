<template>    
  <v-hover>
    <template v-slot:default="{ isHovering, props}">
      <!-- 
      Bind attrs passed by parent component. Also bind hover props for functionality.
      text-caption sets font-size for icon. Text font-weight get's set in v-list-item-title.
      The prop color="primary" sets the color for when the element is active.
      -->
      <v-list-item
        v-bind="{...$attrs, ...props }"
        class="my-0 text-caption"
        color="primary"
        :class="isHovering ? 'text-primary': ''"
      >
        <v-list-item-title class="text-body-2 font-weight-regular">
          <slot name="default" />
        </v-list-item-title>
      </v-list-item>
    </template>
  </v-hover>
</template>

<script setup lang="ts">
/*
  This component is a wrapper for v-list-item.
  It has a custom hover effect and a default slot for text content.
  The default slot should be used instead of the title prop.
  
  Example usage:
    <NavLink value="mon_messkonzept" :to="{ name: 'messkonzepte' }">Messkonzept</NavLink>
*/


// Disable automatic atts bind to root element
defineOptions({
  inheritAttrs: false
});
</script>

<style scoped>
.v-list-item {
  --custom-list-icon-width: 2rem;
  --default-vuetify-indent-padding: calc(var(--parent-padding) + var(--list-indent-size));
  /* bigger values push NavLinks to the left, smaller to the right */
  --custom-indent-padding: 2rem; 
  --indent-padding: calc(
    var(--default-vuetify-indent-padding) 
    - var(--custom-indent-padding)
    - var(--custom-list-icon-width)
    + var(--list-indent-size)
  );
  grid-template-columns: var(--custom-list-icon-width) 1fr auto;
}
</style>