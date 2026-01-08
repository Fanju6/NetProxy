<template>
  <div class="settings-page">
    <mdui-tabs ref="tabsRef" :value="currentTab">
      <mdui-tab value="dns" icon="dns">DNS 配置</mdui-tab>
      <mdui-tab value="routing" icon="route">路由配置</mdui-tab>
      <mdui-tab value="appearance" icon="palette">外观</mdui-tab>
      <mdui-tab value="logs" icon="terminal">运行日志</mdui-tab>

      <mdui-tab-panel slot="panel" value="dns">
        <div class="panel-content">
          <!-- DNS 服务器配置 -->
          <mdui-card>
            <div class="card-content">
              <h3>DNS 服务器</h3>
              <div class="dns-list">
                <div 
                  v-for="(server, index) in dnsConfig.servers" 
                  :key="index"
                  class="dns-item"
                >
                  <mdui-icon name="dns" class="item-icon"></mdui-icon>
                  <div class="item-info">
                    <span class="server-addr">{{ getServerDisplay(server) }}</span>
                    <div class="server-domains" v-if="getDomains(server).length">
                      <span class="label">绑定域名: </span>
                      <span v-for="domain in getDomains(server)" :key="domain" class="domain-tag">{{ domain }}</span>
                    </div>
                  </div>
                  <mdui-button-icon icon="edit" @click="editDnsServer(index)"></mdui-button-icon>
                  <mdui-button-icon icon="delete" @click="removeDnsServer(index)"></mdui-button-icon>
                </div>
              </div>
              
              <div class="add-btn-container">
                <mdui-button variant="tonal" @click="addDnsServer">
                  <mdui-icon slot="icon" name="add"></mdui-icon>
                  添加 DNS 服务器
                </mdui-button>
              </div>
            </div>
          </mdui-card>

          <!-- DNS 策略 -->
          <mdui-card>
            <div class="card-content">
              <h3>查询策略</h3>
              <p class="hint">当同时配置了 IPv4 和 IPv6 时，决定如何查询 DNS</p>
              <mdui-select 
                :value="dnsConfig.queryStrategy"
                @change="handleQueryStrategyChange"
              >
                <mdui-menu-item value="UseIP">UseIP (双栈并发)</mdui-menu-item>
                <mdui-menu-item value="UseIPv4">UseIPv4 (优先 IPv4)</mdui-menu-item>
                <mdui-menu-item value="UseIPv6">UseIPv6 (优先 IPv6)</mdui-menu-item>
              </mdui-select>
              
              <div class="setting-item">
                <mdui-checkbox 
                  :checked="dnsConfig.disableFallbackIfMatch"
                  @change="dnsConfig.disableFallbackIfMatch = ($event.target as any).checked"
                >
                  如果匹配则禁用回退 (disableFallbackIfMatch)
                </mdui-checkbox>
              </div>
            </div>
          </mdui-card>

          <div class="actions">
            <mdui-button variant="filled" @click="saveDns">保存 DNS 配置</mdui-button>
          </div>
        </div>
      </mdui-tab-panel>

      <mdui-tab-panel slot="panel" value="routing">
        <div class="panel-content">
          <mdui-card>
            <div class="card-content">
              <h3>路由策略</h3>
              <mdui-select 
                :value="routingConfig.domainStrategy"
                @change="handleDomainStrategyChange"
                label="域名解析策略"
              >
                <mdui-menu-item value="AsIs">AsIs (只解析域名)</mdui-menu-item>
                <mdui-menu-item value="IPIfNonMatch">IPIfNonMatch (无匹配时解析 IP)</mdui-menu-item>
                <mdui-menu-item value="IPOnDemand">IPOnDemand (按需解析 IP)</mdui-menu-item>
              </mdui-select>
            </div>
          </mdui-card>

          <mdui-card>
            <div class="card-content">
              <div class="card-header-row">
                <h3>路由规则</h3>
                <mdui-button variant="tonal" @click="addRoutingRule">
                  <mdui-icon slot="icon" name="add"></mdui-icon>
                  添加规则
                </mdui-button>
              </div>
              
              <div class="rules-list">
                <div v-for="(rule, index) in routingConfig.rules" :key="index" class="rule-item">
                  <div class="rule-priority">{{ index + 1 }}</div>
                  <div class="rule-content">
                    <div class="rule-main">
                      <span class="tag type-tag">{{ rule.type }}</span>
                      <span class="arrow">➔</span>
                      <span class="tag outbound-tag">{{ rule.outboundTag }}</span>
                    </div>
                    <div class="rule-details">
                      <span v-if="rule.domain && rule.domain.length" class="detail-text">Domain: {{ rule.domain.length }} 个</span>
                      <span v-if="rule.ip && rule.ip.length" class="detail-text">IP: {{ rule.ip.length }} 个</span>
                      <span v-if="rule.port" class="detail-text">Port: {{ rule.port }}</span>
                    </div>
                  </div>
                  <div class="rule-actions">
                    <mdui-button-icon icon="arrow_upward" @click="moveRule(index, -1)" :disabled="index === 0"></mdui-button-icon>
                    <mdui-button-icon icon="arrow_downward" @click="moveRule(index, 1)" :disabled="index === routingConfig.rules.length - 1"></mdui-button-icon>
                    <mdui-button-icon icon="edit" @click="editRoutingRule(index)"></mdui-button-icon>
                    <mdui-button-icon icon="delete" @click="removeRoutingRule(index)"></mdui-button-icon>
                  </div>
                </div>
              </div>
            </div>
          </mdui-card>

          <div class="actions">
            <mdui-button variant="filled" @click="saveRouting">保存路由配置</mdui-button>
          </div>
        </div>
      </mdui-tab-panel>

      <mdui-tab-panel slot="panel" value="appearance">
        <div class="panel-content">
          <mdui-card>
            <div class="card-content">
              <h3>主题模式</h3>
              <p class="hint">选择应用显示的颜色模式</p>
              
              <div class="theme-options">
                <mdui-segmented-button-group :value="currentTheme" @change="handleThemeChange" selects="single">
                  <mdui-segmented-button value="auto" icon="brightness_auto">自动</mdui-segmented-button>
                  <mdui-segmented-button value="light" icon="light_mode">浅色</mdui-segmented-button>
                  <mdui-segmented-button value="dark" icon="dark_mode">深色</mdui-segmented-button>
                </mdui-segmented-button-group>
              </div>

              <div class="divider"></div>

              <h3>主题颜色</h3>
              <p class="hint">自定义应用强调色</p>
              <div class="color-palette">
                <div 
                  v-for="color in themeColors" 
                  :key="color.color"
                  class="color-item"
                  :class="{ selected: currentColor === color.color }"
                  :style="{ backgroundColor: color.color }"
                  @click="handleColorChange(color.color)"
                  :title="color.name"
                >
                  <mdui-icon name="check" class="check-icon" v-if="currentColor === color.color"></mdui-icon>
                </div>
              </div>
            </div>
          </mdui-card>
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
                <pre>{{ logContent || '暂无日志' }}</pre>
            </div>
          </mdui-card>
        </div>
      </mdui-tab-panel>
    </mdui-tabs>

    <!-- Dialogs -->
    <!-- DNS Server Dialog -->
    <mdui-dialog :open="dnsDialogOpen" @closed="dnsDialogOpen = false" close-on-overlay-click class="config-dialog">
      <div class="dialog-header">
        <h3>{{ editingDnsIndex === -1 ? '添加 DNS 服务器' : '编辑 DNS 服务器' }}</h3>
      </div>
      <div class="dialog-content">
        <mdui-text-field label="服务器地址 (IP)" v-model="editDnsForm.address" class="full-width"></mdui-text-field>
        <mdui-text-field label="端口 (可选，默认53)" v-model.number="editDnsForm.port" type="number" class="full-width"></mdui-text-field>
        <p class="hint">指定域名 (每行一个)</p>
        <mdui-text-field 
          rows="3" 
          v-model="editDnsForm.domainsText" 
          placeholder="geosite:google&#10;domain:example.com"
          class="full-width"
        ></mdui-text-field>
      </div>
      <div class="dialog-actions">
        <mdui-button variant="text" @click="dnsDialogOpen = false">取消</mdui-button>
        <mdui-button @click="confirmDnsServer">确定</mdui-button>
      </div>
    </mdui-dialog>

    <!-- Routing Rule Dialog -->
    <mdui-dialog :open="routingDialogOpen" @closed="routingDialogOpen = false" close-on-overlay-click class="config-dialog">
      <div class="dialog-header">
        <h3>{{ editingRuleIndex === -1 ? '添加路由规则' : '编辑路由规则' }}</h3>
      </div>
      <div class="dialog-content">
        <mdui-select label="规则类型" :value="editRuleForm.type" @change="editRuleForm.type = ($event.target as any).value" class="full-width">
          <mdui-menu-item value="field">Field (基于字段)</mdui-menu-item>
        </mdui-select>
        
        <mdui-text-field label="出站标签 (Outbound Tag)" v-model="editRuleForm.outboundTag" class="full-width"></mdui-text-field>
        
        <div class="form-group">
            <p class="label">域名列表 (每行一个)</p>
            <mdui-text-field 
              rows="3" 
              v-model="editRuleForm.domainsText" 
              placeholder="geosite:google&#10;domain:example.com"
              class="full-width"
            ></mdui-text-field>
        </div>

        <div class="form-group">
            <p class="label">IP 列表 (每行一个)</p>
            <mdui-text-field 
              rows="3" 
              v-model="editRuleForm.ipsText" 
              placeholder="geoip:cn&#10;192.168.0.0/24"
              class="full-width"
            ></mdui-text-field>
        </div>

        <mdui-text-field label="端口范围 (如 80,443)" v-model="editRuleForm.port" class="full-width"></mdui-text-field>
      </div>
      <div class="dialog-actions">
        <mdui-button variant="text" @click="routingDialogOpen = false">取消</mdui-button>
        <mdui-button @click="confirmRoutingRule">确定</mdui-button>
      </div>
    </mdui-dialog>

  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { getDnsConfig, saveDnsConfig, getRoutingConfig, saveRoutingConfig, getXrayLog } from '../api';
