<template>
  <div id="app" class="app">
    <div class="app-header">
      <h1>AI File Explorer</h1>
      <div class="header-actions">
        <button @click="toggleTheme" class="theme-toggle">
          {{ isDark ? '☀️' : '🌙' }}
        </button>
      </div>
    </div>
    
    <div class="app-content">
      <aside class="sidebar">
        <nav class="navigation">
          <button 
            v-for="tab in tabs" 
            :key="tab.id"
            :class="['nav-item', { active: activeTab === tab.id }]"
            @click="activeTab = tab.id"
          >
            {{ tab.label }}
          </button>
        </nav>
      </aside>
      
      <main class="main-content">
        <FileExplorer v-if="activeTab === 'files'" />
        <SearchPanel v-if="activeTab === 'search'" />
        <Dashboard v-if="activeTab === 'dashboard'" />
        <Settings v-if="activeTab === 'settings'" />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useDark, useToggle } from '@vueuse/core'
import FileExplorer from './components/FileExplorer/FileExplorer.vue'
import SearchPanel from './components/Search/SearchPanel-simple.vue'
import Dashboard from './components/Dashboard/Dashboard-simple.vue'
import Settings from './components/Settings/Settings-simple.vue'

const isDark = useDark()
const toggleTheme = useToggle(isDark)

const activeTab = ref('files')

const tabs = [
  { id: 'files', label: 'Files' },
  { id: 'search', label: 'Search' },
  { id: 'dashboard', label: 'Dashboard' },
  { id: 'settings', label: 'Settings' }
]
</script>

<style>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-family: system-ui, -apple-system, sans-serif;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
}

.app-header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.theme-toggle {
  background: none;
  border: none;
  font-size: 1.2rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 0.25rem;
  transition: background-color 0.2s;
}

.theme-toggle:hover {
  background: var(--bg-tertiary);
}

.app-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 200px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  padding: 1rem;
}

.navigation {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.nav-item {
  padding: 0.75rem 1rem;
  background: none;
  border: none;
  text-align: left;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 0.375rem;
  transition: all 0.2s;
}

.nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--accent-primary);
  color: white;
}

.main-content {
  flex: 1;
  overflow: hidden;
}
</style>