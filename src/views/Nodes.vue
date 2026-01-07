<template>
  <div class="nodes-page">
    <div class="toolbar">
      <mdui-button variant="filled" @click="showImportDialog = true">
        <mdui-icon slot="icon" name="add"></mdui-icon>
        导入节点
      </mdui-button>
      <mdui-button variant="outlined" @click="showSubDialog = true">
        <mdui-icon slot="icon" name="subscriptions"></mdui-icon>
        导入订阅
      </mdui-button>
    </div>

    <div class="nodes-list">
      <mdui-card 
        v-for="node in nodes" 
        :key="node.fileName"
        class="node-card"
        :class="{ selected: node.fileName === selectedNode }"
        @click="handleSelectNode(node.fileName)"
      >
        <div class="node-content">
          <div class="node-info">
            <span class="node-name">{{ node.name }}</span>
            <span class="node-path" v-if="node.fileName.includes('/')">{{ getSubName(node.fileName) }}</span>
          </div>
          <div class="node-actions">
            <mdui-button-icon @click.stop="deleteNodeHandler(node.fileName)">
              <mdui-icon name="delete"></mdui-icon>
            </mdui-button-icon>
          </div>
        </div>
      </mdui-card>

      <div v-if="nodes.length === 0" class="empty-state">
        <mdui-icon name="cloud_off" class="empty-icon"></mdui-icon>
        <p>暂无节点，请导入链接或订阅</p>
      </div>
    </div>

    <!-- 导入链接对话框 -->
    <mdui-dialog 
      :open="showImportDialog" 
      @closed="showImportDialog = false"
      headline="导入节点"
    >
      <mdui-text-field
        v-model="importLink"
        label="节点链接"
        placeholder="vless://, vmess://, trojan://, ss://..."
        rows="3"
      ></mdui-text-field>
      <mdui-button slot="action" variant="text" @click="showImportDialog = false">
        取消
      </mdui-button>
      <mdui-button slot="action" variant="filled" @click="handleImportLink">
        导入
      </mdui-button>
    </mdui-dialog>

    <!-- 导入订阅对话框 -->
    <mdui-dialog 
      :open="showSubDialog" 
      @closed="showSubDialog = false"
      headline="导入订阅"
    >
      <mdui-text-field
        v-model="subName"
        label="订阅名称"
        placeholder="输入订阅名称（如：机场名）"
        style="margin-bottom: 16px;"
      ></mdui-text-field>
      <mdui-text-field
        v-model="subUrl"
        label="订阅地址"
        placeholder="https://..."
      ></mdui-text-field>
      <mdui-button slot="action" variant="text" @click="showSubDialog = false">
        取消
      </mdui-button>
      <mdui-button slot="action" variant="filled" @click="handleImportSub">
        导入
      </mdui-button>
    </mdui-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { listNodes, importLink as importLinkApi, importSubscription, deleteNode } from '../api';
import { selectNode, getSelectedNode } from '../api/proxy';
import type { NodeInfo } from '../types';

const nodes = ref<NodeInfo[]>([]);
const selectedNode = ref<string | null>(null);
const showImportDialog = ref(false);
const showSubDialog = ref(false);
const importLink = ref('');
const subUrl = ref('');
const subName = ref('');

const refreshNodes = async () => {
  try {
    nodes.value = await listNodes();
  } catch (error) {
    console.error('获取节点列表失败:', error);
  }
};

const initSelectedNode = async () => {
  try {
    const node = await getSelectedNode();
    if (node) {
      selectedNode.value = node;
    }
  } catch (error) {
    console.error('获取选中节点失败:', error);
  }
};

const handleSelectNode = async (fileName: string) => {
  try {
    await selectNode(fileName);
    selectedNode.value = fileName;
  } catch (error) {
    console.error('选择节点失败:', error);
  }
};

const getSubName = (filePath: string) => {
  const parts = filePath.split('/');
  if (parts.length > 1) {
    return parts[0].replace('sub_', '');
  }
  return '';
};

const handleImportLink = async () => {
  try {
    await importLinkApi(importLink.value);
    importLink.value = '';
    showImportDialog.value = false;
    await refreshNodes();
  } catch (error) {
    console.error('导入链接失败:', error);
  }
};

const handleImportSub = async () => {
  if (!subName.value.trim()) {
    console.error('请输入订阅名称');
    return;
  }
  try {
    await importSubscription(subUrl.value, subName.value.trim());
    subUrl.value = '';
    subName.value = '';
    showSubDialog.value = false;
    await refreshNodes();
  } catch (error) {
    console.error('导入订阅失败:', error);
  }
};

const deleteNodeHandler = async (filePath: string) => {
  try {
    await deleteNode(filePath);
    await refreshNodes();
    // 如果删除的是当前选中节点，清除选择状态
    if (selectedNode.value === filePath) {
      selectedNode.value = null;
      // TODO: 也可以调用后端清除选中的逻辑，但后端逻辑当前没有 clear_selection。
      // 不过如果不主动清除，用户点击别的节点会覆盖。
    }
  } catch (error) {
    console.error('删除节点失败:', error);
  }
};

onMounted(() => {
  refreshNodes();
  initSelectedNode();
});
</script>

<style scoped>
.nodes-page {
  padding: 24px;
}

.toolbar {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.nodes-list {
  display: grid;
  gap: 12px;
}

.node-card {
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid transparent; /* 预留边框位置避免跳动 */
}

.node-card:hover {
  transform: translateY(-2px);
}

.node-card.selected {
  border: 2px solid rgb(var(--mdui-color-primary));
  /* MDUI v2 使用 RGB 变量，需要注意格式。或者直接使用 --mdui-color-primary */
  border-color: rgb(var(--mdui-color-primary));
}

/* 如果 MDUI 变量不是 RGB 格式，尝试直接引用 */
@supports (color: var(--mdui-color-primary)) {
    .node-card.selected {
        border-color: var(--mdui-color-primary);
    }
}


.node-content {
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 16px;
}

.node-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.node-name {
  font-weight: 500;
  font-size: 16px;
}

.node-path {
  font-size: 12px;
  color: var(--mdui-color-primary);
}

.empty-state {
  text-align: center;
  padding: 48px;
  color: var(--mdui-color-on-surface-variant);
}

.empty-icon {
  font-size: 64px;
  opacity: 0.5;
}
</style>