import type { DnsConfig, RoutingConfig, DnsServer, RoutingRule } from '../types';
import { useTheme } from '../composables/theme';

const currentTab = ref('dns');
const tabsRef = ref<HTMLElement | null>(null);

const { currentTheme, currentColor, themeColors, setThemeMode, setThemeColor } = useTheme();

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

// Logs
const logType = ref<'access' | 'error'>('access');
const logContent = ref('');
let logInterval: number | null = null;

// DNS Edit State
const dnsDialogOpen = ref(false);
const editingDnsIndex = ref(-1);
const editDnsForm = ref({ address: '', port: undefined as number | undefined, domainsText: '' });

// Routing Edit State
const routingDialogOpen = ref(false);
const editingRuleIndex = ref(-1);
const editRuleForm = ref({ type: 'field', outboundTag: 'proxy', domainsText: '', ipsText: '', port: '' });

const handleTabChange = (event: Event) => {
  const target = event.target as any;
  if (target && target.value) currentTab.value = target.value;
};

// DNS Helpers
const getServerDisplay = (server: string | DnsServer) => {
  if (typeof server === 'string') return server;
  return `${server.address}${server.port ? ':' + server.port : ''}`;
};
const getDomains = (server: string | DnsServer) => {
  if (typeof server === 'string' || !server.domains) return [];
  return server.domains;
};

