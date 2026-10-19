<template>
  <div class="w-full h-screen bg-[#0b0d13] text-slate-100 font-sans flex items-center justify-center overflow-hidden select-none">
    <div class="max-w-md w-full px-6 text-center flex flex-col items-center animate-in fade-in duration-300">
      <!-- Kite Glowing Badge -->
      <div class="relative mb-6">
        <div class="absolute -inset-2 bg-gradient-to-r from-sky-500 to-indigo-600 rounded-3xl blur-xl opacity-30 animate-pulse" />
        <div class="relative w-20 h-20 rounded-3xl bg-slate-900 border border-slate-700/80 flex items-center justify-center text-sky-400 shadow-2xl">
          <svg
            class="w-10 h-10 transform -rotate-12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M12 2L2 7l10 5 10-5-10-5z" />
            <path d="M2 17l10 5 10-5" />
            <path d="M2 12l10 5 10-5" />
          </svg>
        </div>
      </div>

      <h1 class="text-2xl font-black text-white tracking-tight">
        Bienvenido a <span class="bg-gradient-to-r from-sky-400 to-indigo-400 bg-clip-text text-transparent">Kite</span>
      </h1>
      <p class="text-xs text-slate-400 mt-2 leading-relaxed">
        Gestor multi-webapp nativo ultraligero. Sesiones 100% aisladas con suspensión inteligente de procesos.
      </p>

      <!-- Feature pills -->
      <div class="grid grid-cols-2 gap-3 mt-6 w-full text-left">
        <div class="p-3 rounded-2xl bg-slate-900/80 border border-slate-800/80">
          <div class="text-[11px] font-bold text-sky-400 mb-0.5">Multi-Perfil Aislado</div>
          <div class="text-[10px] text-slate-400">Varias cuentas simultáneas de WhatsApp sin conflicto</div>
        </div>
        <div class="p-3 rounded-2xl bg-slate-900/80 border border-slate-800/80">
          <div class="text-[11px] font-bold text-amber-400 mb-0.5">Suspensión Inteligente</div>
          <div class="text-[10px] text-slate-400">Libera recursos en memoria conservando la sesión activa</div>
        </div>
      </div>

      <button
        @click="openAddModal"
        class="mt-6 px-6 py-3 rounded-2xl font-semibold text-sm bg-gradient-to-r from-sky-500 to-indigo-600 hover:from-sky-400 hover:to-indigo-500 text-white shadow-xl shadow-sky-500/25 transition-all transform hover:scale-[1.02] active:scale-[0.98] flex items-center gap-2 cursor-pointer"
      >
        <Plus class="w-4 h-4" />
        <span>Añadir Servicio</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Plus } from 'lucide-vue-next';
import { useServices } from '../stores/services';

const { openAddModal } = useServices();
</script>
