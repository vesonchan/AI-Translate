<template>
  <div class="translation-result">
    <div class="result-header">
      <span class="result-label">翻译结果</span>
      <div class="result-actions">
        <button 
          class="action-btn"
          @click="$emit('copy')"
          :disabled="!translatedText"
          title="复制结果"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
          </svg>
          <span>复制</span>
        </button>
        <button
          v-if="isEnglishText"
          class="action-btn"
          @click="$emit('copy-snake')"
          :disabled="!translatedText"
          title="复制蛇形命名"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 6h4a2 2 0 0 1 2 2v0a2 2 0 0 1-2 2h-2a2 2 0 0 0-2 2v0a2 2 0 0 0 2 2h4" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <span>蛇形</span>
        </button>
        <button
          v-if="isEnglishText"
          class="action-btn"
          @click="$emit('copy-camel')"
          :disabled="!translatedText"
          title="复制驼峰命名"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 16v-4a2 2 0 0 1 2-2h0a2 2 0 0 1 2 2v4" stroke-linecap="round" stroke-linejoin="round" />
            <path d="M10 16v-3.5a2 2 0 0 1 2-2h0a2 2 0 0 1 2 2V16" stroke-linecap="round" stroke-linejoin="round" />
            <path d="M16 16v-2a2 2 0 0 1 2-2h0" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
          <span>驼峰</span>
        </button>
      </div>
    </div>
    <div
      v-if="copyMessage && copyMessage.text"
      class="copy-message"
      :class="copyMessageStatusClass"
    >
      {{ copyMessage.text }}
    </div>
    <div class="result-content">
      <div v-if="isTranslating" class="loading">
        <div class="loading-spinner"></div>
        <span>翻译中...</span>
      </div>
      <div v-else-if="translatedText" class="result-text">
        {{ translatedText }}
      </div>
      <div v-else class="placeholder">
        翻译结果将显示在这里
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  translatedText: {
    type: String,
    default: ''
  },
  isTranslating: {
    type: Boolean,
    default: false
  },
  copyMessage: {
    type: Object,
    default: null
  }
})

defineEmits(['copy', 'copy-snake', 'copy-camel'])

const isEnglishText = computed(() => {
  const text = (props.translatedText || '').trim()
  if (!text) return false
  const hasEnglishLetters = /[A-Za-z]/.test(text)
  const hasCjkCharacters = /[\u4e00-\u9fa5]/.test(text)
  return hasEnglishLetters && !hasCjkCharacters
})

const copyMessageStatusClass = computed(() => {
  if (!props.copyMessage || !props.copyMessage.text) return ''
  return props.copyMessage.type === 'error'
    ? 'copy-message--error'
    : 'copy-message--success'
})
</script>

<style scoped>
.translation-result {
  background: white;
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.result-label {
  font-weight: 600;
  color: #374151;
}

.result-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.action-btn {
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  color: #374151;
  cursor: pointer;
  padding: 3px 6px;
  border-radius: 4px;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 3px;
  font-size: 11px;
  line-height: 1;
}

.action-btn svg {
  flex-shrink: 0;
}

.action-btn span {
  font-weight: 500;
}

.action-btn:hover:not(:disabled) {
  background: #e5edff;
  border-color: #c3d4ff;
  color: #1d4ed8;
}

.action-btn:disabled {
  opacity: 0.55;
  background: #f3f4f6;
  border-color: #e5e7eb;
  color: #9ca3af;
  cursor: not-allowed;
}

.copy-message {
  font-size: 12px;
  margin-bottom: 8px;
  padding: 4px 8px;
  border-radius: 4px;
  width: fit-content;
}

.copy-message--success {
  background: #ecfdf5;
  color: #047857;
}

.copy-message--error {
  background: #fef2f2;
  color: #b91c1c;
}

.result-content {
  height: 180px;
  position: relative;
  overflow: hidden;
}

.loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  color: #6b7280;
  font-size: 14px;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #e5e7eb;
  border-top: 2px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.result-text {
  color: #374151;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
  height: 100%;
  overflow-y: auto;
  padding-right: 4px;
}

.placeholder {
  color: #9ca3af;
  font-style: italic;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 120px;
}
</style>
