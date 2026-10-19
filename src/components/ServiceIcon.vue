<template>
  <div class="relative group flex items-center justify-center my-2">
    <!-- Active left indicator pill -->
    <div
      v-if="service.state === 'active'"
      class="absolute left-0 w-1 h-8 bg-sky-400 rounded-r-full shadow-lg shadow-sky-500/50 transition-all duration-300"
    />

    <!-- Main Icon Button -->
    <button
      @click="$emit('select', service.id)"
      @contextmenu="$emit('contextmenu', $event, service)"
      :title="`${service.name} • ${
        service.state === 'active'
          ? 'Activo en pantalla'
          : service.state === 'suspended'
          ? 'Hibernado (0 MB RAM)'
          : 'En segundo plano'
      }`"
      :class="[
        'relative w-12 h-12 rounded-2xl flex items-center justify-center transition-all duration-200 outline-none select-none',
        service.state === 'active'
          ? 'bg-slate-800/90 text-white shadow-md shadow-sky-500/10 ring-2 ring-sky-400/80'
          : service.state === 'suspended'
          ? 'bg-slate-900/60 text-slate-400 hover:bg-slate-800 hover:text-slate-200 opacity-75 hover:opacity-100'
          : 'bg-slate-800/50 text-slate-300 hover:bg-slate-800 hover:text-white',
      ]"
    >
      <!-- Vector Brand Icon -->
      <svg
        class="w-6 h-6 transition-transform duration-200 group-hover:scale-110"
        viewBox="0 0 24 24"
        v-html="iconSvg"
      />

      <!-- Suspended / Hibernated Badge (zZ) -->
      <span
        v-if="service.state === 'suspended'"
        title="Hibernado (Ahorro de RAM)"
        class="absolute -bottom-1 -right-1 px-1 py-0.2 text-[9px] font-bold bg-amber-500/90 text-slate-950 rounded-md tracking-tighter shadow-sm pointer-events-none"
      >
        zZ
      </span>

      <!-- Unread Notification Badge -->
      <span
        v-if="service.unreadCount > 0"
        class="absolute -top-1 -right-1 min-w-[18px] h-[18px] px-1 text-[10px] font-bold bg-rose-500 text-white rounded-full flex items-center justify-center shadow-md pointer-events-none"
      >
        {{ service.unreadCount > 99 ? '99+' : service.unreadCount }}
      </span>
    </button>

    <!-- Floating Tooltip on Hover -->
    <div
      class="absolute left-[72px] z-50 invisible opacity-0 group-hover:visible group-hover:opacity-100 transition-all duration-200 pointer-events-none whitespace-nowrap bg-slate-900/95 backdrop-blur-md text-slate-200 text-xs px-3 py-1.5 rounded-lg border border-slate-700/60 shadow-xl"
    >
      <div class="font-semibold text-white flex items-center gap-1.5">
        <span>{{ service.name }}</span>
        <span
          :class="[
            'w-1.5 h-1.5 rounded-full inline-block',
            service.state === 'active'
              ? 'bg-emerald-400'
              : service.state === 'suspended'
              ? 'bg-amber-400'
              : 'bg-sky-400',
          ]"
        />
      </div>
      <div class="text-[10px] text-slate-400 mt-0.5">
        {{
          service.state === 'active'
            ? 'Activo en pantalla'
            : service.state === 'suspended'
            ? 'Hibernado (Ahorro de RAM)'
            : 'En segundo plano'
        }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { ServiceInstance } from '../types/service';
import { SERVICE_METADATA } from '../assets/serviceIcons';

const props = defineProps<{
  service: ServiceInstance;
}>();

defineEmits<{
  (e: 'select', id: string): void;
  (e: 'contextmenu', ev: MouseEvent, service: ServiceInstance): void;
}>();

const iconSvg = computed(() => {
  const meta = SERVICE_METADATA[props.service.template];
  if (meta) {
    return meta.svg;
  }
  return SERVICE_METADATA.customUrl.svg;
});
</script>
