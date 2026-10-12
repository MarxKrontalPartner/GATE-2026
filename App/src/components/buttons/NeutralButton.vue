<template>
  <v-btn color="primary" variant="tonal" class="text-body-2">
    <!-- Forward slots -->
    <slot></slot>
    <slot name="prepend-icon"></slot>
    <slot name="append-icon"></slot>
    <slot name="loader"></slot>
  </v-btn>
</template>

<!-- 
# Intended use
Multipurpose button.
May be used for saving, initiating an edit, to submit a form, navigate to a new page, or trigger an action.
-->