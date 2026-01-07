import type { NodeInfo } from '../types';

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

export async function importLink(link: string): Promise<NodeInfo> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('import_link', { link });
    }
    throw new Error('Not in Tauri environment');
}

export async function importSubscription(url: string, subName: string): Promise<NodeInfo[]> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('import_subscription', { url, subName });
    }
    return [];
}

export async function listNodes(): Promise<NodeInfo[]> {
    const invoke = await getInvoke();
    if (invoke) {
        return await invoke('list_nodes');
    }
    return [];
}

export async function deleteNode(filePath: string): Promise<void> {
    const invoke = await getInvoke();
    if (invoke) {
        await invoke('delete_node', { filePath });
    }
}
