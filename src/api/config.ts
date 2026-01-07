import type { DnsConfig, RoutingConfig } from '../types';

// 检查是否在 Tauri 环境中
const isTauri = () => {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
};

// 动态导入 invoke
const getInvoke = async () => {
    if (!isTauri()) {
        return null;
    }
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke;
};

export async function getDnsConfig(): Promise<DnsConfig> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_dns_config');
    }
    return {
        tag: 'dns-module',
        queryStrategy: 'UseIPv4',
        disableFallbackIfMatch: true,
        hosts: {},
        servers: []
    };
}

export async function saveDnsConfig(config: DnsConfig): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('save_dns_config', { config });
    }
}

export async function getRoutingConfig(): Promise<RoutingConfig> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_routing_config');
    }
    return {
        domainStrategy: 'AsIs',
        rules: []
    };
}

export async function saveRoutingConfig(config: RoutingConfig): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('save_routing_config', { config });
    }
}

export async function getXrayLog(logType: 'access' | 'error'): Promise<string> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_xray_log', { logType });
    }
    return '';
}
