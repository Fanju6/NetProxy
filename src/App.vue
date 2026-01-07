<template>
  <mdui-layout>
    <mdui-navigation-rail ref="navRail" :value="currentPage">
      <img src="./assets/logo.png" alt="NetProxy" class="nav-logo" />
      <mdui-navigation-rail-item value="dashboard" icon="dashboard">
        仪表盘
      </mdui-navigation-rail-item>
      <mdui-navigation-rail-item value="nodes" icon="cloud">
        节点
      </mdui-navigation-rail-item>
      <mdui-navigation-rail-item value="settings" icon="settings">
        设置
      </mdui-navigation-rail-item>
    </mdui-navigation-rail>

    <mdui-layout-main>
      <Dashboard v-if="currentPage === 'dashboard'" />
      <Nodes v-else-if="currentPage === 'nodes'" />
      <Settings v-else-if="currentPage === 'settings'" />
    </mdui-layout-main>
  </mdui-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import Dashboard from './views/Dashboard.vue';
import Nodes from './views/Nodes.vue';
import Settings from './views/Settings.vue';

const currentPage = ref('dashboard');
const navRail = ref<HTMLElement | null>(null);

const handleNavChange = (event: Event) => {
  const target = event.target as HTMLElement;
  const value = (target as any).value;
  if (value) {
    currentPage.value = value;
  }
};

onMounted(() => {
  // 使用原生 DOM 事件监听
  if (navRail.value) {
    navRail.value.addEventListener('change', handleNavChange);
  }
});

onUnmounted(() => {
  if (navRail.value) {
    navRail.value.removeEventListener('change', handleNavChange);
  }
});
</script>

<style>
@import 'mdui/mdui.css';

:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
  color-scheme: light dark;
}

body {
  margin: 0;
  min-width: 320px;
  min-height: 100vh;
}

mdui-layout {
  min-height: 100vh;
}

mdui-navigation-rail {
  --shape-corner: 0;
}

.nav-logo {
  width: 48px;
  height: 48px;
  margin: 16px auto;
  object-fit: contain;
}

mdui-layout-main {
  background: var(--mdui-color-surface);
  overflow-y: auto;
}
</style>