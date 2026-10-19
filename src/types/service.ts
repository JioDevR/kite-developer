export type ServiceTemplate =
  | 'whatsApp'
  | 'telegram'
  | 'discord'
  | 'slack'
  | 'messenger'
  | 'instagram'
  | 'twitter'
  | 'gmail'
  | 'customUrl';

export type ServiceState = 'active' | 'background' | 'suspended';

export interface ServiceInstance {
  id: string;
  name: string;
  template: ServiceTemplate;
  url: string;
  icon: string;
  state: ServiceState;
  autoSuspendMinutes?: number;
  unreadCount: number;
  order: number;
}

export interface CreateServiceDto {
  name: string;
  template: ServiceTemplate;
  url?: string;
  icon?: string;
  autoSuspendMinutes?: number;
}
