<script setup lang="ts">
import { ref } from 'vue';
import { mdiEye, mdiEyeOff, mdiLockOutline, mdiEmailOutline } from '@mdi/js';
import MKPLogo from '@/assets/mkp/MKP_Logo_Webseite.svg';

const email = ref('');
const password = ref('');
const showPassword = ref(false);
const submitted = ref(false);
const validEmail = (value: string) => /\S+@\S+\.\S+/.test(value) || 'Bitte geben Sie eine gültige E-Mail-Adresse ein.';
const requiredPassword = (value: string) => !!value || 'Bitte geben Sie Ihr Passwort ein.';

function submit() {
  submitted.value = true;
  // Frontend only: no credentials are sent or stored.
}
</script>

<template>
  <div class="login-page">
    <v-card class="login-card" elevation="3" rounded="lg">
      <div class="login-brand">
        <img :src="MKPLogo" alt="MKP Diagnostik" class="login-logo" />
        <h1>Willkommen zurück</h1>
        <p>Melden Sie sich an, um fortzufahren.</p>
      </div>
      <v-form @submit.prevent="submit" validate-on="submit">
        <label for="login-email" class="field-label">E-Mail-Adresse</label>
        <v-text-field
          id="login-email" v-model="email" type="email" autocomplete="username"
          placeholder="name@beispiel.de" :prepend-inner-icon="mdiEmailOutline"
          :rules="[validEmail]" variant="outlined" density="comfortable" class="mb-3"
        />
        <label for="login-password" class="field-label">Passwort</label>
        <v-text-field
          id="login-password" v-model="password" autocomplete="current-password"
          :type="showPassword ? 'text' : 'password'" placeholder="Passwort eingeben"
          :prepend-inner-icon="mdiLockOutline" :append-inner-icon="showPassword ? mdiEyeOff : mdiEye"
          @click:append-inner="showPassword = !showPassword"
          :rules="[requiredPassword]" variant="outlined" density="comfortable"
        />
        <v-alert v-if="submitted" class="my-4" type="info" variant="tonal" density="compact">
          Dies ist eine Demo-Anmeldeseite. Die Anmeldung ist noch nicht mit einem Server verbunden.
        </v-alert>
        <v-btn type="submit" class="login-submit mt-4" block size="large" elevation="0">Anmelden</v-btn>
      </v-form>
      <p class="login-footer">MKP Diagnostik</p>
    </v-card>
  </div>
</template>

<style scoped>
.login-page { min-height: 100vh; display: flex; align-items: center; justify-content: center; padding: 32px 16px; background: linear-gradient(145deg, #edf6fa 0%, #f8fafc 65%, #e4eff5 100%); }
.login-card { width: 100%; max-width: 460px; padding: 42px 38px 25px; background: #fff; border: 1px solid #e1eaf0; }
.login-brand { text-align: center; margin-bottom: 32px; }
.login-logo { width: 225px; max-width: 100%; height: 70px; object-fit: contain; margin: 0 auto 24px; }
h1 { color: var(--corp-logo-dark-blue, #004972); font-size: 1.6rem; font-weight: 700; margin-bottom: 8px; }
.login-brand p { color: var(--corp-dark-gray, #555658); font-size: .93rem; }
.field-label { display: block; font-size: .88rem; font-weight: 600; color: var(--corp-logo-dark-blue, #004972); margin-bottom: 8px; }
.login-submit { background: var(--corp-logo-dark-blue, #004972) !important; color: #fff !important; font-weight: 650; text-transform: none; letter-spacing: .02em; }
.login-submit:hover { background: var(--corp-light-blue, #12a5e2) !important; }
.login-footer { margin-top: 32px; text-align: center; font-size: .78rem; color: #758896; }
@media (max-width: 480px) { .login-card { padding: 30px 22px 20px; } h1 { font-size: 1.4rem; } }
</style>
