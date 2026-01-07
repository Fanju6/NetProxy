<template>
  <div class="dashboard">
    <mdui-card class="status-card">
      <div class="card-content">
        <h2>代理状态</h2>
        <div class="status-row">
          <span>运行状态</span>
          <mdui-switch 
            :checked="status.running" 
            @change="toggleProxy"
          ></mdui-switch>
        </div>
        <div class="status-row">
          <span>系统代理</span>
          <mdui-switch 
            :checked="status.systemProxy" 
            @change="toggleSystemProxy"
            :disabled="!status.running"
          ></mdui-switch>
        </div>
      </div>
    </mdui-card>

    <mdui-card class="node-card">
      <div class="card-content">
        <h2>当前节点</h2>
        <p class="current-node">{{ status.currentNode ? status.currentNode : '未选择节点' }}</p>
        <p class="port-info" v-if="status.running">本地端口: {{ status.port }}</p>
      </div>
    </mdui-card>

    <mdui-card class="info-card">
      <div class="card-content">
        <h2>连接信息</h2>
        <div class="info-row">
          <mdui-icon name="circle" :class="status.running ? 'online' : 'offline'"></mdui-icon>
          <span>{{ status.running ? '已连接' : '未连接' }}</span>
        </div>
      </div>
    </mdui-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getProxyStatus, startProxy, stopProxy, setSystemProxy } from '../api';
import type { ProxyStatus } from '../types';

const status = ref<ProxyStatus>({
  running: false,
  systemProxy: false,
  currentNode: null,
  port: 10808
});

// 切换代理运行状态
const toggleProxy = async () => {
  try {
    if (status.value.running) {
      // 停止代理
      await stopProxy();
      // 如果开启了系统代理，也一并关闭，防止断网
      if (status.value.systemProxy) {
        await setSystemProxy(false, status.value.port);
      }
    } else if (status.value.currentNode) {
      // 启动代理
      await startProxy(status.value.currentNode);
    } else {
      // 未选择节点，这里理论上可以提示用户，但 Switch 会自动回弹通过 refreshStatus
      console.warn('未选择节点，无法启动');
    }
    // 刷新状态
    await refreshStatus();
  } catch (error) {
    console.error('代理切换失败:', error);
  }
};

// 切换系统代理
const toggleSystemProxy = async () => {
  try {
    await setSystemProxy(!status.value.systemProxy, status.value.port);
    await refreshStatus();
  } catch (error) {
    console.error('系统代理切换失败:', error);
  }
};

// 刷新状态
const refreshStatus = async () => {
  try {
    status.value = await getProxyStatus();
  } catch (error) {
    console.error('获取状态失败:', error);
  }
};

onMounted(() => {
  refreshStatus();
});
</script>

<style scoped>
.dashboard {
  padding: 24px;
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

.status-card, .node-card, .info-card {
  padding: 0;
}

.card-content {
  padding: 20px;
}

.card-content h2 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 500;
}

.status-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.current-node {
  font-size: 16px;
  color: var(--mdui-color-primary);
  margin: 8px 0;
  word-break: break-all; /* 防止长路径撑破布局 */
}

.port-info {
  font-size: 14px;
  color: var(--mdui-color-on-surface-variant);
}

.info-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.online {
  color: #4caf50;
}

.offline {
  color: #9e9e9e;
}
</style>
