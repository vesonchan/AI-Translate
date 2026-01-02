<template>
  <div v-if="show" class="modal-overlay" @click="$emit('close')">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>语音设置</h3>
        <button class="close-btn" @click="$emit('close')">
          <svg width="16" height="16" viewBox="0 0 16 16">
            <line x1="2" y1="2" x2="14" y2="14" stroke="currentColor" stroke-width="2"/>
            <line x1="14" y1="2" x2="2" y2="14" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <!-- Model Selection -->
        <div class="settings-section">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>Whisper 模型</h4>
                <p class="card-subtitle">选择语音识别模型</p>
              </div>
            </div>
            <div class="card-body">
              <div class="model-list">
                <div 
                  v-for="model in models" 
                  :key="model.model"
                  class="model-item"
                  :class="{ 
                    'selected': selectedModel === model.model,
                    'downloaded': model.is_downloaded 
                  }"
                >
                  <div class="model-info">
                    <span class="model-name">{{ model.display_name }}</span>
                    <span v-if="model.model === 'turbo'" class="model-badge">推荐</span>
                  </div>
                  <div class="model-actions">
                    <button 
                      v-if="!model.is_downloaded"
                      class="btn btn-sm btn-primary"
                      :disabled="downloadingModel === model.model"
                      @click="downloadModel(model.model)"
                    >
                      <span v-if="downloadingModel === model.model">
                        {{ downloadProgress }}%
                      </span>
                      <span v-else>下载</span>
                    </button>
                    <button 
                      v-else
                      class="btn btn-sm"
                      :class="selectedModel === model.model ? 'btn-success' : 'btn-secondary'"
                      @click="selectModel(model.model)"
                    >
                      {{ selectedModel === model.model ? '已选择' : '选择' }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Language Settings -->
        <div class="settings-section">
          <div class="card">
            <div class="card-header">
              <h4>语言设置</h4>
            </div>
            <div class="card-body">
              <div class="setting-item">
                <label class="setting-label">
                  <span>识别语言</span>
                  <select v-model="language" class="setting-select">
                    <option value="auto">自动检测</option>
                    <option value="zh">中文</option>
                    <option value="en">英语</option>
                    <option value="ja">日语</option>
                    <option value="ko">韩语</option>
                    <option value="es">西班牙语</option>
                    <option value="fr">法语</option>
                    <option value="de">德语</option>
                  </select>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Recording Mode -->
        <div class="settings-section">
          <div class="card">
            <div class="card-header">
              <h4>录音模式</h4>
            </div>
            <div class="card-body">
              <div class="radio-group">
                <label class="radio-item">
                  <input type="radio" v-model="recordingMode" value="toggle" />
                  <span class="radio-label">
                    <strong>点击切换</strong>
                    <small>点击开始录音，再次点击停止</small>
                  </span>
                </label>
                <label class="radio-item">
                  <input type="radio" v-model="recordingMode" value="push" />
                  <span class="radio-label">
                    <strong>按住说话</strong>
                    <small>按住按钮录音，松开停止</small>
                  </span>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Audio Device -->
        <div class="settings-section">
          <div class="card">
            <div class="card-header">
              <h4>音频输入</h4>
            </div>
            <div class="card-body">
              <div class="setting-item">
                <label class="setting-label">
                  <span>输入设备</span>
                  <select v-model="audioDevice" class="setting-select">
                    <option value="default">默认设备</option>
                    <option v-for="device in audioDevices" :key="device" :value="device">
                      {{ device }}
                    </option>
                  </select>
                </label>
              </div>
            </div>
          </div>
        </div>

        <!-- Shortcut Key -->
        <div class="settings-section">
          <div class="card">
            <div class="card-header">
              <h4>快捷键</h4>
            </div>
            <div class="card-body">
              <div class="setting-item">
                <label class="setting-label">
                  <span>录音快捷键</span>
                  <div class="shortcut-input-wrapper">
                    <input 
                      type="text" 
                      v-model="shortcutKey" 
                      @keydown.prevent="captureShortcut"
                      placeholder="点击并按下快捷键..."
                      class="shortcut-input"
                      readonly
                    />
                    <button class="clear-shortcut-btn" @click="shortcutKey = ''" title="清除">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                      </svg>
                    </button>
                  </div>
                  <small class="shortcut-hint">例如：Ctrl+Shift+R 或 Alt+V</small>
                </label>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <div class="footer-status">
          <span v-if="modelLoaded" class="status-badge success">模型已加载</span>
          <span v-else class="status-badge warning">模型未加载</span>
        </div>
        <div class="footer-right">
          <button class="btn btn-secondary" @click="$emit('close')">取消</button>
          <button class="btn btn-primary" @click="saveSettings">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const props = defineProps({
  show: Boolean
})

const emit = defineEmits(['close', 'save'])

// State
const models = ref([])
const selectedModel = ref('turbo')
const downloadingModel = ref(null)
const downloadProgress = ref(0)
const language = ref('auto')
const recordingMode = ref('toggle')
const audioDevice = ref('default')
const audioDevices = ref([])
const modelLoaded = ref(false)
const shortcutKey = ref('')

// Load models and settings
onMounted(async () => {
  try {
    models.value = await invoke('get_speech_models')
    audioDevices.value = await invoke('get_audio_devices')
    modelLoaded.value = await invoke('is_speech_model_loaded')
  } catch (e) {
    console.error('Failed to load speech settings:', e)
  }
})

// Listen for download progress
listen('speech-model-download-progress', (event) => {
  downloadProgress.value = event.payload.progress
})

// Watch for modal open to refresh state
watch(() => props.show, async (newVal) => {
  if (newVal) {
    try {
      models.value = await invoke('get_speech_models')
      modelLoaded.value = await invoke('is_speech_model_loaded')
    } catch (e) {
      console.error('Failed to refresh speech settings:', e)
    }
  }
})

// Actions
const downloadModel = async (model) => {
  downloadingModel.value = model
  downloadProgress.value = 0
  try {
    await invoke('download_speech_model', { model })
    models.value = await invoke('get_speech_models')
  } catch (e) {
    console.error('Failed to download model:', e)
  } finally {
    downloadingModel.value = null
  }
}

const selectModel = async (model) => {
  selectedModel.value = model
  try {
    await invoke('load_speech_model', { model })
    modelLoaded.value = true
  } catch (e) {
    console.error('Failed to load model:', e)
    modelLoaded.value = false
  }
}

const saveSettings = async () => {
  try {
    await invoke('set_speech_language', { language: language.value })
    emit('save', {
      model: selectedModel.value,
      language: language.value,
      recordingMode: recordingMode.value,
      audioDevice: audioDevice.value,
      shortcutKey: shortcutKey.value
    })
    emit('close')
  } catch (e) {
    console.error('Failed to save settings:', e)
  }
}

// Capture keyboard shortcut
const captureShortcut = (event) => {
  const keys = []
  if (event.ctrlKey) keys.push('Ctrl')
  if (event.altKey) keys.push('Alt')
  if (event.shiftKey) keys.push('Shift')
  if (event.metaKey) keys.push('Meta')
  
  const key = event.key.toUpperCase()
  if (!['CONTROL', 'ALT', 'SHIFT', 'META'].includes(key)) {
    keys.push(key)
  }
  
  if (keys.length > 0) {
    shortcutKey.value = keys.join('+')
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
  max-width: 550px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: var(--mac-shadow);
  border: 1px solid var(--mac-border);
  color: var(--mac-text);
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
  opacity: 0.7;
  transition: all 0.2s;
}

.close-btn:hover {
  background: var(--mac-card);
  opacity: 1;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.modal-body::-webkit-scrollbar {
  width: 6px;
}

.modal-body::-webkit-scrollbar-thumb {
  background: var(--mac-border);
  border-radius: 3px;
}

.settings-section {
  margin-bottom: 16px;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.card {
  background: var(--mac-card);
  border: 1px solid var(--mac-border);
  border-radius: 10px;
  overflow: hidden;
}

.card-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--mac-border);
  background: rgba(255, 255, 255, 0.02);
}

.card-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
}

.card-subtitle {
  margin: 2px 0 0;
  font-size: 11px;
  opacity: 0.5;
}

.card-body {
  padding: 12px 16px;
}

/* Model List */
.model-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.model-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 6px;
  transition: all 0.2s;
}

