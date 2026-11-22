<template>
  <v-app-bar class="gate-top-bar">
    <template v-slot:prepend>
      <!-- sidebar toggle -->
      <v-app-bar-nav-icon :icon="mdiMenu" @click="props.toggleDrawer"></v-app-bar-nav-icon>
      <!-- MKP Logo -->
      <v-img v-if="shouldShowLogo"
        @click="$router.push({ name: 'bauwerkswahl' })"
        :src="theme.global.current.value.dark ? MKPLogoWebsiteInvers : MKPLogoWebsite" 
        alt="MKP-Logo"
        class="ms-10 cursor-pointer"
        width="252"
      />
    </template>
    <v-spacer></v-spacer>


    <!-- Bauwerkswahl Button -->
  <v-btn icon :to="{ name: 'bauwerkswahl' }" :active=false>
    <v-icon color="primary" :icon="mdiHome" />
    <v-tooltip
      activator="parent"
      text="Bauwerkswahl" 
      location="bottom"
    />
  </v-btn>
    <!-- Account menu -->
    <v-btn icon class="me-5">
      <v-icon color="primary" :icon="mdiAccount" />
      <v-menu activator="parent" :closeOnContentClick="false">
        <v-list density="compact">
          <!-- Toggle theme -->
          <v-list-item
            title="Hell / Dunkel"
            value="theme"
            @click="toggleTheme"
          >
            <template v-slot:prepend>
              <v-icon :icon="mdiThemeLightDark" color="secondary" />
            </template>
          </v-list-item>
          <!-- Logout -->
          <v-list-item
            title="Abmelden"
            value="logout"
            :disabled="loggingOut"
            @click="signOut"
          >
            <template v-slot:prepend>
              <v-icon :icon="mdiLogout" color="secondary" />
            </template>
          </v-list-item>
        </v-list>
      </v-menu>
      <v-tooltip
        activator="parent"
        text="Account"
        location="bottom"
      />
    </v-btn>
  </v-app-bar>
</template>

<script lang="ts" setup>
import { mdiAccount, mdiHome, mdiThemeLightDark, mdiMenu, mdiLogout } from '@mdi/js';
import { useDisplay } from 'vuetify';
import { computed, ref } from 'vue';
import { useTheme } from 'vuetify';
import { useRouter } from 'vue-router';
import { logout } from '@/services/auth';

import MKPLogoWebsite from '@/assets/mkp/MKP_Logo_Webseite.svg';
import MKPLogoWebsiteInvers from '@/assets/mkp/MKP_Logo_Webseite_invers.svg';


const props = defineProps<{
  toggleDrawer: () => void
}>();

const theme = useTheme()

const { width } = useDisplay();
const shouldShowLogo = computed(() => width.value > 500);

// The theme is not persisted.
function toggleTheme() {
  theme.global.name.value = theme.global.current.value.dark ? 'mkpLightTheme' : 'mkpDarkTheme';
}

const router = useRouter();
const loggingOut = ref(false);

// Offline changes stay in the local database and are sent after the next login.
async function signOut() {
  loggingOut.value = true;
  try {
    await logout();
  } finally {
    loggingOut.value = false;
    await router.replace({ name: 'login' });
  }
}



</script>
