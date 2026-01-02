<template>
  <div class="text-input">
    <div class="input-header">
      <div class="input-actions">
        <button @click="$emit('paste')" class="action-btn" title="粘贴">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
            <rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>
          </svg>
        </button>
        <button @click="$emit('clear')" class="action-btn" title="清空">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 6h18M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2m3 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6h14zM10 11v6M14 11v6"/>
          </svg>
        </button>
        <!-- Microphone Button -->
        <button 
          @click="toggleRecording"
          @mousedown="startPushToTalk"
          @mouseup="stopPushToTalk"
          @mouseleave="stopPushToTalk"
          class="action-btn mic-btn"
          :class="{ recording: isRecording }"
          :title="isRecording ? '停止录音' : '语音输入'"
        >
          <svg v-if="!isRecording" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"/>
            <path d="M19 10v2a7 7 0 0 1-14 0v-2"/>
            <line x1="12" y1="19" x2="12" y2="23"/>
            <line x1="8" y1="23" x2="16" y2="23"/>
          </svg>
          <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <rect x="6" y="6" width="12" height="12" rx="2"/>
          </svg>
        </button>
      </div>
      <button @click="$emit('translate')" class="translate-button" title="翻译">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M5 8l6 6m0 0l6-6m-6 6V3"/>
          <path d="M5 16l6-6m0 0l6 6m-6-6v9"/>
        </svg>
        <span>翻译</span>
      </button>
    </div>
    <div class="input-body">
      <div class="textarea-scroll">
        <div class="textarea-container">
          <textarea
            v-model="inputText"
            @input="handleInput"
            @keydown="handleKeydown"
            placeholder="请输入要翻译的文本..."
            class="input-textarea"
            :disabled="isOcrProcessing"
          ></textarea>
          <!-- OCR Loading Indicator -->
          <div v-if="isOcrProcessing" class="ocr-loading">
            <div class="ocr-loading-spinner"></div>
            <span>正在识别文字...</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  isOcrProcessing: {
    type: Boolean,
    default: false
  },
  recordingMode: {
    type: String,
    default: 'toggle' // 'toggle' or 'push'
  }
})

const emit = defineEmits(['update:modelValue', 'paste', 'clear', 'translate'])

// Listen for backend alerts
let unlistenAlert;

onMounted(async () => {
  unlistenAlert = await listen('backend-alert', (event) => {
    console.log('Backend Alert:', event.payload)
    // Only alert if it looks like an error or specific status
    // To avoid spamming alerts for every step, maybe just log? 
    // User said "no reaction". Alerts are needed.
    alert('系统消息: ' + event.payload);
  });
});

onUnmounted(() => {
  if (unlistenAlert) unlistenAlert();
});

// Recording state
const isRecording = ref(false)
let isPushToTalk = false

const toggleRecording = async () => {
  alert('Frontend Debug: 按钮点击已触发 (Button Clicked)'); // Debug click
  console.log('toggleRecording called, isRecording:', isRecording.value)
  
  if (isRecording.value) {
    await stopRecording()
  } else {
    await startRecording()
  }
}

const startPushToTalk = async () => {
  if (props.recordingMode !== 'push') return
  isPushToTalk = true
  await startRecording()
}

const stopPushToTalk = async () => {
  if (!isPushToTalk) return
  isPushToTalk = false
  await stopRecording()
}

const startRecording = async () => {
  console.log('startRecording called')
  
  try {
    // Start recording directly - backend will check if model is loaded
    await invoke('start_speech_recording')
    isRecording.value = true
    console.log('Recording started successfully')
  } catch (e) {
    console.error('Failed to start recording:', e)
    // Show error to user
    const errorMsg = String(e)
    if (errorMsg.includes('model') || errorMsg.includes('Model')) {
      alert('请先在语音设置中下载并加载 Whisper 模型')
    } else {
      alert('录音启动失败: ' + errorMsg)
    }
  }
}

const stopRecording = async () => {
  console.log('stopRecording called')
  
  try {
    const text = await invoke('stop_speech_recording')
    isRecording.value = false
    console.log('Recording stopped, transcribed text:', text)
    if (text) {
      inputText.value = (inputText.value ? inputText.value + ' ' : '') + text
      emit('update:modelValue', inputText.value)
    }
  } catch (e) {
    console.error('Failed to stop recording:', e)
    isRecording.value = false
    alert('转录失败: ' + e)
  }
}

const inputText = ref(props.modelValue)

const handleInput = () => {
  emit('update:modelValue', inputText.value)
}

const handleKeydown = (event) => {
  if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
    event.preventDefault()
    emit('translate')
  }
}

// 监听props变化
watch(() => props.modelValue, (newVal) => {
  inputText.value = newVal
})
</script>

<style scoped>
.text-input {
  background: var(--mac-card);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border-radius: 12px;
  border: 1px solid var(--mac-border);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.text-input:focus-within {
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  border-color: var(--mac-accent);
}

.input-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: rgba(255, 255, 255, 0.03);
  border-bottom: 1px solid var(--mac-toolbar-border);
  z-index: 50; /* Ensure header is above other elements */
  position: relative; /* Needed for z-index */
}

/* Microphone Button Styles */
.mic-btn {
  position: relative;
}

.mic-btn.recording {
  color: #ff3b30;
  animation: pulse 1s infinite;
}

.mic-btn.recording::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 100%;
  height: 100%;
  background: rgba(255, 59, 48, 0.2);
  border-radius: 50%;
  transform: translate(-50%, -50%);
  animation: recording-pulse 1.5s ease-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.6; }
}

@keyframes recording-pulse {
  0% { transform: translate(-50%, -50%) scale(1); opacity: 0.6; }
  100% { transform: translate(-50%, -50%) scale(2); opacity: 0; }
}

.input-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  padding: 6px;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  color: var(--mac-text);
  opacity: 0.7;
}

.action-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  opacity: 1;
}

[data-theme='dark'] .action-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.input-body {
  position: relative;
  padding: 0;
  display: flex;
  flex-direction: column;
  height: 200px;
}

.textarea-scroll {
  flex: 1;
  overflow: hidden;
}

.textarea-container {
  position: relative;
  height: 100%;
}

.input-textarea {
  width: 100%;
  height: 100%;
  padding: 14px;
  border: none;
  resize: none;
  font-size: 15px;
  line-height: 1.6;
  font-family: inherit;
  overflow-y: auto;
  box-sizing: border-box;
  background: transparent;
  color: var(--mac-text);
}

.input-textarea:focus {
  outline: none;
}

.input-textarea::placeholder {
  color: var(--mac-text);
  opacity: 0.3;
}

.translate-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  background: var(--mac-accent);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 8px rgba(0, 122, 255, 0.3);
}

.translate-button:hover {
  filter: brightness(1.1);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 122, 255, 0.4);
}

.translate-button:active {
  transform: translateY(0);
}

.ocr-loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  color: var(--mac-text);
  font-size: 14px;
  background: var(--mac-bg);
  backdrop-filter: blur(10px);
  padding: 20px 30px;
  border-radius: 16px;
  box-shadow: var(--mac-shadow);
  z-index: 10;
  border: 1px solid var(--mac-border);
}

.ocr-loading-spinner {
  width: 24px;
  height: 24px;
  border: 3px solid rgba(0, 122, 255, 0.1);
  border-top: 3px solid var(--mac-accent);
  border-radius: 50%;
  animation: spin 0.8s cubic-bezier(0.4, 0, 0.2, 1) infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.input-textarea:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
