import { createPinia } from 'pinia';
import { createApp } from 'vue';
import { VueQueryPlugin } from '@tanstack/vue-query'

import App from './App.vue';
import router from './router';

import vuetify from './plugins/vuetify';
import "@/assets/main.scss"; // must be imported after vuetify!


const app = createApp(App)

const pinia = createPinia();
app.use(pinia);
app.use(VueQueryPlugin)
app.use(vuetify);

app.use(router);
app.mount('#app');