const addDnsServer = () => {
  editingDnsIndex.value = -1;
  editDnsForm.value = { address: '', port: undefined, domainsText: '' };
  dnsDialogOpen.value = true;
};

const editDnsServer = (index: number) => {
  editingDnsIndex.value = index;
  const server = dnsConfig.value.servers[index];
  if (typeof server === 'string') {
    editDnsForm.value = { address: server, port: undefined, domainsText: '' };
  } else {
    editDnsForm.value = {
      address: server.address,
      port: server.port,
      domainsText: server.domains ? server.domains.join('\n') : ''
    };
  }
  dnsDialogOpen.value = true;
};

const confirmDnsServer = () => {
    const { address, port, domainsText } = editDnsForm.value;
    if (!address) return;

    const domains = domainsText.trim() ? domainsText.split('\n').map(d => d.trim()).filter(d => d) : undefined;
    
    // 但如果有端口或域名，必须存为对象
    let newServer: string | DnsServer;
    
    if (!port && !domains) {
        newServer = address;
    } else {
        newServer = { address, port, domains };
    }

    if (editingDnsIndex.value === -1) {
        dnsConfig.value.servers.push(newServer);
    } else {
        dnsConfig.value.servers[editingDnsIndex.value] = newServer;
    }
    dnsDialogOpen.value = false;
};

