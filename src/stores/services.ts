import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ServiceInstance, CreateServiceDto } from '../types/service';

const services = ref<ServiceInstance[]>([]);
const loading = ref(false);
const isAddModalOpen = ref(false);

export function useServices() {
  const activeService = computed(() =>
    services.value.find((s) => s.state === 'active')
  );

  const activeServiceId = computed(() => activeService.value?.id || null);

  const totalUnread = computed(() =>
    services.value.reduce((acc, s) => acc + (s.unreadCount || 0), 0)
  );

  const ramSavedEstimateMB = computed(() => {
    // Estimación: cada servicio suspendido ahorra entre 150MB y 350MB de RAM
    const suspendedCount = services.value.filter(
      (s) => s.state === 'suspended'
    ).length;
    return suspendedCount * 250;
  });

  async function loadServices() {
    loading.value = true;
    try {
      const data = await invoke<ServiceInstance[]>('get_services');
      services.value = data;
    } catch (e) {
      console.error('Error cargando servicios:', e);
    } finally {
      loading.value = false;
    }
  }

  async function createService(dto: CreateServiceDto) {
    try {
      const newService = await invoke<ServiceInstance>('create_service', {
        input: dto,
      });
      return newService;
    } catch (e) {
      console.error('Error creando servicio:', e);
      throw e;
    }
  }

  async function activateService(serviceId: string) {
    try {
      const updated = await invoke<ServiceInstance[]>('activate_service', {
        serviceId,
      });
      services.value = updated;
    } catch (e) {
      console.error('Error activando servicio:', e);
    }
  }

  async function suspendService(serviceId: string) {
    try {
      const updated = await invoke<ServiceInstance[]>('suspend_service', {
        serviceId,
      });
      services.value = updated;
    } catch (e) {
      console.error('Error suspendiendo servicio:', e);
    }
  }

  async function deleteService(serviceId: string) {
    try {
      const updated = await invoke<ServiceInstance[]>('delete_service', {
        serviceId,
      });
      services.value = updated;
    } catch (e) {
      console.error('Error eliminando servicio:', e);
    }
  }

  async function reloadService(serviceId: string, clearCache = false) {
    try {
      await invoke('reload_service', { serviceId, clearCache });
    } catch (e) {
      console.error('Error recargando servicio:', e);
    }
  }

  async function openAddModal() {
    try {
      await invoke('open_add_service_modal');
    } catch (e) {
      console.error('Error abriendo ventana de añadir servicio:', e);
    }
  }

  async function closeAddModal() {
    try {
      await invoke('close_add_service_modal');
    } catch (e) {
      console.error(e);
    }
  }

  async function openContextMenu(e: MouseEvent, service: ServiceInstance) {
    e.preventDefault();
    try {
      await invoke('show_service_context_menu', { serviceId: service.id });
    } catch (err) {
      console.error('Error mostrando menu contextual nativo:', err);
    }
  }

  return {
    services,
    loading,
    activeService,
    activeServiceId,
    totalUnread,
    ramSavedEstimateMB,
    isAddModalOpen,
    loadServices,
    createService,
    activateService,
    suspendService,
    deleteService,
    reloadService,
    openAddModal,
    closeAddModal,
    openContextMenu,
  };
}

// Inicializar un listener único de eventos Tauri a nivel de módulo
let isListenerInitialized = false;
if (!isListenerInitialized && typeof window !== 'undefined') {
  isListenerInitialized = true;
  listen<ServiceInstance[]>('services_updated', (event) => {
    services.value = event.payload;
  });
}