.model-item.selected {
  border-color: var(--mac-accent);
  background: rgba(10, 132, 255, 0.1);
}

.model-item.downloaded {
  opacity: 1;
}

.model-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.model-name {
  font-size: 13px;
  font-weight: 500;
}

.model-badge {
  font-size: 10px;
  padding: 2px 6px;
  background: var(--mac-accent);
  color: white;
  border-radius: 4px;
}

/* Buttons */
.btn {
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-sm {
  padding: 4px 10px;
  font-size: 11px;
}

.btn-primary {
  background: var(--mac-accent);
  color: white;
}

.btn-secondary {
  background: var(--mac-btn-bg);
  color: var(--mac-text);
  border: 1px solid var(--mac-toolbar-border);
}

.btn-success {
  background: #30d158;
  color: white;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Settings */
.setting-item {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 13px;
}

.setting-label span {
  opacity: 0.8;
}

.setting-select {
  background: var(--mac-card);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 6px;
  padding: 8px 12px;
  color: var(--mac-text);
  font-size: 13px;
  cursor: pointer;
}

.setting-select option {
  background: var(--mac-card);
  color: var(--mac-text);
  padding: 8px;
}

[data-theme='dark'] .setting-select {
  background: #2c2c2e;
}

[data-theme='dark'] .setting-select option {
  background: #2c2c2e;
  color: #ffffff;
}

/* Shortcut Input */
.shortcut-input-wrapper {
  display: flex;
  gap: 8px;
  align-items: center;
}

.shortcut-input {
  flex: 1;
  background: var(--mac-card);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 6px;
  padding: 8px 12px;
  color: var(--mac-text);
  font-size: 13px;
  font-family: monospace;
  cursor: pointer;
}

.shortcut-input:focus {
  outline: none;
  border-color: var(--mac-accent);
  box-shadow: 0 0 0 2px rgba(10, 132, 255, 0.2);
}

[data-theme='dark'] .shortcut-input {
  background: #2c2c2e;
}

.clear-shortcut-btn {
  padding: 6px;
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 6px;
  cursor: pointer;
  color: var(--mac-text);
  opacity: 0.6;
  transition: all 0.2s;
}

.clear-shortcut-btn:hover {
  opacity: 1;
  background: var(--mac-card);
}

.shortcut-hint {
  font-size: 11px;
  opacity: 0.5;
  margin-top: 4px;
}

/* Radio Group */
.radio-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.radio-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 12px;
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.radio-item:hover {
  border-color: var(--mac-border);
}

.radio-item input[type="radio"] {
  margin-top: 2px;
  accent-color: var(--mac-accent);
}

.radio-label {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.radio-label strong {
  font-size: 13px;
  font-weight: 500;
}

.radio-label small {
  font-size: 11px;
  opacity: 0.5;
}

/* Footer */
.modal-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-top: 1px solid var(--mac-border);
}

.footer-status {
  display: flex;
  gap: 8px;
}

.status-badge {
  font-size: 11px;
  padding: 4px 8px;
  border-radius: 4px;
}

.status-badge.success {
  background: rgba(48, 209, 88, 0.15);
  color: #30d158;
}

.status-badge.warning {
  background: rgba(255, 159, 10, 0.15);
  color: #ff9f0a;
}

.footer-right {
  display: flex;
  gap: 10px;
}
</style>