const removeDnsServer = (index: number) => {
    dnsConfig.value.servers.splice(index, 1);
};

// Routing Helpers
const addRoutingRule = () => {
    editingRuleIndex.value = -1;
    editRuleForm.value = { type: 'field', outboundTag: 'proxy', domainsText: '', ipsText: '', port: '' };
    routingDialogOpen.value = true;
};

const editRoutingRule = (index: number) => {
    editingRuleIndex.value = index;
    const rule = routingConfig.value.rules[index];
    editRuleForm.value = {
        type: rule.type,
        outboundTag: rule.outboundTag,
        domainsText: rule.domain ? rule.domain.join('\n') : '',
        ipsText: rule.ip ? rule.ip.join('\n') : '',
        port: rule.port || ''
    };
    routingDialogOpen.value = true;
};

const confirmRoutingRule = () => {
    const { type, outboundTag, domainsText, ipsText, port } = editRuleForm.value;
    
    const domain = domainsText.trim() ? domainsText.split('\n').map(d => d.trim()).filter(d => d) : undefined;
    const ip = ipsText.trim() ? ipsText.split('\n').map(d => d.trim()).filter(d => d) : undefined;
    
    const newRule: RoutingRule = {
        type,
        outboundTag,
        domain,
        ip,
        port: port || undefined
    };

    if (editingRuleIndex.value === -1) {
        routingConfig.value.rules.push(newRule);
    } else {
        routingConfig.value.rules[editingRuleIndex.value] = newRule;
    }
    routingDialogOpen.value = false;
};

const removeRoutingRule = (index: number) => {
    routingConfig.value.rules.splice(index, 1);
};

const moveRule = (index: number, direction: number) => {
    const rules = routingConfig.value.rules;
    const newIndex = index + direction;
    if (newIndex >= 0 && newIndex < rules.length) {
        // Swap
        [rules[index], rules[newIndex]] = [rules[newIndex], rules[index]];
    }
};

// Common
const handleThemeChange = (e: any) => {
    setThemeMode(e.target.value);
};

const handleColorChange = (color: string) => {
    setThemeColor(color);
};

const handleQueryStrategyChange = (e: any) => { if(e.target) dnsConfig.value.queryStrategy = e.target.value; };
const handleDomainStrategyChange = (e: any) => { if(e.target) routingConfig.value.domainStrategy = e.target.value; };
const handleLogTypeChange = (e: any) => { 
    if(e.target) {
        logType.value = e.target.value;
        refreshLog();
    }
};

const saveDns = async () => {
    try { await saveDnsConfig(dnsConfig.value); /* Toast success */ } catch(e) { console.error(e); }
};
const saveRouting = async () => {
    try { await saveRoutingConfig(routingConfig.value); /* Toast success */ } catch(e) { console.error(e); }
};

const refreshLog = async () => {
    try { logContent.value = await getXrayLog(logType.value); } catch(e) { console.error(e); }
};

const startLogAutoRefresh = () => {
    stopLogAutoRefresh();
    refreshLog();
    logInterval = window.setInterval(refreshLog, 3000);
};

const stopLogAutoRefresh = () => {
    if (logInterval) { window.clearInterval(logInterval); logInterval = null; }
};

watch(currentTab, (val) => {
    if (val === 'logs') startLogAutoRefresh();
    else stopLogAutoRefresh();
});

onMounted(() => {
    if (tabsRef.value) tabsRef.value.addEventListener('change', handleTabChange);
    // Load config
    getDnsConfig().then(c => { if(c) dnsConfig.value = c; });
    getRoutingConfig().then(c => { if(c) routingConfig.value = c; });
});

onUnmounted(() => {
  stopLogAutoRefresh();
  if (tabsRef.value) tabsRef.value.removeEventListener('change', handleTabChange);
});
</script>

