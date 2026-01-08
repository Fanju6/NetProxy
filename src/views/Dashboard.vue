<template>
  <div class="dashboard-container">
    <div class="dashboard-grid">
      <!-- Network Speed (Full Width, Height 2) -->
      <div class="common-card full-width height-2 speed-card" id="card-speed">
        <div class="card-header">
          <mdui-icon name="speed" class="card-icon"></mdui-icon>
          <span class="card-title">网络速度</span>
          <div class="header-speed-text">
            <span>{{ formatSpeed(traffic.download) }}/s</span>
            <span style="opacity: 0.5; margin: 0 4px">|</span>
            <span style="opacity: 0.7">{{ formatSpeed(traffic.upload) }}/s</span>
          </div>
        </div>
        <div class="card-content">
          <div class="chart-area" id="speed-chart-container">
             <!-- Mock Chart -->
             <div class="mock-chart-bars">
                <div v-for="(val, index) in chartData" :key="index" class="bar" :style="{ height: Math.min(100, (val / 1024 / 1024) * 100) + '%' }"></div>
             </div>
          </div>
        </div>
      </div>

      <!-- Left Col: Outbound Mode (Height 2) -->
      <div class="common-card half-width height-2 mode-card" id="card-mode">
        <div class="card-header">
          <mdui-icon name="call_split" class="card-icon"></mdui-icon>
          <span class="card-title">出站模式</span>
        </div>
        <div class="card-content">
          <div class="mode-options">
            <div 
              class="mode-option" 
              :class="{ active: outboundMode === 'rule' }"
              @click="setOutboundMode('rule')"
            >
              <div class="mode-radio"></div>
              <span class="mode-label">规则</span>
            </div>
            <div 
              class="mode-option" 
              :class="{ active: outboundMode === 'global' }"
              @click="setOutboundMode('global')"
            >
              <div class="mode-radio"></div>
              <span class="mode-label">全局</span>
            </div>
            <div 
              class="mode-option" 
              :class="{ active: outboundMode === 'direct' }"
              @click="setOutboundMode('direct')"
            >
              <div class="mode-radio"></div>
              <span class="mode-label">直连</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right Col: Network Detection (Height 1) -->
      <div class="common-card half-width height-1" id="card-external-ip">
        <div class="card-header">
          <mdui-icon name="network_check" class="card-icon"></mdui-icon>
          <span class="card-title">网络检测</span>
        </div>
        <div class="card-content">
          <div class="card-value">{{ ips.external }}</div>
        </div>
      </div>

      <!-- Right Col: Traffic Stats (Height 2) -->
      <div class="common-card half-width height-2 traffic-card" id="card-traffic">
        <div class="card-header">
          <mdui-icon name="data_saver_off" class="card-icon"></mdui-icon>
          <span class="card-title">流量统计</span>
        </div>
        <div class="card-content">
          <div class="traffic-donut-container">
            <div class="donut-chart-wrapper">
              <svg viewBox="0 0 100 100" class="active">
                <circle cx="50" cy="50" r="35" fill="none" stroke="var(--mdui-color-secondary)" stroke-width="10" stroke-dasharray="109.96 219.91" transform="rotate(-90 50 50)"></circle>
                <circle cx="50" cy="50" r="35" fill="none" stroke="var(--mdui-color-primary)" stroke-width="10" stroke-dasharray="109.96 219.91" stroke-dashoffset="-109.96" transform="rotate(-90 50 50)"></circle>
              </svg>
            </div>
            <div class="traffic-legend">
              <div class="legend-item">
                <span class="legend-color upload"></span>
                <span>上传</span>
              </div>
              <div class="legend-item">
                <span class="legend-color download"></span>
                <span>下载</span>
              </div>
            </div>
          </div>
          <div class="traffic-stats">
            <div class="traffic-stat-row">
              <span class="stat-label">
                <mdui-icon name="arrow_upward" style="font-size: 12px; color: var(--mdui-color-primary);"></mdui-icon>
                <span>上传</span>
              </span>
              <span><span class="stat-value">{{ formatTraffic(traffic.uploadTotal) }}</span></span>
            </div>
            <div class="traffic-stat-row">
              <span class="stat-label">
                <mdui-icon name="arrow_downward" style="font-size: 12px; color: var(--mdui-color-secondary);"></mdui-icon>
                <span>下载</span>
              </span>
              <span><span class="stat-value">{{ formatTraffic(traffic.downloadTotal) }}</span></span>
            </div>
          </div>
        </div>
      </div>

       <!-- Left Col: Internal IP (Height 1) - Grid Flow places later -->
      <div class="common-card half-width height-1" id="card-internal-ip">
        <div class="card-header">
          <mdui-icon name="devices" class="card-icon"></mdui-icon>
          <span class="card-title">内网IP</span>
        </div>
        <div class="card-content">
          <div class="card-value">{{ ips.internal }}</div>
        </div>
      </div>
    </div>
    
    <!-- FAB for Service Control -->
    <!-- FAB for Service Control -->
    <div class="dashboard-fab" :class="{ running: status.running }">
        <mdui-fab 
            id="service-fab" 
            :icon="status.running ? 'stop' : 'play_arrow'" 
            variant="primary" 
            :extended="status.running"
            @click="toggleProxy"
        >
            <span slot="label" v-if="status.running">{{ runtime }}</span>
            <span slot="label" v-else>启动</span>
        </mdui-fab>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { getProxyStatus, startProxy, stopProxy, setSystemProxy, getTrafficStats, getIpInfo } from '../api';
