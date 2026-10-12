<template>
  <v-btn variant="outlined" class="text-body-2">
    <!-- Forward slots -->
    <slot></slot>
    <slot name="prepend-icon"></slot>
    <slot name="append-icon"></slot>
    <slot name="loader"></slot>
  </v-btn>
</template>

<!-- 
# Intended use
Clicking this button does not modify any data or configuration.
Could be used to close a dialog or dismiss a notification.
 -->