<style scoped>
.settings-page { max-width: 800px; margin: 0 auto; padding: 24px; }
.panel-content { display: flex; flex-direction: column; gap: 24px; padding-top: 24px; }
.card-content { padding: 24px; }
h3 { margin: 0 0 16px 0; font-size: 18px; font-weight: 500; color: var(--mdui-color-primary); }
.hint { font-size: 13px; color: var(--mdui-color-on-surface-variant); margin-bottom: 16px; }

/* DNS List */
.dns-list { display: flex; flex-direction: column; gap: 12px; margin-bottom: 24px; }
.dns-item { 
    display: flex; align-items: center; gap: 16px; 
    padding: 12px 16px; background: var(--mdui-color-surface-container-low); 
    border-radius: 12px;
}
.item-info { flex: 1; }
.server-addr { font-family: monospace; font-size: 14px; font-weight: bold; }
.server-domains { margin-top: 4px; display: flex; flex-wrap: wrap; gap: 4px; }
.domain-tag { font-size: 10px; padding: 2px 6px; background: var(--mdui-color-secondary-container); border-radius: 4px; color: var(--mdui-color-on-secondary-container); }
.add-btn-container { text-align: center; }

/* Rules List */
.card-header-row { display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px; }
.rules-list { display: flex; flex-direction: column; gap: 8px; }
.rule-item { 
    display: flex; align-items: center; gap: 12px; 
    padding: 12px; background: var(--mdui-color-surface-container-low); 
    border-radius: 8px; border: 1px solid transparent;
}
.rule-item:hover { border-color: var(--mdui-color-outline-variant); }
.rule-priority { 
    width: 24px; height: 24px; display: flex; align-items: center; justify-content: center; 
    background: var(--mdui-color-surface-container-high); border-radius: 50%; font-size: 12px; 
}
.rule-content { flex: 1; }
.rule-main { display: flex; align-items: center; gap: 8px; font-size: 14px; font-weight: 500; }
.arrow { color: var(--mdui-color-on-surface-variant); }
.tag { padding: 2px 8px; border-radius: 4px; font-size: 12px; }
.type-tag { background: var(--mdui-color-tertiary-container); color: var(--mdui-color-on-tertiary-container); }
.outbound-tag { background: var(--mdui-color-primary-container); color: var(--mdui-color-on-primary-container); }
.rule-details { margin-top: 4px; display: flex; gap: 12px; font-size: 12px; color: var(--mdui-color-on-surface-variant); }

.rule-actions { display: flex; opacity: 0.6; }
.rule-item:hover .rule-actions { opacity: 1; }

.actions { display: flex; justify-content: flex-end; margin-top: 16px; }

/* Dialogs */
.config-dialog { --shape-corner: 28px; }
.dialog-content { display: flex; flex-direction: column; gap: 16px; min-width: 400px; }
.full-width { width: 100%; }
.form-group .label { font-size: 14px; margin-bottom: 8px; color: var(--mdui-color-on-surface); }

/* Logs */
.log-card { display: flex; flex-direction: column; height: 500px; }
.log-content { flex: 1; background: #1e1e1e; color: #d4d4d4; padding: 16px; margin: 16px; overflow: auto; border-radius: 8px; font-family: monospace; font-size: 12px; }
.log-controls { display: flex; gap: 8px; }
.card-header { display: flex; justify-content: space-between; align-items: center; padding: 16px 24px 0; }

.theme-options { display: flex; justify-content: flex-start; margin-top: 16px; margin-bottom: 24px; }
.divider { height: 1px; background-color: var(--mdui-color-outline-variant); margin: 24px 0; opacity: 0.5; }

.color-palette { display: flex; gap: 16px; flex-wrap: wrap; margin-top: 16px; }
.color-item { 
  width: 48px; height: 48px; border-radius: 50%; cursor: pointer; 
  display: flex; align-items: center; justify-content: center;
  transition: transform 0.2s;
  border: 2px solid transparent;
}
.color-item:hover { transform: scale(1.1); }
.color-item.selected { border-color: var(--mdui-color-on-surface); }
.check-icon { color: white; filter: drop-shadow(0 1px 2px rgba(0,0,0,0.3)); }
</style>
