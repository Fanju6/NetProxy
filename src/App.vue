<template>
  <mdui-layout>
    <mdui-navigation-rail ref="navRail" :value="currentPage">
      <div style="height: 16px;"></div>
      
      <mdui-navigation-rail-item value="dashboard" icon="dashboard">
        仪表盘
      </mdui-navigation-rail-item>
      <mdui-navigation-rail-item value="nodes" icon="cloud">
        节点
      </mdui-navigation-rail-item>
      <mdui-navigation-rail-item value="settings" icon="settings">
        设置
      </mdui-navigation-rail-item>

      <div style="flex: 1"></div>
      
      <!-- 主题切换按钮 -->
      <mdui-button-icon :icon="themeIcon" @click="toggleTheme" class="theme-btn" tooltip="切换主题"></mdui-button-icon>
      <div style="height: 16px;"></div>
    </mdui-navigation-rail>

    <mdui-layout-main>
      <Transition name="fade">
        <div :key="currentPage" class="page-container">
          <Dashboard v-if="currentPage === 'dashboard'" />
          <Nodes v-else-if="currentPage === 'nodes'" />
          <Settings v-else-if="currentPage === 'settings'" />
        </div>
      </Transition>
    </mdui-layout-main>
  </mdui-layout>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import Dashboard from './views/Dashboard.vue';
import Nodes from './views/Nodes.vue';
import Settings from './views/Settings.vue';
import { useTheme } from './composables/theme';

const currentPage = ref('dashboard');
const navRail = ref<HTMLElement | null>(null);

// Theme Logic
const { currentTheme, toggleTheme, initTheme } = useTheme();

const themeIcon = computed(() => {
  if (currentTheme.value === 'light') return 'light_mode';
  if (currentTheme.value === 'dark') return 'dark_mode';
  return 'brightness_auto';
});

const handleNavChange = (event: Event) => {
  const target = event.target as HTMLElement;
  const value = (target as any).value;
  if (value) {
    currentPage.value = value;
  }
};

onMounted(() => {
  if (navRail.value) {
    navRail.value.addEventListener('change', handleNavChange);
  }
  initTheme();
});
</script>

<style scoped>
.page-container {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.theme-btn {
  margin: 0 auto;
  display: flex;
}
</style>