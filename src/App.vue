<template>
  <div class="h-screen w-full overflow-hidden select-none bg-[#0b0d13] text-slate-100 font-sans">
    <!-- View: Modal para añadir servicio (ventana flotante independiente) -->
    <AddServiceModal v-if="currentView === 'add-service'" />

    <!-- View: Pantalla de bienvenida (Webview canvas cuando no hay servicios) -->
    <WelcomeView v-else-if="currentView === 'welcome'" />

    <!-- View: Barra lateral independiente (68px fijos) -->
    <div v-else class="w-[68px] h-screen bg-[#0d1017]">
      <Sidebar />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Sidebar from './components/Sidebar.vue';
import WelcomeView from './components/WelcomeView.vue';
import AddServiceModal from './components/AddServiceModal.vue';
import { useServices } from './stores/services';

const currentView = ref('sidebar');
const { loadServices } = useServices();

onMounted(async () => {
  const params = new URLSearchParams(window.location.search);
  const viewParam = params.get('view');
  if (viewParam) {
    currentView.value = viewParam;
  }
  await loadServices();
});
</script>