import type { ProxyStatus } from '../types';

const status = ref<ProxyStatus>({
  running: false,
  systemProxy: false,
  currentNode: null,
  port: 10808
});

const traffic = ref({
  upload: 0,
  download: 0,
  uploadTotal: 0,
  downloadTotal: 0
});

const ips = ref({
    external: 'Checking...',
    internal: 'Checking...'
});

const runtime = ref('00:00:00');
const startTime = ref<number | null>(null);

const outboundMode = ref('rule');

const setOutboundMode = (mode: string) => {
  outboundMode.value = mode;
  // TODO: Call API
};

const formatSpeed = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

const formatTraffic = (bytes: number) => {
    return formatSpeed(bytes); 
}

// Chart data
const chartData = ref<number[]>(new Array(30).fill(0));

let trafficTimer: number;
let runtimeTimer: number;

const fetchTraffic = async () => {
    try {
        const stats = await getTrafficStats();
        traffic.value = {
            upload: stats.upload_speed,
            download: stats.download_speed,
            uploadTotal: stats.upload_total,
            downloadTotal: stats.download_total
        };

        // Update chart
        chartData.value.push(stats.download_speed);
        chartData.value.shift();
    } catch (e) {
        console.error('Failed to fetch traffic stats', e);
    }
};

const updateRuntime = () => {
    if (status.value.running && startTime.value) {
        const diff = Math.floor((Date.now() - startTime.value) / 1000);
        const h = Math.floor(diff / 3600).toString().padStart(2, '0');
        const m = Math.floor((diff % 3600) / 60).toString().padStart(2, '0');
        const s = (diff % 60).toString().padStart(2, '0');
        runtime.value = `${h}:${m}:${s}`;
    } else {
        runtime.value = '00:00:00';
    }
}

const toggleProxy = async () => {
  try {
    if (status.value.running) {
      await stopProxy();
      if (status.value.systemProxy) await setSystemProxy(false, status.value.port);
      startTime.value = null;
    } else if (status.value.currentNode) {
      await startProxy(status.value.currentNode);
      startTime.value = Date.now();
    } else {
        console.warn('Nodes logic waiting');
    }
    await refreshStatus();
  } catch (error) {
    console.error('代理切换失败:', error);
  }
};

const refreshStatus = async () => {
  try { status.value = await getProxyStatus(); } catch (e) { console.error(e); }
};

onMounted(async () => {
  await refreshStatus();
  if (status.value.running) {
      // If already running, start timer from now (approximate)
      if (!startTime.value) startTime.value = Date.now();
  }
  
  trafficTimer = window.setInterval(fetchTraffic, 1000);
  runtimeTimer = window.setInterval(updateRuntime, 1000);

  // Fetch IPs
  const ipInfo = await getIpInfo();
  ips.value.external = ipInfo.external_ip || 'Unavailable';
  ips.value.internal = ipInfo.internal_ip || 'Unavailable';
});

onUnmounted(() => {
  clearInterval(trafficTimer);
  clearInterval(runtimeTimer);
});
</script>

<style scoped>
/* Copied and adapted from dashboard.css */
:root {
    --card-radius: 14px;
    --card-padding: 12px;
    --widget-height-1: 76px;
    --widget-height-2: calc(var(--widget-height-1) * 2 + 12px);
    --grid-gap: 12px;
}

.dashboard-container {
    /* To allow FAB absolute positioning inside if needed, or fixed relative to window */
    min-height: 100%;
}

.dashboard-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-template-rows: auto auto auto auto;
    gap: 12px; /* var(--grid-gap) */
    padding: 16px;
    padding-bottom: 100px;
    max-width: 560px;
    margin: 0 auto;
}

/* Grid Layout definitions specific to reference */
.speed-card { grid-column: 1 / -1; }
.mode-card { grid-column: 1; grid-row: 2 / 4; }
.dashboard-grid > .common-card:nth-child(3) { grid-column: 2; grid-row: 2; } /* External IP */
.dashboard-grid > .common-card:nth-child(5) { grid-column: 1; grid-row: 4; } /* Internal IP - was 4th in ref but we have traffic card */
.traffic-card { grid-column: 2; grid-row: 3 / 5; }

/* NOTE: nth-child might be fragile if V-IF removes elements. 
   Better to use classes for grid-row/col if possible, but copying ref logic for now. 
   Ref has: 1: speed, 2: mode, 3: ext-ip, 4: int-ip, 5: traffic, 6/7: cpu/mem hidden.
   My template order: Speed, Mode, ExtIP, Traffic, IntIP. 
   So:
   1: Speed -> 1/-1
   2: Mode -> 1, row 2/4
   3: ExtIP -> 2, row 2
   4: Traffic -> 2, row 3/5
   5: IntIP -> 1, row 4
*/

