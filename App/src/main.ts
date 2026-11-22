import { createPinia } from 'pinia';
import { createApp } from 'vue';
import { VueQueryPlugin } from '@tanstack/vue-query'

import App from './App.vue';
import router from './router';

import vuetify from './plugins/vuetify';
import { isTauri } from '@tauri-apps/api/core';
import { watchServer } from '@/services/connectivity';
import { installAuthFetch } from '@/services/auth';
import "@/assets/base.css";
import "@/assets/main.scss"; // must be imported after vuetify!


// Every API request carries the Keycloak token; an expired login opens the login page.
installAuthFetch(() => {
  if (router.currentRoute.value.name !== 'login') {
    router.push({ name: 'login', query: { redirect: router.currentRoute.value.fullPath } });
  }
});

const app = createApp(App)

const pinia = createPinia();
app.use(pinia);
app.use(VueQueryPlugin)
app.use(vuetify);

app.use(router);
app.mount('#app');

// Desktop app: keep track of whether the server is reachable (online/offline mode).
// Offline changes are sent only when the user clicks "Synchronisieren".
if (isTauri()) {
  watchServer();
}
