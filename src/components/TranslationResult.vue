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
  background: var(--mac-card);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border-radius: 12px;
  border: 1px solid var(--mac-border);
  padding: 16px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.05);
  display: flex;
  flex-direction: column;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.result-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--mac-text);
  letter-spacing: -0.01em;
}

.result-actions {
  display: flex;
  gap: 6px;
}

.action-btn {
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  color: var(--mac-text);
  cursor: pointer;
  padding: 4px 10px;
  border-radius: 6px;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 11px;
}

.action-btn:hover:not(:disabled) {
  background: var(--mac-card);
  transform: translateY(-1px);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  color: var(--mac-accent);
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.copy-message {
  font-size: 11px;
  margin-bottom: 8px;
  padding: 4px 10px;
  border-radius: 6px;
  width: fit-content;
  font-weight: 500;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

.copy-message--success {
  background: rgba(52, 199, 89, 0.1);
  color: #34c759;
}

.copy-message--error {
  background: rgba(255, 59, 48, 0.1);
  color: #ff3b30;
}

.result-content {
  height: 200px;
  position: relative;
  overflow: hidden;
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 12px;
  color: var(--mac-text);
  font-size: 14px;
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border: 3px solid rgba(0, 122, 255, 0.1);
  border-top: 3px solid var(--mac-accent);
  border-radius: 50%;
  animation: spin 0.8s cubic-bezier(0.4, 0, 0.2, 1) infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.result-text {
  color: var(--mac-text);
  line-height: 1.6;
  font-size: 15px;
  white-space: pre-wrap;
  word-wrap: break-word;
  height: 100%;
  overflow-y: auto;
  padding-right: 6px;
}

.result-text::-webkit-scrollbar {
  width: 4px;
}

.result-text::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 2px;
}

[data-theme='dark'] .result-text::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
}

.placeholder {
  color: var(--mac-text);
  opacity: 0.3;
  font-style: normal;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  font-size: 14px;
}
</style>
