<template>
  <div class="settings-page">
    <mdui-tabs ref="tabsRef" :value="currentTab">
      <mdui-tab value="dns">DNS 配置</mdui-tab>
      <mdui-tab value="routing">路由配置</mdui-tab>
      <mdui-tab value="logs">运行日志</mdui-tab>

      <mdui-tab-panel slot="panel" value="dns">
        <div class="panel-content">
          <mdui-card>
            <div class="card-content">
              <h3>DNS 服务器</h3>
              <div v-if="dnsConfig.servers.length > 0">
                <div 
                  v-for="(server, index) in dnsConfig.servers" 
                  :key="index"
                  class="dns-server"
                >
                  <mdui-text-field
                    :value="getServerAddress(server)"
                    @input="updateDnsServerAddress(index, $event)"
                    label="服务器地址"
                  ></mdui-text-field>
                  <mdui-button-icon @click="removeDnsServer(index)">
                    <mdui-icon name="remove_circle"></mdui-icon>
                  </mdui-button-icon>
                </div>
              </div>
              <div v-else class="empty-hint">暂无 DNS 服务器配置</div>
              <mdui-button variant="text" @click="addDnsServer">
                <mdui-icon slot="icon" name="add"></mdui-icon>
                添加服务器
              </mdui-button>
            </div>
          </mdui-card>

          <mdui-card>
            <div class="card-content">
              <h3>查询策略</h3>
              <mdui-select 
                :value="dnsConfig.queryStrategy"
                @change="handleQueryStrategyChange"
              >
                <mdui-menu-item value="UseIP">UseIP</mdui-menu-item>
                <mdui-menu-item value="UseIPv4">UseIPv4</mdui-menu-item>
                <mdui-menu-item value="UseIPv6">UseIPv6</mdui-menu-item>
              </mdui-select>
            </div>
          </mdui-card>

          <div class="actions">
            <mdui-button variant="filled" @click="saveDns">
              保存 DNS 配置
            </mdui-button>
          </div>
        </div>
      </mdui-tab-panel>

      <mdui-tab-panel slot="panel" value="routing">
        <div class="panel-content">
          <mdui-card>
            <div class="card-content">
              <h3>路由规则</h3>
              <p class="hint">路由规则用于控制流量的走向，支持按域名、IP、端口等匹配</p>
              
              <mdui-select 
                :value="routingConfig.domainStrategy"
                @change="handleDomainStrategyChange"
                label="域名策略"
              >
                <mdui-menu-item value="AsIs">AsIs</mdui-menu-item>
                <mdui-menu-item value="IPIfNonMatch">IPIfNonMatch</mdui-menu-item>
                <mdui-menu-item value="IPOnDemand">IPOnDemand</mdui-menu-item>
              </mdui-select>

              <div class="rules-count">
                当前规则数: {{ routingConfig.rules.length }}
              </div>
            </div>
          </mdui-card>

          <div class="actions">
            <mdui-button variant="filled" @click="saveRouting">
              保存路由配置
            </mdui-button>
          </div>
        </div>
      </mdui-tab-panel>
      
      <mdui-tab-panel slot="panel" value="logs">
        <div class="panel-content">
          <mdui-card class="log-card">
            <div class="card-header">
                <h3>运行日志</h3>
                <div class="log-controls">
                    <mdui-segment-button-group :value="logType" @change="handleLogTypeChange">
                        <mdui-segment-button value="access">Access</mdui-segment-button>
                        <mdui-segment-button value="error">Error</mdui-segment-button>
                    </mdui-segment-button-group>
                    <mdui-button-icon icon="refresh" @click="refreshLog"></mdui-button-icon>
                </div>
            </div>
            <div class="log-content">
                <pre>{{ logContent }}</pre>
            </div>
          </mdui-card>
        </div>
      </mdui-tab-panel>
    </mdui-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { getDnsConfig, saveDnsConfig, getRoutingConfig, saveRoutingConfig, getXrayLog } from '../api';
import type { DnsConfig, RoutingConfig, DnsServer } from '../types';

const currentTab = ref('dns');
const tabsRef = ref<HTMLElement | null>(null);

const dnsConfig = ref<DnsConfig>({
  tag: 'dns-module',
  queryStrategy: 'UseIPv4',
  disableFallbackIfMatch: true,
  hosts: {},
  servers: []
});

const routingConfig = ref<RoutingConfig>({
  domainStrategy: 'AsIs',
  rules: []
});

// 日志相关
const logType = ref<'access' | 'error'>('access');
const logContent = ref('');
let logInterval: number | null = null;

