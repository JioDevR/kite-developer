<template>
  <div class="h-screen w-full bg-slate-900 flex flex-col text-slate-100 overflow-hidden select-none">
    <!-- Header -->
    <div class="px-6 py-5 border-b border-slate-800 flex items-center justify-between shrink-0">
      <div>
        <h2 class="text-lg font-bold text-white flex items-center gap-2">
          <span>Añadir Servicio Web</span>
        </h2>
        <p class="text-xs text-slate-400 mt-0.5">
          Crea una nueva instancia aislada con cookies y sesión independientes
        </p>
      </div>
      <button
        @click="handleClose"
        title="Cerrar ventana"
        class="w-8 h-8 rounded-full bg-slate-800/80 hover:bg-slate-700 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
      >
        <X class="w-4 h-4" />
      </button>
    </div>

    <!-- Body -->
    <div class="p-6 flex-1 flex flex-col gap-5 overflow-y-auto">
      <!-- 1. Catalog Grid -->
      <div>
        <label class="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-2">
          1. Selecciona la plataforma
        </label>
        <div class="grid grid-cols-3 gap-2.5">
          <button
            v-for="(meta, key) in templates"
            :key="key"
            type="button"
            @click="selectTemplate(key as ServiceTemplate)"
            :class="[
              'flex flex-col items-center justify-center p-3 rounded-2xl border text-center transition-all duration-150 cursor-pointer',
              selectedTemplate === key
                ? 'border-sky-500 bg-sky-500/15 text-white ring-2 ring-sky-500/30'
                : 'border-slate-800 bg-slate-800/40 hover:bg-slate-800/90 text-slate-300 hover:border-slate-700',
            ]"
          >
            <svg class="w-6 h-6 mb-1.5" viewBox="0 0 24 24" v-html="meta.svg" />
            <span class="text-xs font-medium truncate w-full">{{ meta.name }}</span>
          </button>
        </div>
      </div>

      <!-- 2. Profile Details -->
      <div class="flex flex-col gap-3">
        <div>
          <label class="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1.5">
            2. Nombre del Perfil
          </label>
          <input
            v-model="name"
            type="text"
            placeholder="Ej: WhatsApp Personal, Telegram Trabajo..."
            class="w-full px-4 py-2.5 bg-slate-800/80 border border-slate-700 rounded-xl text-sm text-white placeholder-slate-500 focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500"
            required
          />
        </div>

        <!-- URL (for custom or overriding) -->
        <div v-if="selectedTemplate === 'customUrl'">
          <label class="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1.5">
            URL del Servicio
          </label>
          <input
            v-model="customUrl"
            type="url"
            placeholder="https://app.tuweb.com"
            class="w-full px-4 py-2.5 bg-slate-800/80 border border-slate-700 rounded-xl text-sm text-white placeholder-slate-500 focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500"
            required
          />
        </div>

        <!-- Auto-suspend minutes -->
        <div>
          <label class="block text-xs font-semibold text-slate-300 uppercase tracking-wider mb-1.5 flex items-center justify-between">
            <span>Auto-Hibernación (Ahorro de RAM)</span>
            <span class="text-amber-400 font-mono text-[10px]">Libera recursos al estar inactivo</span>
          </label>
          <select
            v-model.number="autoSuspendMinutes"
            class="w-full px-4 py-2.5 bg-slate-800/80 border border-slate-700 rounded-xl text-sm text-white focus:outline-none focus:border-sky-500 focus:ring-1 focus:ring-sky-500"
          >
            <option :value="10">Suspender tras 10 minutos inactivo</option>
            <option :value="20">Suspender tras 20 minutos inactivo (Recomendado)</option>
            <option :value="45">Suspender tras 45 minutos inactivo</option>
            <option :value="0">Mantener siempre en RAM (No hibernar)</option>
          </select>
        </div>

        <!-- Error Alert Banner -->
        <div
          v-if="errorMessage"
          class="p-3 bg-rose-500/15 border border-rose-500/30 rounded-xl text-xs text-rose-300 flex items-center gap-2"
        >
          <span>{{ errorMessage }}</span>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="px-6 py-4 border-t border-slate-800 bg-slate-950/60 flex items-center justify-end gap-3 shrink-0">
      <button
        type="button"
        @click="handleClose"
        class="px-4 py-2.5 rounded-xl text-sm text-slate-400 hover:text-white hover:bg-slate-800 transition-colors cursor-pointer"
      >
        Cancelar
      </button>
      <button
        type="button"
        :disabled="!name.trim() || isSubmitting"
        @click="submit"
        class="px-5 py-2.5 rounded-xl text-sm font-semibold bg-gradient-to-r from-sky-500 to-indigo-600 hover:from-sky-400 hover:to-indigo-500 text-white shadow-lg shadow-sky-500/25 transition-all disabled:opacity-50 disabled:pointer-events-none flex items-center gap-2 cursor-pointer"
      >
        <span v-if="isSubmitting">Creando...</span>
        <span v-else>Crear e Iniciar</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { X } from 'lucide-vue-next';
import { SERVICE_METADATA } from '../assets/serviceIcons';
import type { ServiceTemplate } from '../types/service';
import { useServices } from '../stores/services';

const templates = SERVICE_METADATA;
const { closeAddModal, createService, activateService } = useServices();

const selectedTemplate = ref<ServiceTemplate>('whatsApp');
const name = ref('WhatsApp');
const customUrl = ref('');
const autoSuspendMinutes = ref(20);
const isSubmitting = ref(false);
const errorMessage = ref('');

function isValidHttpUrl(str: string): boolean {
  try {
    const url = new URL(str);
    return url.protocol === 'http:' || url.protocol === 'https:';
  } catch {
    return false;
  }
}

function selectTemplate(t: ServiceTemplate) {
  selectedTemplate.value = t;
  errorMessage.value = '';
  const meta = SERVICE_METADATA[t];
  if (meta) {
    name.value = meta.name;
    customUrl.value = t === 'customUrl' ? '' : meta.defaultUrl;
  }
}

function resetForm() {
  selectedTemplate.value = 'whatsApp';
  name.value = 'WhatsApp';
  customUrl.value = '';
  autoSuspendMinutes.value = 20;
  isSubmitting.value = false;
  errorMessage.value = '';
}

async function handleClose() {
  resetForm();
  await closeAddModal();
}

async function submit() {
  errorMessage.value = '';
  if (!name.value.trim()) {
    errorMessage.value = 'Por favor ingresa un nombre para el servicio';
    return;
  }

  if (selectedTemplate.value === 'customUrl') {
    const trimmedUrl = customUrl.value.trim();
    if (!trimmedUrl) {
      errorMessage.value = 'Por favor ingresa la URL de la web';
      return;
    }
    if (!isValidHttpUrl(trimmedUrl)) {
      errorMessage.value = 'La URL debe ser válida y empezar por https:// o http://';
      return;
    }
  }

  isSubmitting.value = true;
  try {
    const newService = await createService({
      name: name.value.trim(),
      template: selectedTemplate.value,
      url: selectedTemplate.value === 'customUrl' ? customUrl.value.trim() : undefined,
      autoSuspendMinutes: typeof autoSuspendMinutes.value === 'number' ? autoSuspendMinutes.value : 20,
    });
    if (newService && newService.id) {
      await activateService(newService.id);
    }
    resetForm();
    await closeAddModal();
  } catch (e: any) {
    console.error(e);
    errorMessage.value = typeof e === 'string' ? e : 'Error al crear el servicio web';
  } finally {
    isSubmitting.value = false;
  }
}
</script>
