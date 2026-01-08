<template>
  <div class="nodes-container">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <div class="search-box">
        <mdui-icon name="search" class="search-icon"></mdui-icon>
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="搜索节点..." 
          class="search-input"
        />
      </div>
      
      <div class="actions">
        <mdui-button-icon icon="add" @click="showAddDialog = true" tooltip="添加节点"></mdui-button-icon>
        <mdui-button-icon icon="refresh" @click="loadNodes" tooltip="刷新列表"></mdui-button-icon>
      </div>
    </div>

    <!-- 节点列表 -->
    <div class="nodes-content">
      <div v-if="loading" class="loading-state">
        <mdui-circular-progress></mdui-circular-progress>
      </div>
      
      <div v-else-if="Object.keys(groupedNodes).length === 0" class="empty-state">
        <mdui-icon name="dns" class="empty-icon"></mdui-icon>
        <p>暂无节点，请添加订阅或手动导入</p>
        <mdui-button @click="showAddDialog = true">添加节点</mdui-button>
      </div>

      <div v-else class="groups-container">
        <!-- 分组展示 -->
        <div v-for="(nodes, groupName) in groupedNodes" :key="groupName" class="node-group">
          <div class="group-header" @click="toggleGroup(groupName)">
            <mdui-icon :name="expandedGroups[groupName] ? 'expand_more' : 'chevron_right'"></mdui-icon>
            <span class="group-title">{{ groupName }}</span>
            <span class="group-count">{{ nodes.length }}</span>
          </div>
          
          <div v-show="expandedGroups[groupName]" class="group-nodes">
            <div 
              v-for="node in nodes" 
              :key="node.fileName"
              class="node-card"
              :class="{ 'active': currentNodeId === node.fileName }"
              @click="selectNode(node)"
            >
              <div class="node-info">
                <div class="node-header">
                  <span class="node-name" :title="node.name">{{ node.name }}</span>
                  <span class="node-protocol">{{ node.protocol.toUpperCase() }}</span>
                </div>
                <div class="node-detail">
                  {{ node.address }}:{{ node.port }}
                </div>
              </div>
              
              <div class="node-actions">
                <!-- 延迟显示 -->
                <div 
                  class="latency-tag" 
                  :class="getLatencyClass(nodePings[node.fileName])"
                  v-if="nodePings[node.fileName] !== undefined"
                >
                  {{ nodePings[node.fileName] === -1 ? '超时' : nodePings[node.fileName] + 'ms' }}
                </div>
                <mdui-button-icon 
                  icon="network_check" 
                  @click.stop="pingOne(node)"
                  variant="standard"
                  class="action-btn"
                ></mdui-button-icon>
                <mdui-button-icon 
                  icon="delete" 
                  @click.stop="deleteNodeConfirm(node)"
                  variant="standard"
                  class="action-btn delete-btn"
                ></mdui-button-icon>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 添加节点/订阅对话框 -->
    <mdui-dialog 
      class="add-dialog"
      :open="showAddDialog"
      @closed="showAddDialog = false"
      close-on-overlay-click
    >
      <div class="dialog-header">
        <h3>添加资源</h3>
      </div>
      <div class="dialog-content">
        <mdui-text-field 
          label="订阅链接 / 节点链接" 
          v-model="importUrl"
          class="full-width"
        ></mdui-text-field>
        <mdui-text-field 
          label="备注名称 (可选)" 
          v-model="importName"
          class="full-width"
          v-if="isSubscription"
        ></mdui-text-field>
      </div>
      <div class="dialog-actions">
        <mdui-button variant="text" @click="showAddDialog = false">取消</mdui-button>
        <mdui-button @click="handleImport" :loading="importing">导入</mdui-button>
      </div>
    </mdui-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { listNodes, deleteNode, pingNode, importLink, importSubscription, selectNode as apiSelectNode, getProxyStatus } from '../api';
import type { NodeInfo } from '../types';

const nodes = ref<NodeInfo[]>([]);
const loading = ref(true);
const searchQuery = ref('');
const currentNodeId = ref<string | null>(null);
const expandedGroups = ref<Record<string, boolean>>({});
const nodePings = ref<Record<string, number>>({});

const showAddDialog = ref(false);
const importUrl = ref('');
const importName = ref('');
const importing = ref(false);

const isSubscription = computed(() => {
  return importUrl.value.startsWith('http') && !importUrl.value.startsWith('vmess://') && !importUrl.value.startsWith('vless://') && !importUrl.value.startsWith('ss://');
});

// 加载节点列表
const loadNodes = async () => {
  loading.value = true;
  try {
    nodes.value = await listNodes();
    // 默认展开所有组
    const groups = getGroupNames(nodes.value);
    groups.forEach(g => expandedGroups.value[g] = true);
    
    // 获取当前选中状态
    const status = await getProxyStatus();
    if (status.currentNode) {
      // 暂时用 name 匹配。
      const matched = nodes.value.find(n => n.name === status.currentNode);
      if (matched) currentNodeId.value = matched.fileName;
    }
  } catch (error) {
    console.error('加载节点失败:', error);
  } finally {
    loading.value = false;
  }
};

// 获取分组名
const getGroupNames = (list: NodeInfo[]) => {
  const groups = new Set<string>();
  list.forEach(node => {
    const parts = node.fileName.split(/[\\/]/);
    const groupName = parts.length > 1 ? parts[0].replace('sub_', '') : '默认';
    groups.add(groupName);
  });
  return Array.from(groups);
};

