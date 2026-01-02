<template>
  <div class="bottom-toolbar">
    <div class="toolbar-left">
      <button 
        class="toolbar-btn"
        @click="$emit('togglePin')"
        :class="{ active: isPinned }"
        title="置顶窗口"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="12" y1="17" x2="12" y2="22"></line>
          <path d="M5 17h14v-1.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3v4.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24Z"></path>
        </svg>
        <span>{{ isPinned ? '取消置顶' : '置顶' }}</span>
      </button>
      
      <button 
        class="toolbar-btn"
        @click="$emit('showHistory')"
        title="查看历史记录"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"></circle>
          <polyline points="12 6 12 12 16 14"></polyline>
        </svg>
        <span>历史记录</span>
      </button>

      <div class="service-toggle">
        <button 
          class="toggle-item" 
          :class="{ active: serviceType === 'ai' }"
          @click="$emit('update:serviceType', 'ai')"
        >AI</button>
        <button 
          class="toggle-item" 
          :class="{ active: serviceType === 'google' }"
          @click="$emit('update:serviceType', 'google')"
        >Google</button>
      </div>
    </div>
    
    <div class="toolbar-right">
      <button 
        class="toolbar-btn"
        @click="$emit('toggleTheme')"
        :title="theme === 'dark' ? '切换到浅色模式' : '切换到深色模式'"
      >
        <svg v-if="theme === 'dark'" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5"></circle>
          <line x1="12" y1="1" x2="12" y2="3"></line>
          <line x1="12" y1="21" x2="12" y2="23"></line>
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
          <line x1="1" y1="12" x2="3" y2="12"></line>
          <line x1="21" y1="12" x2="23" y2="12"></line>
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
        </svg>
        <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
      </button>

      <button 
        class="toolbar-btn"
        @click="$emit('showSettings')"
        title="设置"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
        <span>设置</span>
      </button>

      <!-- Voice Settings Button -->
      <button 
        class="toolbar-btn"
        @click="$emit('showVoiceSettings')"
        title="语音设置"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
          <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
          <line x1="12" y1="19" x2="12" y2="23"/>
          <line x1="8" y1="23" x2="16" y2="23"/>
        </svg>
      </button>
      
      <button 
        class="toolbar-btn ocr-btn"
        @click="$emit('start-ocr')"
        title="OCR截图翻译"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
        <span>OCR截图</span>
      </button>
    </div>
  </div>
</template>

<script setup>
defineProps({
  isPinned: {
    type: Boolean,
    default: false
  },
  serviceType: {
    type: String,
    default: 'ai'
  },
  theme: {
    type: String,
    default: 'light'
  }
})

defineEmits(['togglePin', 'showHistory', 'showSettings', 'showVoiceSettings', 'start-ocr', 'update:serviceType', 'toggleTheme'])
</script>

<style scoped>
.bottom-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--mac-toolbar-bg);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-top: 1px solid var(--mac-toolbar-border);
  gap: 16px;
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 100;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.service-toggle {
  display: flex;
  background: rgba(0, 0, 0, 0.05);
  padding: 2px;
  border-radius: 8px;
  margin-left: 8px;
}

[data-theme='dark'] .service-toggle {
  background: rgba(255, 255, 255, 0.1);
}

.toggle-item {
  padding: 4px 10px;
  font-size: 12px;
  border: none;
  background: transparent;
  border-radius: 6px;
  cursor: pointer;
  color: var(--mac-text);
  opacity: 0.6;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  font-weight: 500;
}

.toggle-item.active {
  background: var(--mac-card);
  color: var(--mac-text);
  opacity: 1;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.toolbar-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 8px;
  color: var(--mac-text);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.toolbar-btn:hover {
  background: var(--mac-card);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.toolbar-btn.active {
  background: var(--mac-accent);
  border-color: var(--mac-accent);
  color: white;
}

.ocr-btn {
  background: #34c759;
  border-color: #34c759;
  color: white;
  font-weight: 500;
}

.ocr-btn:hover {
  background: #28a745;
  border-color: #28a745;
}

.toolbar-btn svg {
  flex-shrink: 0;
}

@media (max-width: 640px) {
  .bottom-toolbar {
    padding: 8px 12px;
  }
  
  .toolbar-btn span {
    display: none;
  }
  
  .toolbar-btn {
    padding: 8px;
  }

  .service-toggle {
    margin-left: 4px;
  }

  .toggle-item {
    padding: 4px 8px;
  }
}
</style>
