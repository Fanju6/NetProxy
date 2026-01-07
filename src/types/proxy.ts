export interface ProxyStatus {
    running: boolean;
    systemProxy: boolean;
    currentNode: string | null;
    port: number;
}

export interface ProxyConfig {
    autoStart: boolean;
    defaultPort: number;
}