// 分组与筛选
const groupedNodes = computed(() => {
  const query = searchQuery.value.toLowerCase();
  const filtered = nodes.value.filter(n => n.name.toLowerCase().includes(query) || n.address.toLowerCase().includes(query));
  
  const groups: Record<string, NodeInfo[]> = {};
  
  filtered.forEach(node => {
    const parts = node.fileName.split(/[\\/]/);
    // 假设 import_subscription 生成的目录是 sub_xxx
    let groupName = '默认';
    if (parts.length > 1) {
      groupName = parts[0].replace(/^sub_/, '');
    }
    
    if (!groups[groupName]) {
      groups[groupName] = [];
    }
    groups[groupName].push(node);
  });
  
  return groups;
});

const toggleGroup = (name: string) => {
  expandedGroups.value[name] = !expandedGroups.value[name];
};

const selectNode = async (node: NodeInfo) => {
  try {
    await apiSelectNode(node.fileName);
    currentNodeId.value = node.fileName;
    // 提示成功？
  } catch (error) {
    console.error('选择节点失败:', error);
  }
};

const pingOne = async (node: NodeInfo) => {
  nodePings.value[node.fileName] = -2; // Loading
  try {
    const latency = await pingNode(node.address, node.port);
    nodePings.value[node.fileName] = latency;
  } catch (e) {
    nodePings.value[node.fileName] = -1; // Timeout/Error
  }
};

const getLatencyClass = (ping: number) => {
  if (ping === -2) return 'loading'; // 暂不处理 loading 样式，直接显示
  if (ping < 0) return 'timeout';
  if (ping < 100) return 'good';
  if (ping < 300) return 'medium';
  return 'bad';
};

const handleImport = async () => {
  if (!importUrl.value) return;
  importing.value = true;
  try {
    if (isSubscription.value) {
      await importSubscription(importUrl.value, importName.value || 'Subscription');
    } else {
      await importLink(importUrl.value);
    }
    showAddDialog.value = false;
    importUrl.value = '';
    importName.value = '';
    await loadNodes();
  } catch (error) {
    console.error('导入失败:', error);
    // TODO: Toast error
  } finally {
    importing.value = false;
  }
};

const deleteNodeConfirm = async (node: NodeInfo) => {
  if (confirm(`确定要删除节点 "${node.name}" 吗？`)) {
    try {
      await deleteNode(node.fileName);
      await loadNodes();
    } catch (e) {
      console.error(e);
    }
  }
};

onMounted(() => {
  loadNodes();
});
</script>

<style scoped>
.nodes-container {
  padding: 24px;
  max-width: 1000px;
  margin: 0 auto;
}

.toolbar {
  display: flex;
  gap: 16px;
  margin-bottom: 24px;
}

.search-box {
  flex: 1;
  background-color: var(--mdui-color-surface-container-high);
  border-radius: 24px;
  display: flex;
  align-items: center;
  padding: 0 16px;
  height: 48px;
  transition: background-color 0.2s;
}

.search-box:focus-within {
  background-color: var(--mdui-color-surface-container-highest);
}

.search-icon {
  color: var(--mdui-color-on-surface-variant);
  margin-right: 8px;
}

.search-input {
  border: none;
  background: none;
  outline: none;
  width: 100%;
  color: var(--mdui-color-on-surface);
  font-size: 16px;
}

.actions {
  display: flex;
  gap: 8px;
}

.node-group {
  margin-bottom: 24px;
}

.group-header {
  display: flex;
  align-items: center;
  padding: 8px 0;
  cursor: pointer;
  user-select: none;
  color: var(--mdui-color-primary);
}

.group-title {
  font-weight: 500;
  font-size: 16px;
  margin: 0 8px;
}

.group-count {
  font-size: 12px;
  color: var(--mdui-color-on-surface-variant);
  background-color: var(--mdui-color-secondary-container);
  padding: 2px 8px;
  border-radius: 12px;
}

.group-nodes {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 12px;
  padding-top: 8px;
}

.node-card {
  background-color: var(--mdui-color-surface-container-low);
  border-radius: 12px;
  padding: 16px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.node-card:hover {
  background-color: var(--mdui-color-surface-container);
}

.node-card.active {
  background-color: var(--mdui-color-primary-container);
  border-color: var(--mdui-color-primary);
}

.node-card.active .node-name {
  color: var(--mdui-color-on-primary-container);
}

.node-info {
  flex: 1;
  min-width: 0;
  margin-right: 12px;
}

.node-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.node-name {
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--mdui-color-on-surface);
}

.node-protocol {
  font-size: 10px;
  padding: 2px 4px;
  background-color: var(--mdui-color-surface-variant);
  border-radius: 4px;
  color: var(--mdui-color-on-surface-variant);
}

.node-detail {
  font-size: 12px;
  color: var(--mdui-color-on-surface-variant);
  font-family: monospace;
}

.node-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.latency-tag {
  font-size: 12px;
  font-family: monospace;
  margin-right: 4px;
}
.latency-tag.good { color: #4caf50; }
.latency-tag.medium { color: #ff9800; }
.latency-tag.bad, .latency-tag.timeout { color: #f44336; }

.action-btn {
  opacity: 0.6;
}
.action-btn:hover {
  opacity: 1;
}

.delete-btn:hover {
  color: #f44336;
}

.full-width {
  width: 100%;
  margin-bottom: 16px;
}
</style>
