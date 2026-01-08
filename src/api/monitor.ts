import type { TrafficStats, IpInfo } from '../types';

const isTauri = () => {
    return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
};

const getInvoke = async () => {
    if (!isTauri()) {
        return null;
    }
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke;
};

export async function getTrafficStats(): Promise<TrafficStats> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_traffic_stats');
    }
    // 浏览器开发环境的模拟数据
    return {
        upload_speed: 0,
        download_speed: 0,
        upload_total: 0,
        download_total: 0
    };
}

export async function getIpInfo(): Promise<IpInfo> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_ip_info');
    }
    return {
        external_ip: '127.0.0.1 (Dev)',
        internal_ip: '192.168.1.x (Dev)'
    };
}
