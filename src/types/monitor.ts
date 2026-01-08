export interface TrafficStats {
    upload_speed: number;
    download_speed: number;
    upload_total: number;
    download_total: number;
}

export interface IpInfo {
    external_ip: string | null;
    internal_ip: string | null;
}
