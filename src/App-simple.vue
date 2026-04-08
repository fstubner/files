<template>
  <div class="app">
    <div class="header">
      <h1>AI File Explorer</h1>
      <p>Status: {{ status }}</p>
    </div>
    
    <div class="content">
      <div class="sidebar">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          :class="['nav-item', { active: activeTab === tab.id }]"
          @click="activeTab = tab.id"
        >
          {{ tab.label }}
        </button>
      </div>
      
      <div class="main">
        <div v-if="activeTab === 'files'" class="panel">
          <h2>Files Panel</h2>
          <p>File explorer functionality will go here</p>
        </div>
        
        <div v-if="activeTab === 'search'" class="panel">
          <h2>Search Panel</h2>
          <p>Search functionality will go here</p>
        </div>
        
        <div v-if="activeTab === 'dashboard'" class="panel">
          <h2>Dashboard Panel</h2>
          <p>Dashboard functionality will go here</p>
        </div>
        
        <div v-if="activeTab === 'settings'" class="panel">
          <h2>Settings Panel</h2>
          <p>Settings functionality will go here</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const status = ref('Loading...')
const activeTab = ref('files')

const tabs = [
  { id: 'files', label: 'Files' },
  { id: 'search', label: 'Search' },
  { id: 'dashboard', label: 'Dashboard' },
  { id: 'settings', label: 'Settings' }
]

onMounted(() => {
  status.value = 'Ready'
})
</script>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  font-family: system-ui, sans-serif;
}

.header {
  padding: 1rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
}

.header h1 {
  margin: 0 0 0.5rem 0;
  color: #333;
}

.content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 200px;
  background: #f8f9fa;
  border-right: 1px solid #e9ecef;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.nav-item {
  padding: 0.75rem;
  background: none;
  border: none;
  text-align: left;
  cursor: pointer;
  border-radius: 0.375rem;
  transition: all 0.2s;
}

.nav-item:hover {
  background: #e9ecef;
}

.nav-item.active {
  background: #007bff;
  color: white;
}

.main {
  flex: 1;
  padding: 1rem;
  overflow: auto;
}

.panel {
  background: white;
  padding: 2rem;
  border-radius: 0.5rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.panel h2 {
  margin: 0 0 1rem 0;
  color: #333;
}
</style>