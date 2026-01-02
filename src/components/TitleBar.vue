<template>
  <div class="title-bar" data-tauri-drag-region>
    <div class="title">AI-Translate</div>
    <div class="window-controls" data-tauri-drag-region="false">
      <button class="control-btn close" data-tauri-drag-region="false" @click="$emit('close')">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <line x1="2" y1="2" x2="10" y2="10" stroke="currentColor" stroke-width="1.5"/>
          <line x1="10" y1="2" x2="2" y2="10" stroke="currentColor" stroke-width="1.5"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup>
defineEmits(['close'])
</script>

<style scoped>
.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: var(--mac-toolbar-bg);
  color: var(--mac-text);
  font-weight: 500;
  user-select: none;
  -webkit-app-region: drag;
  border-bottom: 1px solid var(--mac-toolbar-border);
}

.window-controls {
  display: flex;
  gap: 8px;
  -webkit-app-region: no-drag;
}

.control-btn {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 4px;
  background: var(--mac-btn-bg);
  color: var(--mac-text);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  opacity: 0.7;
}

.control-btn:hover {
  background: var(--mac-card);
  opacity: 1;
}

.control-btn.close:hover {
  background: #ff5f56;
  color: white;
  opacity: 1;
}
</style>
