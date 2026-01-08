export interface DnsConfig {
    tag: string;
    queryStrategy: string;
    disableFallbackIfMatch: boolean;
    hosts: Record<string, string>;
    servers: (string | DnsServer)[];
}

export interface DnsServer {
    address: string;
    port?: number;
    domains?: string[];
    expectIPs?: string[];
}

export interface RoutingConfig {
    domainStrategy: string;
    rules: RoutingRule[];
}

export interface RoutingRule {
    type: string;
    inboundTag?: string[];
    outboundTag: string;
    domain?: string[];
    ip?: string[];
    port?: string;
}
