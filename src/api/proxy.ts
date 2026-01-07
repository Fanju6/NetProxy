import type { ProxyStatus } from '../types';

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

export async function startProxy(nodeFile: string): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('start_proxy', { nodeFile });
    }
}

export async function stopProxy(): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('stop_proxy');
    }
}

export async function getProxyStatus(): Promise<ProxyStatus> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_proxy_status');
    }
    // 返回默认状态
    return {
        running: false,
        systemProxy: false,
        currentNode: null,
        port: 10808
    };
}

export async function setSystemProxy(enable: boolean, port: number): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('set_system_proxy', { enable, port });
    }
}

export async function selectNode(nodeFile: string): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('select_node', { nodeFile });
    }
}

export async function getSelectedNode(): Promise<string | null> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('get_selected_node');
    }
    return null;
}
