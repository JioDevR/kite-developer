<template>
  <aside
    class="w-[68px] min-w-[68px] max-w-[68px] h-screen bg-[#0d1017] border-r border-slate-800/80 flex flex-col items-center justify-between py-3 z-30 select-none shadow-2xl"
  >
    <!-- Top Branding / Logo -->
    <div class="flex flex-col items-center gap-1">
      <div
        class="w-11 h-11 rounded-2xl bg-gradient-to-tr from-sky-600 to-indigo-600 flex items-center justify-center shadow-lg shadow-sky-600/30 text-white font-black text-xl tracking-tighter"
        title="Kite Multi-WebApp"
      >
        <svg
          class="w-6 h-6 transform -rotate-12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M12 2L2 7l10 5 10-5-10-5z" />
          <path d="M2 17l10 5 10-5" />
          <path d="M2 12l10 5 10-5" />
        </svg>
      </div>
      <div class="w-8 h-[1px] bg-slate-800/80 my-2" />
    </div>

    <!-- Middle: Service Icons list -->
    <div class="flex-1 w-full overflow-y-auto overflow-x-hidden flex flex-col items-center py-1">
      <ServiceIcon
        v-for="service in services"
        :key="service.id"
        :service="service"
        @select="onSelectService"
        @contextmenu="onContextMenu"
      />

      <!-- Add Service Button -->
      <button
        @click="openAddModal"
        title="Añadir nuevo servicio"
        class="relative w-12 h-12 my-2 rounded-2xl border-2 border-dashed border-slate-700/80 hover:border-sky-400/80 text-slate-400 hover:text-sky-300 hover:bg-sky-500/10 flex items-center justify-center transition-all duration-200 outline-none group"
      >
        <Plus class="w-6 h-6 transition-transform duration-200 group-hover:scale-110" />
      </button>
    </div>

    <!-- Bottom Actions & Metrics -->
    <div class="flex flex-col items-center gap-3 pt-2 border-t border-slate-800/80 w-full">
      <!-- RAM Savings Indicator -->
      <div
        v-if="ramSavedEstimateMB > 0"
        :title="`Ahorro estimado aprox: ~${ramSavedEstimateMB} MB de memoria mediante suspensión inteligente`"
        class="flex flex-col items-center text-center cursor-help px-1"
      >
        <span class="text-[9px] font-bold text-emerald-400 tracking-wider">
          -{{ ramSavedEstimateMB }}MB
        </span>
        <span class="text-[8px] text-slate-500">RAM</span>
      </div>

      <!-- Quick Action: Suspend Current or Settings -->
      <button
        v-if="activeService"
        @click="suspendService(activeService.id)"
        title="Suspender servicio activo para liberar recursos de memoria"
        class="w-10 h-10 rounded-xl bg-slate-900/80 hover:bg-amber-500/20 text-slate-400 hover:text-amber-400 flex items-center justify-center transition-all border border-slate-800"
      >
        <Moon class="w-4 h-4" />
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { Plus, Moon } from 'lucide-vue-next';
import ServiceIcon from './ServiceIcon.vue';
import { useServices } from '../stores/services';
import type { ServiceInstance } from '../types/service';

const {
  services,
  activeService,
  ramSavedEstimateMB,
  activateService,
  suspendService,
  openAddModal,
  openContextMenu,
} = useServices();

function onSelectService(id: string) {
  activateService(id);
}

function onContextMenu(e: MouseEvent, service: ServiceInstance) {
  openContextMenu(e, service);
}
</script>
