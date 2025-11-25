<template>
  <div v-if="show" class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>设置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg width="16" height="16" viewBox="0 0 16 16">
            <line x1="2" y1="2" x2="14" y2="14" stroke="currentColor" stroke-width="2"/>
            <line x1="14" y1="2" x2="2" y2="14" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>
      
      <div class="modal-body">
        <div class="settings-section">
          <h4>翻译设置</h4>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>翻译服务</span>
              <select v-model="localConfig.translation.service" class="setting-select">
                <option value="openai">OpenAI</option>
                <option value="google">Google 翻译</option>
                <option value="baidu">百度翻译</option>
                <option value="youdao">有道翻译</option>
              </select>
            </label>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>API Base URL</span>
              <input 
                type="text" 
                v-model="localConfig.translation.base_url"
                class="setting-input"
                placeholder="https://api.openai.com/v1"
              >
            </label>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>API Key</span>
              <input 
                type="password" 
                v-model="localConfig.translation.api_key"
                class="setting-input"
                placeholder="输入API密钥"
              >
            </label>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>模型ID</span>
              <input 
                type="text" 
                v-model="localConfig.translation.model_id"
                class="setting-input"
                placeholder="gpt-5-nano"
              >
            </label>
          </div>
        </div>
        
        <div class="settings-section">
          <h4>OCR设置</h4>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>OCR Base URL</span>
              <input 
                type="text" 
                v-model="localConfig.ocr.base_url"
                class="setting-input"
                placeholder="https://api.openai.com/v1"
              >
            </label>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>OCR API Key</span>
              <input 
                type="password" 
                v-model="localConfig.ocr.api_key"
                class="setting-input"
                placeholder="输入OCR API密钥"
              >
            </label>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>OCR模型ID</span>
              <input 
                type="text" 
                v-model="localConfig.ocr.model_id"
                class="setting-input"
                placeholder="gpt-4-vision-preview"
              >
            </label>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>复用翻译设置</span>
              <input 
                type="checkbox" 
                v-model="localConfig.ocr.reuse_translation"
                class="setting-checkbox"
              >
            </label>
            <p class="setting-hint">勾选后，OCR将使用翻译服务的API配置</p>
          </div>
        </div>
        
        <div class="settings-section">
          <h4>快捷键设置</h4>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>弹出窗口</span>
              <input 
                type="text" 
                v-model="localConfig.hotkeys.popup_window"
                class="setting-input"
                placeholder="Ctrl+Shift+T"
                readonly
              >
            </label>
            <p class="setting-hint">按下快捷键弹出翻译窗口</p>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>滑动翻译</span>
              <input 
                type="text" 
                v-model="localConfig.hotkeys.slide_translation"
                class="setting-input"
                placeholder="Ctrl+Shift+S"
                readonly
              >
            </label>
            <p class="setting-hint">按下快捷键启动滑动翻译</p>
          </div>
          
          <div class="setting-item">
            <label class="setting-label">
              <span>截图翻译</span>
              <input 
                type="text" 
                v-model="localConfig.hotkeys.screenshot_translation"
                class="setting-input"
                placeholder="Ctrl+Shift+A"
                readonly
              >
            </label>
            <p class="setting-hint">按下快捷键启动截图翻译</p>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn btn-secondary" @click="resetToDefaults">
          恢复默认
        </button>
        <div class="footer-right">
          <button class="btn btn-secondary" @click="$emit('close')">
            取消
          </button>
          <button class="btn btn-primary" @click="saveSettings">
            保存
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  show: Boolean,
  config: Object
})

const emit = defineEmits(['close', 'save'])

// 默认配置 - 与App.vue中的appConfig结构匹配
const defaultConfig = {
  translation: {
    service: "openai",
    base_url: "https://api.openai.com/v1",
    api_key: "",
    model_id: "gpt-5-nano"
  },
  ocr: {
    base_url: "https://api.openai.com/v1",
    api_key: "",
    model_id: "gpt-4-vision-preview",
    reuse_translation: false
  },
  hotkeys: {
    popup_window: "Ctrl+Shift+T",
    slide_translation: "Ctrl+Shift+S",
    screenshot_translation: "Ctrl+Shift+A"
  }
}

const localConfig = ref({})

watch(() => props.show, (newShow) => {
  if (newShow && props.config) {
    // 深拷贝配置以避免直接修改props
    localConfig.value = JSON.parse(JSON.stringify(props.config))
  }
})

const resetToDefaults = () => {
  localConfig.value = JSON.parse(JSON.stringify(defaultConfig))
}

const saveSettings = () => {
  emit('save', localConfig.value)
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 12px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px 16px;
  border-bottom: 1px solid #e5e7eb;
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
}

.close-btn {
  background: none;
  border: none;
  padding: 8px;
  border-radius: 6px;
  cursor: pointer;
  color: #6b7280;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #f3f4f6;
  color: #374151;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 24px;
}

.settings-section {
  margin-bottom: 24px;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.settings-section h4 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
  padding-bottom: 8px;
  border-bottom: 1px solid #e5e7eb;
}

.setting-item {
  margin-bottom: 16px;
}

.setting-item:last-child {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
  color: #374151;
  margin-bottom: 4px;
}

.setting-select,
.setting-input {
  padding: 6px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 14px;
  background: white;
  transition: all 0.2s;
  min-width: 200px;
}

.setting-select:focus,
.setting-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.setting-input[readonly] {
  background: #f9fafb;
  color: #6b7280;
  cursor: not-allowed;
}

.setting-checkbox {
  width: 16px;
  height: 16px;
  accent-color: #3b82f6;
}

.setting-hint {
  margin: 4px 0 0 0;
  font-size: 12px;
  color: #6b7280;
  line-height: 1.4;
}

.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px 20px;
  border-top: 1px solid #e5e7eb;
}

.footer-right {
  display: flex;
  gap: 12px;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: #f3f4f6;
  color: #374151;
}

.btn-secondary:hover {
  background: #e5e7eb;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover {
  background: #2563eb;
}

@media (max-width: 768px) {
  .modal-content {
    width: 95%;
    max-height: 90vh;
  }
  
  .modal-header,
  .modal-body,
  .modal-footer {
    padding-left: 16px;
    padding-right: 16px;
  }
  
  .setting-label {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .setting-select,
  .setting-input {
    width: 100%;
  }
  
  .modal-footer {
    flex-direction: column;
    gap: 12px;
  }
  
  .footer-right {
    width: 100%;
    justify-content: flex-end;
  }
}
</style>
