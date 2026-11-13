<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { login } from '@/utils/auth';

const router = useRouter();
const username = ref('');
const password = ref('');
const error = ref('');
const loading = ref(false);

async function submit() {
  error.value = '';
  loading.value = true;
  try {
    await login(username.value, password.value);
    router.replace('/');
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="login" @submit.prevent="submit">
    <h1>Login</h1>
    <input v-model="username" placeholder="Username" autocomplete="username" required />
    <input v-model="password" type="password" placeholder="Password" autocomplete="current-password" required />
    <p v-if="error" class="error">{{ error }}</p>
    <button type="submit" :disabled="loading">{{ loading ? 'Logging in…' : 'Log in' }}</button>
  </form>
</template>

<style scoped>
.login { max-width: 320px; margin: 15vh auto; display: flex; flex-direction: column; gap: 12px; }
.error { color: #c62828; margin: 0; }
</style>