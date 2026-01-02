<template>
  <div v-if="show" class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>翻译历史</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg width="16" height="16" viewBox="0 0 16 16">
            <line x1="2" y1="2" x2="14" y2="14" stroke="currentColor" stroke-width="2"/>
            <line x1="14" y1="2" x2="2" y2="14" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>
      
      <div class="modal-body">
        <div v-if="history.length === 0" class="empty-history">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
          </svg>
          <p>暂无翻译历史</p>
        </div>
        
        <div v-else class="history-list">
          <div 
            v-for="(item, index) in history" 
            :key="index" 
            class="history-item"
          >
            <div class="history-text">
              <div class="source-text">{{ item.sourceText }}</div>
              <div class="target-text">{{ item.targetText }}</div>
              <div class="history-meta">
                <span class="languages">{{ item.sourceLanguage }} → {{ item.targetLanguage }}</span>
                <span class="time">{{ formatTime(item.timestamp) }}</span>
              </div>
            </div>
            <div class="history-actions">
              <button 
                class="action-btn"
                @click="$emit('copy-history', item.targetText)"
                title="复制翻译结果"
              >
                <svg width="16" height="16" viewBox="0 0 16 16">
                  <rect x="4" y="4" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.5" rx="1"/>
                  <path d="M6 2h6a2 2 0 0 1 2 2v6" fill="none" stroke="currentColor" stroke-width="1.5"/>
                </svg>
              </button>
              <button 
                class="action-btn"
                @click="$emit('use-history', item)"
                title="使用此翻译"
              >
                <svg width="16" height="16" viewBox="0 0 16 16">
                  <path d="M2 8l4 4 8-8" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <div class="footer-left">
          <button 
            class="btn btn-secondary clear-btn" 
            :class="{ 'confirming': confirmingClear }"
            @click="handleClearClick"
            :disabled="history.length === 0"
          >
            {{ confirmingClear ? '确认清空' : '清空历史' }}
          </button>
          <button 
            v-if="confirmingClear" 
            class="btn btn-tertiary cancel-btn" 
            @click="cancelClearConfirmation"
          >
            取消
          </button>
          <span v-if="confirmingClear" class="clear-confirm-hint">
            请在 {{ confirmCountdown }} 秒内再次点击确认
          </span>
        </div>
        <button class="btn btn-primary" @click="$emit('close')">
          关闭
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onBeforeUnmount, ref, watch } from 'vue'

const props = defineProps({
  show: Boolean,
  history: {
    type: Array,
    default: () => []
  }
})

const emit = defineEmits(['close', 'clear-history', 'copy-history', 'use-history'])

const confirmingClear = ref(false)
const confirmCountdown = ref(0)
let confirmTimer = null

const clearConfirmTimer = () => {
  if (confirmTimer) {
    clearInterval(confirmTimer)
    confirmTimer = null
  }
}

const resetConfirmState = () => {
  confirmingClear.value = false
  confirmCountdown.value = 0
  clearConfirmTimer()
}

const startConfirmCountdown = () => {
  resetConfirmState()
  confirmingClear.value = true
  confirmCountdown.value = 5

  const tick = () => {
    confirmCountdown.value = Math.max(0, confirmCountdown.value - 1)
    if (confirmCountdown.value === 0) {
      resetConfirmState()
    }
  }

  const setIntervalFn = typeof window !== 'undefined' && window.setInterval ? window.setInterval : setInterval
  confirmTimer = setIntervalFn(tick, 1000)
}

const handleClearClick = () => {
  if (!props.history || props.history.length === 0) {
    return
  }

  if (!confirmingClear.value) {
    startConfirmCountdown()
    return
  }

  emit('clear-history')
  resetConfirmState()
}

const cancelClearConfirmation = () => {
  if (confirmingClear.value) {
    resetConfirmState()
  }
}

watch(() => props.show, (visible) => {
  if (!visible) {
    resetConfirmState()
  }
})

watch(() => props.history?.length, (length) => {
  if (!length) {
    resetConfirmState()
  }
})

onBeforeUnmount(() => {
  resetConfirmState()
})

