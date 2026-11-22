<script setup lang="ts">
import { ref } from 'vue';
import { mdiEye, mdiEyeOff, mdiLockOutline, mdiEmailOutline } from '@mdi/js';
import MKPLogo from '@/assets/mkp/MKP_Logo_Webseite.svg';
import MKPLogoInverse from '@/assets/mkp/MKP_Logo_Webseite_invers.svg';
import './LoginView.styles.scss';

const email = ref('');
const password = ref('');
const showPassword = ref(false);
const submitted = ref(false);

const validEmail = (value: string) =>
  /\S+@\S+\.\S+/.test(value) || 'Bitte geben Sie eine gültige E-Mail-Adresse ein.';
const requiredPassword = (value: string) =>
  !!value || 'Bitte geben Sie Ihr Passwort ein.';

function submit() {
  submitted.value = true;
}
</script>

<template>
  <main class="login-page">
    <section class="login-showcase" aria-label="Marx Krontal Partner">
      <div class="login-showcase-content">
        <div class="login-showcase-brand">
  <div class="login-showcase-text">
    <h2>Willkommen bei Marx Krontal Partner</h2>
    <p>
      Ihre digitale Plattform für eine einfache
      und effiziente Zusammenarbeit.
    </p>
  </div>
</div>
        <div class="login-showcase-art" aria-hidden="true">
          <div class="login-art-screen"><div class="login-art-glare"></div></div>
          <div class="login-art-base"></div>
          <div class="login-art-book login-art-book-one"></div>
          <div class="login-art-book login-art-book-two"></div>
          <div class="login-art-pot"></div>
          <div class="login-art-leaf leaf-one"></div>
          <div class="login-art-leaf leaf-two"></div>
          <div class="login-art-leaf leaf-three"></div>
        </div>
      </div>
    </section>

    <section class="login-form-panel" aria-label="Anmeldung">
      <v-card class="login-card" elevation="0" rounded="xl">
        <div class="login-brand">
          <img :src="MKPLogo" alt="Marx Krontal Partner" class="login-logo" />
          <h1>Willkommen zurück</h1>
          <p>Melden Sie sich an, um fortzufahren.</p>
        </div>

        <v-form @submit.prevent="submit" validate-on="submit">
          <label for="login-email" class="field-label">E-Mail-Adresse</label>
          <v-text-field
            id="login-email"
            v-model="email"
            type="email"
            autocomplete="username"
            placeholder="name@beispiel.de"
            :prepend-inner-icon="mdiEmailOutline"
            :rules="[validEmail]"
            variant="outlined"
            density="comfortable"
            class="login-input mb-3"
          />

          <label for="login-password" class="field-label">Passwort</label>
          <v-text-field
            id="login-password"
            v-model="password"
            autocomplete="current-password"
            :type="showPassword ? 'text' : 'password'"
            placeholder="Passwort eingeben"
            :prepend-inner-icon="mdiLockOutline"
            :append-inner-icon="showPassword ? mdiEyeOff : mdiEye"
            @click:append-inner="showPassword = !showPassword"
            :rules="[requiredPassword]"
            variant="outlined"
            density="comfortable"
            class="login-input"
          />

          <v-alert v-if="submitted" class="my-4" type="info" variant="tonal" density="compact">
            Dies ist eine Demo-Anmeldeseite. Die Anmeldung ist noch nicht mit einem Server verbunden.
          </v-alert>

          <v-btn type="submit" class="login-submit mt-5" block size="large" elevation="0">
            Anmelden
          </v-btn>
        </v-form>
        <p class="login-footer">Marx Krontal Partner</p>
      </v-card>
    </section>
  </main>
</template>
