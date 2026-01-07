export interface NodeInfo {
    name: string;
    protocol: string;
    address: string;
    port: number;
    fileName: string;
}

export interface NodeConfig {
    outbounds: OutboundConfig[];
}

export interface OutboundConfig {
    tag: string;
    protocol: string;
    settings?: Record<string, unknown>;
    streamSettings?: Record<string, unknown>;
}