const parseTimestamp = (timestamp) => {
  if (!timestamp) return null

  if (timestamp instanceof Date) {
    return isNaN(timestamp.getTime()) ? null : timestamp
  }

  if (typeof timestamp === 'number') {
    const fromNumber = new Date(timestamp)
    return isNaN(fromNumber.getTime()) ? null : fromNumber
  }

  let normalized = String(timestamp).trim()
  if (!normalized) return null

  if (/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$/.test(normalized)) {
    normalized = normalized.replace(' ', 'T')
  }

  if (!/[zZ]|[+-]\d{2}:\d{2}$/.test(normalized)) {
    normalized = `${normalized}Z`
  }

  const parsed = new Date(normalized)
  return isNaN(parsed.getTime()) ? null : parsed
}

const formatTime = (timestamp) => {
  const date = parseTimestamp(timestamp)
  if (!date) {
    return '未知时间'
  }

  const diff = Date.now() - date.getTime()

  if (diff < 60000) {
    return '刚刚'
  } else if (diff < 3600000) {
    return `${Math.max(1, Math.floor(diff / 60000))}分钟前`
  } else if (diff < 86400000) {
    return `${Math.max(1, Math.floor(diff / 3600000))}小时前`
  } else {
    return date.toLocaleDateString()
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--mac-bg);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--mac-shadow);
  border: 1px solid var(--mac-border);
  color: var(--mac-text);
  transition: all 0.3s ease;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--mac-border);
}

.modal-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.close-btn {
  background: var(--mac-btn-bg);
  border: none;
  padding: 8px;
  border-radius: 6px;
  cursor: pointer;
  color: var(--mac-text);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.7;
}

.close-btn:hover {
  background: var(--mac-card);
  opacity: 1;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.modal-body::-webkit-scrollbar {
  width: 6px;
}

.modal-body::-webkit-scrollbar-thumb {
  background: var(--mac-border);
  border-radius: 3px;
}

.empty-history {
  text-align: center;
  padding: 48px 24px;
  opacity: 0.5;
}

.empty-history svg {
  margin-bottom: 16px;
}

.empty-history p {
  margin: 0;
  font-size: 14px;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item {
  background: var(--mac-card);
  border: 1px solid var(--mac-border);
  border-radius: 8px;
  padding: 16px;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  transition: all 0.2s;
}

.history-item:hover {
  border-color: var(--mac-accent);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.history-text {
  flex: 1;
  margin-right: 12px;
}

.source-text {
  font-size: 13px;
  opacity: 0.7;
  margin-bottom: 4px;
  line-height: 1.4;
  word-break: break-word;
}

.target-text {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 8px;
  line-height: 1.4;
  word-break: break-word;
}

.history-meta {
  display: flex;
  gap: 12px;
  font-size: 11px;
  opacity: 0.5;
}

.languages {
  font-weight: 500;
}

.history-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.action-btn {
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  padding: 6px;
  border-radius: 4px;
  cursor: pointer;
  color: var(--mac-text);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.8;
}

.action-btn:hover {
  background: var(--mac-card);
  border-color: var(--mac-accent);
  color: var(--mac-accent);
  opacity: 1;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid var(--mac-border);
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: var(--mac-btn-bg);
  color: var(--mac-text);
  border: 1px solid var(--mac-toolbar-border);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--mac-card);
  border-color: var(--mac-border);
}

.btn-secondary:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--mac-accent);
  color: white;
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.2);
}

.btn-primary:hover {
  opacity: 0.9;
  transform: translateY(-1px);
}

.clear-btn.confirming {
  background: #ff3b30;
  color: white;
  border-color: #ff3b30;
}

.clear-btn.confirming:hover {
  opacity: 0.9;
}

.btn-tertiary {
  background: transparent;
  color: var(--mac-text);
  opacity: 0.6;
}

.btn-tertiary:hover {
  opacity: 1;
}

.cancel-btn {
  padding: 8px 12px;
}

.clear-confirm-hint {
  font-size: 11px;
  color: #ff3b30;
}

@media (max-width: 768px) {
  .modal-content {
    width: 95%;
    max-height: 90vh;
  }
  
  .history-item {
    flex-direction: column;
    gap: 12px;
  }
  
  .history-text {
    margin-right: 0;
  }
  
  .history-actions {
    align-self: flex-end;
  }
}
</style>
