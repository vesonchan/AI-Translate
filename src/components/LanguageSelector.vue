<template>
  <div class="language-selector">
    <div class="lang-group">
      <select v-model="sourceLang" @change="$emit('source-change', sourceLang)" class="lang-select">
        <option v-for="lang in languages" :key="lang.code" :value="lang.code">
          {{ lang.name }}
        </option>
      </select>
      <button @click="$emit('swap')" class="swap-btn" title="交换语言">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4"/>
        </svg>
      </button>
    </div>
    <div class="lang-group">
      <select v-model="targetLang" @change="$emit('target-change', targetLang)" class="lang-select">
        <option v-for="lang in languages" :key="lang.code" :value="lang.code">
          {{ lang.name }}
        </option>
      </select>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  sourceLanguage: String,
  targetLanguage: String
})

const emit = defineEmits(['source-change', 'target-change', 'swap'])

const sourceLang = ref(props.sourceLanguage)
const targetLang = ref(props.targetLanguage)

const languages = [
  { code: 'auto', name: '自动检测' },
  { code: 'zh', name: '中文' },
  { code: 'en', name: '英语' },
  { code: 'ja', name: '日语' },
  { code: 'ko', name: '韩语' },
  { code: 'fr', name: '法语' },
  { code: 'de', name: '德语' },
  { code: 'es', name: '西班牙语' },
  { code: 'ru', name: '俄语' },
  { code: 'it', name: '意大利语' },
  { code: 'pt', name: '葡萄牙语' },
  { code: 'ar', name: '阿拉伯语' },
  { code: 'hi', name: '印地语' },
  { code: 'th', name: '泰语' },
  { code: 'vi', name: '越南语' }
]

// 监听props变化
watch(() => props.sourceLanguage, (newVal) => {
  sourceLang.value = newVal
})

watch(() => props.targetLanguage, (newVal) => {
  targetLang.value = newVal
})
</script>

<style scoped>
.language-selector {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.lang-group {
  display: flex;
  align-items: center;
  gap: 5px;
  flex: 1;
}

.lang-select {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  font-size: 14px;
  cursor: pointer;
}

.lang-select:hover {
  border-color: #007bff;
}

.lang-select:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.swap-btn {
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.swap-btn:hover {
  background: #f8f9fa;
  border-color: #007bff;
}

.swap-btn:active {
  transform: scale(0.95);
}
</style>