/* Re-applying explicit grid positions to match my template order */
#card-speed { grid-column: 1 / -1; }
#card-mode { grid-column: 1; grid-row: 2 / 4; }
#card-external-ip { grid-column: 2; grid-row: 2 / 3; }
#card-traffic { grid-column: 2; grid-row: 3 / 5; }
#card-internal-ip { grid-column: 1; grid-row: 4 / 5; }

/* Common Card */
.common-card {
    background: var(--monet-surface-container-low, var(--mdui-color-surface-container-low));
    border-radius: 14px; /* var(--card-radius) */
    border: 1px solid var(--monet-surface-container-highest, var(--mdui-color-surface-container-highest));
    overflow: hidden;
    display: flex;
    flex-direction: column;
    transition: border-color 0.2s, transform 0.15s;
    cursor: pointer;
}

.common-card:hover {
    border-color: var(--monet-primary, var(--mdui-color-primary));
    transform: translateY(-1px);
}

.height-1 { height: 76px; }
.height-2 { height: 164px; /* 76*2 + 12 */ }

/* Headers */
.card-header {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    gap: 8px;
    padding: 12px 12px;
    padding-bottom: 0;
    flex-shrink: 0;
    margin-bottom: 0;
}

.card-icon {
    font-size: 24px;
    color: var(--monet-on-surface-variant, var(--mdui-color-on-surface-variant));
}

.card-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--monet-on-surface-variant, var(--mdui-color-on-surface-variant));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.header-speed-text {
    margin-left: auto;
    font-size: 11px;
    font-weight: 500;
    color: var(--monet-on-surface-variant, var(--mdui-color-on-surface-variant));
    opacity: 0.8;
}

/* Content */
.card-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 0 12px 10px 12px;
    overflow: hidden;
}

.card-value {
    font-size: 14px;
    font-weight: 500;
    color: var(--monet-on-surface, var(--mdui-color-on-surface));
}

/* Mode Card */
.mode-card .card-content {
    padding: 4px 8px 8px 8px;
    justify-content: flex-start;
}
.mode-options { display: flex; flex-direction: column; }
.mode-option {
    display: flex; align-items: center; gap: 8px;
    padding: 8px 10px;
    border-radius: 8px;
    cursor: pointer;
    transition: background 0.15s;
}
.mode-option:hover { background: var(--monet-surface-container-high, var(--mdui-color-surface-container-high)); }
.mode-radio {
    width: 18px; height: 18px;
    border: 2px solid var(--monet-outline, var(--mdui-color-outline));
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
}
.mode-option.active .mode-radio { border-color: var(--monet-primary, var(--mdui-color-primary)); }
.mode-option.active .mode-radio::after {
    content: ''; width: 9px; height: 9px;
    background: var(--monet-primary, var(--mdui-color-primary)); border-radius: 50%;
}
.mode-label { font-size: 14px; font-weight: 500; color: var(--monet-on-surface, var(--mdui-color-on-surface)); }

/* Traffic Card */
.traffic-donut-container {
    display: flex; align-items: center; gap: 8px; flex: 1; min-height: 0;
}
.donut-chart-wrapper { width: 50px; height: 50px; flex-shrink: 0; }
.traffic-legend { display: flex; flex-direction: column; gap: 3px; }
.legend-item { display: flex; align-items: center; gap: 4px; font-size: 11px; color: var(--monet-on-surface-variant, var(--mdui-color-on-surface-variant)); }
.legend-color { width: 16px; height: 6px; border-radius: 3px; }
.legend-color.upload { background: var(--monet-primary, var(--mdui-color-primary)); }
.legend-color.download { background: var(--monet-secondary, var(--mdui-color-secondary)); }

.traffic-stats { display: flex; flex-direction: column; gap: 4px; }
.traffic-stat-row { display: flex; align-items: center; justify-content: space-between; font-size: 11px; }
.stat-label { display: flex; align-items: center; gap: 4px; color: var(--monet-on-surface, var(--mdui-color-on-surface)); }
.stat-value { font-size: 11px; font-family: monospace; }
/* Speed Card / Mock Chart */
.speed-card { height: 160px; }
.speed-card .card-content { padding: 0; position: relative; }
.chart-area {
    position: absolute; left: 0; right: 0; bottom: 0; top: 8px;
    display: flex; align-items: flex-end; gap: 2px; padding: 0 12px 12px 12px;
}
.mock-chart-bars {
    width: 100%; height: 100%; display: flex; align-items: flex-end; gap: 4px; opacity: 0.2;
}
.bar { flex: 1; background: var(--monet-primary, var(--mdui-color-primary)); border-radius: 2px; transition: height 0.5s; }

/* FAB */
.dashboard-fab {
    position: fixed;
    right: 24px; bottom: 88px; z-index: 100;
}
#service-fab {
    transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}
.dashboard-fab.running #service-fab {
    padding-left: 16px; padding-right: 16px; 
    /* width expansion handled by mdui-fab extended behavior usually, but we force it */
}
.fab-runtime { margin-left: 8px; font-weight: 500; font-size: 14px; }
</style>