const handleTabChange = (event: Event) => {
  const target = event.target as any;
  if (target && target.value) {
    currentTab.value = target.value;
  }
};

const handleLogTypeChange = (event: Event) => {
    const target = event.target as any;
    if (target && target.value) {
        logType.value = target.value;
        refreshLog();
    }
};

const refreshLog = async () => {
    try {
        logContent.value = await getXrayLog(logType.value);
    } catch (error) {
        console.error('获取日志失败:', error);
    }
};

const startLogAutoRefresh = () => {
    stopLogAutoRefresh();
    refreshLog();
    logInterval = window.setInterval(refreshLog, 3000);
};

const stopLogAutoRefresh = () => {
    if (logInterval) {
        window.clearInterval(logInterval);
        logInterval = null;
    }
};

watch(currentTab, (newTab) => {
    if (newTab === 'logs') {
        startLogAutoRefresh();
    } else {
        stopLogAutoRefresh();
    }
});

const loadConfigs = async () => {
  try {
    const dns = await getDnsConfig();
    if (dns) {
      dnsConfig.value = dns;
    }
  } catch (error) {
    console.error('加载 DNS 配置失败:', error);
  }
  
  try {
    const routing = await getRoutingConfig();
    if (routing) {
      routingConfig.value = routing;
    }
  } catch (error) {
    console.error('加载路由配置失败:', error);
  }
};

const getServerAddress = (server: DnsServer | string): string => {
  if (typeof server === 'string') {
    return server;
  }
  return server.address || '';
};

const updateDnsServerAddress = (index: number, event: Event) => {
  const target = event.target as HTMLInputElement;
  const server = dnsConfig.value.servers[index];
  if (typeof server === 'string') {
    dnsConfig.value.servers[index] = { address: target.value } as DnsServer;
  } else {
    server.address = target.value;
  }
};

const handleQueryStrategyChange = (event: Event) => {
  const target = event.target as any;
  if (target && target.value) {
    dnsConfig.value.queryStrategy = target.value;
  }
};

const handleDomainStrategyChange = (event: Event) => {
  const target = event.target as any;
  if (target && target.value) {
    routingConfig.value.domainStrategy = target.value;
  }
};

const addDnsServer = () => {
  dnsConfig.value.servers.push({ address: '' });
};

const removeDnsServer = (index: number) => {
  dnsConfig.value.servers.splice(index, 1);
};

const saveDns = async () => {
  try {
    await saveDnsConfig(dnsConfig.value);
    console.log('DNS 配置保存成功');
  } catch (error) {
    console.error('保存 DNS 配置失败:', error);
  }
};

const saveRouting = async () => {
  try {
    await saveRoutingConfig(routingConfig.value);
    console.log('路由配置保存成功');
  } catch (error) {
    console.error('保存路由配置失败:', error);
  }
};

onMounted(() => {
  loadConfigs();
  
  if (tabsRef.value) {
    tabsRef.value.addEventListener('change', handleTabChange);
    // 监听 segment button 的 change 事件需要单独处理，或者直接使用 @change
    // 因为 segment button 在 panel 内部，而上面的监听是 tab 容器
  }
});

onUnmounted(() => {
  stopLogAutoRefresh();
  if (tabsRef.value) {
    tabsRef.value.removeEventListener('change', handleTabChange);
  }
});
</script>

<style scoped>
.settings-page {
  padding: 24px;
}

.panel-content {
  padding: 24px 0;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.card-content {
  padding: 20px;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0px;
    padding: 16px 20px 0 20px;
}

.log-controls {
    display: flex;
    align-items: center;
    gap: 8px; 
}

.log-content {
    padding: 20px;
    height: 400px;
    overflow-y: auto;
    background: #1e1e1e; /* Dark background for logs */
    color: #d4d4d4;
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 13px;
    margin: 0 20px 20px 20px;
    border-radius: 4px;
}

.log-content pre {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-all;
}

.log-card {
    display: flex;
    flex-direction: column;
}

.card-content h3, .card-header h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 500;
}

.hint {
  color: var(--mdui-color-on-surface-variant);
  font-size: 14px;
  margin-bottom: 16px;
}

.empty-hint {
  color: var(--mdui-color-on-surface-variant);
  font-size: 14px;
  padding: 12px 0;
}

.dns-server {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.dns-server mdui-text-field {
  flex: 1;
}

.rules-count {
  margin-top: 16px;
  padding: 12px;
  background: var(--mdui-color-surface-variant);
  border-radius: 8px;
  font-size: 14px;
}

.actions {
  display: flex;
  justify-content: flex-end;
  padding-top: 16px;
}
</style>
