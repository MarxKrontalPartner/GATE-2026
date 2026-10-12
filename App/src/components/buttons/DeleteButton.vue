<template>
  <v-btn color="error" variant="tonal" class="text-body-2">
    <!-- Forward slots -->
    <slot></slot>
    <slot name="prepend-icon"></slot>
    <slot name="append-icon"></slot>
    <slot name="loader"></slot>
  </v-btn>
</template>

<!-- 
# Intended use
Clicking this button may delete something.
Which may be data on the server or local changes. 
It may instead open a dialog to ask for confirmation on deletion.
-->