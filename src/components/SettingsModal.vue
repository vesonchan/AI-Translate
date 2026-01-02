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
        <div class="settings-section translation-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>翻译设置</h4>
              </div>
              <span class="card-badge">
                {{ localConfig.translation.service === 'openai' ? 'OpenAI兼容' : '自定义' }}
              </span>
            </div>

            <div class="card-body">
              <div class="card-grid">
                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>翻译服务</span>
                    <select v-model="localConfig.translation.service" class="setting-select">
                      <option value="openai">OpenAI兼容模式</option>
<!--                      <option value="google">Google 翻译</option>-->
<!--                      <option value="baidu">百度翻译</option>-->
<!--                      <option value="youdao">有道翻译</option>-->
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

                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>模型ID</span>
                    <div class="model-input-group">
                      <input 
                        type="text" 
                        v-model="localConfig.translation.model_id"
                        class="setting-input"
                        placeholder="gpt-5-nano"
                      >
                      <button
                        type="button"
                        class="model-fetch-btn"
                        :disabled="!canFetchTranslationModels || translationModelsLoading"
                        @click="openTranslationModelModal"
                      >
                        获取
                      </button>
                    </div>
                  </label>
                  <p
                    v-if="translationModelsError"
                    class="setting-hint setting-hint-error"
                  >
                    {{ translationModelsError }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="settings-section token-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>Token限制</h4>
              </div>
              <label class="toggle-switch">
                <input 
                  type="checkbox" 
                  v-model="localConfig.token_limits.enable_user_max_tokens"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">启用自定义</span>
              </label>
            </div>

            <div class="card-body">
              <div 
                class="setting-item"
                v-if="localConfig.token_limits.enable_user_max_tokens"
              >
                <label class="setting-label">
                  <span>最大输出Token</span>
                  <input
                    type="number"
                    min="1000"
                    step="100"
                    class="setting-input"
                    v-model.number="localConfig.token_limits.user_max_tokens"
                    placeholder="4096"
                  >
                </label>
                <p class="setting-hint">
                  动态 max_tokens 至少为 {{ MIN_MAX_TOKENS }}，启用后不会超过此上限
                </p>
              </div>
              <p
                v-else
                class="setting-hint"
              >
                关闭自定义时根据文本长度自动估算；开启后可输入自定义上限（不少于 {{ MIN_MAX_TOKENS }}）
              </p>
            </div>
          </div>
        </div>

        <div class="settings-section ocr-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>OCR设置</h4>
              </div>
              <label class="toggle-switch">
                <input 
                  type="checkbox" 
                  v-model="localConfig.ocr.reuse_translation"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">复用翻译设置</span>
              </label>
            </div>
            
            <div class="card-body">
              <div class="card-grid">
                <template v-if="!localConfig.ocr.reuse_translation">
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
                </template>

                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>OCR模型ID</span>
                    <div class="model-input-group">
                      <input 
                        type="text" 
                        v-model="localConfig.ocr.model_id"
                        class="setting-input"
                        placeholder="gpt-4-vision-preview"
                      >
                      <button
                        type="button"
                        class="model-fetch-btn"
                        :disabled="!canFetchOcrModels || ocrModelsLoading"
                        @click="openOcrModelModal"
                      >
                        获取
                      </button>
                    </div>
                  </label>
                  <!-- Removed inline select -->
                  <p
                    v-if="ocrModelsError"
                    class="setting-hint setting-hint-error"
                  >
                    {{ ocrModelsError }}
                  </p>
                  <p
                    v-else-if="localConfig.ocr.reuse_translation"
                    class="setting-hint"
                  >
                    当前复用翻译配置，将使用翻译的Base URL和API Key获取模型
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="settings-section network-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>网络设置</h4>
              </div>
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  v-model="localConfig.proxy.enabled"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">使用代理</span>
              </label>
            </div>

            <div class="card-body">
              <div v-if="localConfig.proxy.enabled" class="card-grid">
                <div class="setting-item grid-span-2">
                  <label class="setting-label">
                    <span>代理模式</span>
                    <select v-model="localConfig.proxy.mode" class="setting-select">
                      <option value="system">使用系统代理</option>
                      <option value="https">使用HTTPS代理</option>
                      <option value="http">使用HTTP代理</option>
                      <option value="socks5">使用SOCKS5代理</option>
                    </select>
                  </label>
                </div>

                <div
                  class="setting-item grid-span-2"
                  v-if="localConfig.proxy.mode !== 'system'"
                >
                  <label class="setting-label">
                    <span>代理地址</span>
                    <input
                      type="text"
                      v-model="localConfig.proxy.server"
                      class="setting-input"
                      :placeholder="proxyPlaceholder"
                    >
                  </label>
                  <p class="setting-hint">
                    请输入完整的代理URL，例如 http://127.0.0.1:7890
                  </p>
                </div>
              </div>
              <p v-else class="setting-hint">
                当前未启用代理，默认使用系统直连配置
              </p>
            </div>
          </div>
        </div>

        <div class="settings-section startup-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>启动设置</h4>
                <p class="card-subtitle">控制系统启动时是否自动运行</p>
              </div>
              <label class="toggle-switch">
                <input
                  type="checkbox"
                  v-model="localConfig.autostart.enabled"
                  class="switch-input"
                >
                <span class="switch-track">
                  <span class="switch-thumb"></span>
                </span>
                <span class="switch-label">开机启动</span>
              </label>
            </div>
            <div class="card-body">
              <p class="setting-hint">
                启用后，应用会在系统登录完成后随系统自动启动，可直接从状态栏打开翻译窗口。
              </p>
            </div>
          </div>
        </div>

        <div class="settings-section update-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>软件更新</h4>
                <p class="card-subtitle">当前版本：{{ currentVersion || '加载中...' }}</p>
              </div>
              <button
                class="btn btn-secondary"
                type="button"
                :disabled="isCheckingUpdate || isInstallingUpdate"
                @click="handleManualUpdateCheck"
              >
                {{ isCheckingUpdate ? '检查中...' : '检查更新' }}
              </button>
            </div>
            <div class="card-body">
              <div class="version-info">
                <div class="version-row">
                  <span class="version-label">当前版本</span>
                  <span class="version-value">{{ currentVersion || '加载中...' }}</span>
                </div>
                <div class="version-row">
                  <span class="version-label">服务器版本</span>
                  <span class="version-value">{{ latestVersionDisplay }}</span>
                </div>
              </div>

              <div class="update-status-line">
                <span :class="['update-status-pill', updateStatusClass]">
                  {{ updateStatusMessage }}
                </span>
              </div>

              <div
                class="update-details"
                v-if="availableUpdateInfo"
              >
                <p class="setting-hint">
                  发现新版本：{{ availableUpdateInfo.version }}
                  <span v-if="availableUpdateInfo.date">（{{ availableUpdateInfo.date }}）</span>
                </p>
                <p
                  v-if="availableUpdateInfo.notes"
                  class="update-notes"
                >
                  {{ availableUpdateInfo.notes }}
                </p>
              </div>

              <div
                v-if="showUpdateProgress"
                class="update-progress"
              >
                <div class="update-progress-bar">
                  <div
                    class="update-progress-bar__value"
                    :style="{ width: `${updateProgress}%` }"
                  ></div>
                </div>
                <p class="setting-hint">
                  {{ updateProgressLabel }}
                </p>
              </div>

              <p
                v-if="updateError"
                class="setting-hint setting-hint-error"
              >
                {{ updateError }}
              </p>

              <div
                class="update-actions"
                v-if="canInstallUpdate"
              >
                <button
                  class="btn btn-primary"
                  type="button"
                  :disabled="isInstallingUpdate"
                  @click="downloadAndInstallUpdate"
                >
                  {{ isInstallingUpdate ? '正在更新...' : '立即安装' }}
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <div class="settings-section hotkey-card">
          <div class="card">
            <div class="card-header">
              <div>
                <h4>快捷键设置</h4>
              </div>
            </div>

            <div class="card-body">
              <div class="card-grid">
                <div class="setting-item">
                  <label class="setting-label">
                    <span>弹出窗口</span>
                    <HotkeyRecorder
                      v-model="localConfig.hotkeys.popup_window"
                      :placeholder="platformHotkeys.popup_window"
                    />
                  </label>
                  <p class="setting-hint">按下快捷键弹出翻译窗口</p>
                </div>

                <div class="setting-item">
                  <label class="setting-label">
                    <span>划词翻译</span>
                    <HotkeyRecorder
                      v-model="localConfig.hotkeys.slide_translation"
                      :placeholder="platformHotkeys.slide_translation"
                    />
                  </label>
                  <p class="setting-hint">按下快捷键启动划词翻译</p>
                </div>

                <div class="setting-item">
                  <label class="setting-label">
                    <span>截图翻译</span>
                    <HotkeyRecorder
                      v-model="localConfig.hotkeys.screenshot_translation"
                      :placeholder="platformHotkeys.screenshot_translation"
                    />
                  </label>
                  <p class="setting-hint">按下快捷键启动截图翻译</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn btn-secondary footer-reset" @click="resetToDefaults">
          恢复默认
        </button>
        <div class="footer-right">
          <button class="btn btn-secondary" @click="$emit('close')">
            取消
          </button>
          <button
            class="btn btn-primary"
            :disabled="isSaving"
            @click="saveSettings"
          >
            {{ isSaving ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>
  </div>
    <ModelSelectorModal
      :show="showTranslationModelModal"
      title="选择翻译模型"
      :models="translationModels"
      :loading="translationModelsLoading"
      :error="translationModelsError"
      @close="showTranslationModelModal = false"
      @select="selectTranslationModel"
      @retry="fetchTranslationModels"
    />

    <ModelSelectorModal
      :show="showOcrModelModal"
      title="选择OCR模型"
      :models="ocrModels"
      :loading="ocrModelsLoading"
      :error="ocrModelsError"
      @close="showOcrModelModal = false"
      @select="selectOcrModel"
      @retry="fetchOcrModels"
    />
</template>

<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { relaunch } from '@tauri-apps/plugin-process'
import { confirm } from '@tauri-apps/plugin-dialog'
import { check as checkForAppUpdates } from '@tauri-apps/plugin-updater'
import HotkeyRecorder from './HotkeyRecorder.vue'
import ModelSelectorModal from './ModelSelectorModal.vue'

const props = defineProps({
  show: Boolean,
  config: [Object, Array],
  isSaving: {
    type: Boolean,
    default: false
  },
  saveMessage: {
    type: [String, Object],
    default: ''
  }
})

const emit = defineEmits(['close', 'save'])

const isMacPlatform =
  typeof navigator !== 'undefined' &&
  /mac/i.test((navigator.userAgent || navigator.platform || '').toLowerCase())

const MIN_MAX_TOKENS = 1000

const getDefaultHotkeys = () => {
  if (isMacPlatform) {
    return {
      popup_window: "Option+A",
      slide_translation: "Option+D",
      screenshot_translation: "Option+S"
    }
  }

  return {
    popup_window: "Alt+A",
    slide_translation: "Alt+D",
    screenshot_translation: "Alt+S"
  }
}

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
  proxy: {
    enabled: false,
    mode: "system",
    server: ""
  },
  hotkeys: getDefaultHotkeys(),
  token_limits: {
    enable_user_max_tokens: false,
    user_max_tokens: 4096
  },
  autostart: {
    enabled: false
  }
}

const platformHotkeys = defaultConfig.hotkeys

const localConfig = ref(JSON.parse(JSON.stringify(defaultConfig)))
const validationError = ref('')
const translationModels = ref([])
const ocrModels = ref([])
const translationModelsLoading = ref(false)
const ocrModelsLoading = ref(false)
const translationModelsError = ref('')
const ocrModelsError = ref('')
const translationModelSelection = ref('')
const ocrModelSelection = ref('')
const showTranslationModelModal = ref(false)
const showOcrModelModal = ref(false)
const currentVersion = ref('')
const updateState = ref('idle')
const updateError = ref('')
const updateProgress = ref(0)
const availableUpdateInfo = ref(null)
const totalDownloadBytes = ref(0)
const downloadedBytes = ref(0)
const latestServerVersion = ref('')
const hasCheckedForUpdates = ref(false)
let pendingUpdate = null
const proxyPlaceholder = computed(() => {
  const mode = localConfig.value?.proxy?.mode
  if (mode === 'https') return 'https://127.0.0.1:7890'
  if (mode === 'http') return 'http://127.0.0.1:7890'
  if (mode === 'socks5') return 'socks5://127.0.0.1:7890'
  return 'http://127.0.0.1:7890'
})

const isCheckingUpdate = computed(() => updateState.value === 'checking')
const isInstallingUpdate = computed(() => ['downloading', 'installing'].includes(updateState.value))
const canInstallUpdate = computed(() => updateState.value === 'available')
const showUpdateProgress = computed(() => ['downloading', 'installing'].includes(updateState.value))

const updateStatusMessage = computed(() => {
  switch (updateState.value) {
    case 'checking':
      return '正在检查更新...'
    case 'available':
      return availableUpdateInfo.value
        ? `发现新版本 v${availableUpdateInfo.value.version}`
        : '发现新版本'
    case 'up-to-date':
      return '当前已是最新版本'
    case 'downloading':
      return '正在下载更新...'
    case 'installing':
      return '正在安装更新...'
    case 'installed':
      return '更新已安装，将自动重启'
    case 'error':
      return '检查更新失败'
    default:
      return '尚未检查更新'
  }
})

const updateStatusClass = computed(() => {
  switch (updateState.value) {
    case 'available':
      return 'info'
    case 'up-to-date':
    case 'installed':
      return 'success'
    case 'error':
      return 'error'
    case 'downloading':
    case 'installing':
      return 'warning'
    default:
      return 'muted'
  }
})

const updateProgressLabel = computed(() => {
  if (!showUpdateProgress.value) {
    return ''
  }

  if (totalDownloadBytes.value > 0) {
    const downloaded = formatBytes(downloadedBytes.value)
    const total = formatBytes(totalDownloadBytes.value)
    return `下载进度：${updateProgress.value}%（${downloaded}/${total}）`
  }

  return `下载进度：${updateProgress.value}%`
})

const latestVersionDisplay = computed(() => {
  if (isCheckingUpdate.value) {
    return '检查中...'
  }
  if (latestServerVersion.value) {
    return latestServerVersion.value
  }
  if (!hasCheckedForUpdates.value) {
    return '未检查'
  }
  if (updateError.value) {
    return '获取失败'
  }
  return currentVersion.value || '未知'
})

const formatBytes = (bytes) => {
  if (!bytes || Number.isNaN(bytes)) {
    return '0 MB'
  }
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

const formatUpdateDate = (value) => {
  if (!value) return ''
  try {
    const date = new Date(value)
    if (Number.isNaN(date.getTime())) {
      return value
    }
    return date.toLocaleDateString()
  } catch (error) {
    console.warn('格式化更新日期失败', error)
    return value
  }
}

const parseErrorMessage = (error) => {
  if (!error) return '未知错误'
  if (typeof error === 'string') return error
  if (error.message) return error.message
  try {
    return JSON.stringify(error)
  } catch (_) {
    return String(error)
  }
}

const loadCurrentVersion = async () => {
  try {
    currentVersion.value = await getVersion()
  } catch (error) {
    console.warn('获取版本号失败', error)
  }
}

const disposePendingUpdate = async () => {
  if (!pendingUpdate) {
    return
  }
  try {
    await pendingUpdate.close()
  } catch (error) {
    console.warn('释放更新资源失败', error)
  } finally {
    pendingUpdate = null
  }
}

const runUpdateCheck = async () => {
  updateError.value = ''
  updateProgress.value = 0
  availableUpdateInfo.value = null
  updateState.value = 'checking'
  hasCheckedForUpdates.value = false
  latestServerVersion.value = ''

  try {
    const update = await checkForAppUpdates()
    if (update) {
      await disposePendingUpdate()
      pendingUpdate = update
      availableUpdateInfo.value = {
        version: update.version,
        notes: (update.body || '').trim(),
        date: formatUpdateDate(update.date)
      }
      latestServerVersion.value = update.version || ''
      updateState.value = 'available'
    } else {
      await disposePendingUpdate()
      latestServerVersion.value = currentVersion.value || ''
      updateState.value = 'up-to-date'
    }
  } catch (error) {
    updateError.value = parseErrorMessage(error)
    updateState.value = 'error'
  } finally {
    hasCheckedForUpdates.value = true
    if (!latestServerVersion.value && !updateError.value) {
      latestServerVersion.value = currentVersion.value || ''
    }
  }
}

const handleManualUpdateCheck = async () => {
  if (isCheckingUpdate.value || isInstallingUpdate.value) {
    return
  }
  await runUpdateCheck()
}

const downloadAndInstallUpdate = async () => {
  if (!pendingUpdate || isInstallingUpdate.value) {
    return
  }

  const versionForConfirm = pendingUpdate.version || availableUpdateInfo.value?.version || ''
  const shouldInstall = await confirm(
    `检测到新版本${versionForConfirm ? ` v${versionForConfirm}` : ''}，是否立即下载并安装？`,
    {
      title: '确认更新',
      type: 'info',
      okLabel: '立即更新',
      cancelLabel: '暂不'
    }
  )

  if (!shouldInstall) {
    return
  }

  updateError.value = ''
  updateState.value = 'downloading'
  updateProgress.value = 0
  downloadedBytes.value = 0
  totalDownloadBytes.value = 0

  try {
    await pendingUpdate.downloadAndInstall((event) => {
      if (event.event === 'Started') {
        totalDownloadBytes.value = event.data.contentLength || 0
        downloadedBytes.value = 0
        updateProgress.value = totalDownloadBytes.value ? 0 : 10
      } else if (event.event === 'Progress') {
        downloadedBytes.value += event.data.chunkLength
        if (totalDownloadBytes.value > 0) {
          updateProgress.value = Math.min(
            100,
            Math.round((downloadedBytes.value / totalDownloadBytes.value) * 100)
          )
        } else {
          updateProgress.value = Math.min(95, updateProgress.value + 1)
        }
      } else if (event.event === 'Finished') {
        updateState.value = 'installing'
        updateProgress.value = 100
      }
    })

    updateState.value = 'installed'
    await disposePendingUpdate()
    await relaunch()
  } catch (error) {
    updateError.value = parseErrorMessage(error)
    updateState.value = 'error'
  }
}

const getOcrConfigForFetch = () => {
  if (!localConfig.value) return { base_url: '', api_key: '' }
  if (localConfig.value?.ocr?.reuse_translation) {
    return localConfig.value.translation || { base_url: '', api_key: '' }
  }
  return localConfig.value.ocr || { base_url: '', api_key: '' }
}

const canFetchTranslationModels = computed(() => {
  const translation = localConfig.value?.translation || {}
  return Boolean(translation.base_url?.trim() && translation.api_key?.trim())
})

const canFetchOcrModels = computed(() => {
  const credentials = getOcrConfigForFetch()
  return Boolean(credentials.base_url?.trim() && credentials.api_key?.trim())
})

const normalizeConfig = (config) => {
  if (!config) return null

  if (config.translation && config.ocr && config.hotkeys) {
    return config
  }

  if (
    typeof config === 'object' &&
    config !== null &&
    'value' in config &&
    config.value &&
    typeof config.value === 'object'
  ) {
    return config.value
  }

  return null
}

const mergeWithDefaults = (config = {}) => {
  const base = JSON.parse(JSON.stringify(defaultConfig))
  return {
    ...base,
    ...config,
    translation: {
      ...base.translation,
      ...(config.translation || {})
    },
    ocr: {
      ...base.ocr,
      ...(config.ocr || {})
    },
    proxy: {
      ...base.proxy,
      ...(config.proxy || {})
    },
    hotkeys: {
      ...base.hotkeys,
      ...(config.hotkeys || {})
    },
    token_limits: {
      ...base.token_limits,
      ...(config.token_limits || {})
    },
    autostart: {
      ...base.autostart,
      ...(config.autostart || {}),
      enabled: Boolean(config?.autostart?.enabled)
    }
  }
}

onMounted(() => {
  loadCurrentVersion()
})

const syncLocalConfig = () => {
  const normalized = normalizeConfig(props.config)
  if (normalized) {
    localConfig.value = mergeWithDefaults(normalized)
  } else {
    localConfig.value = JSON.parse(JSON.stringify(defaultConfig))
  }
}

watch(
  () => props.show,
  (newShow) => {
    if (newShow) {
      syncLocalConfig()
      loadCurrentVersion()
    }
  },
  { immediate: true }
)

watch(
  () => props.config,
  () => {
    if (props.show) {
      syncLocalConfig()
    }
  },
  { deep: true }
)

const resetTranslationModelsState = () => {
  translationModels.value = []
  translationModelSelection.value = ''
  translationModelsError.value = ''
}

const resetOcrModelsState = () => {
  ocrModels.value = []
  ocrModelSelection.value = ''
  ocrModelsError.value = ''
}

watch(
  () => [
    localConfig.value?.translation?.base_url,
    localConfig.value?.translation?.api_key
  ],
  resetTranslationModelsState
)

watch(
  () => [
    localConfig.value?.ocr?.base_url,
    localConfig.value?.ocr?.api_key,
    localConfig.value?.ocr?.reuse_translation,
    localConfig.value?.translation?.base_url,
    localConfig.value?.translation?.api_key
  ],
  resetOcrModelsState
)

const syncTranslationSelection = () => {
  const current = localConfig.value?.translation?.model_id || ''
  if (translationModels.value.some(model => model.id === current)) {
    translationModelSelection.value = current
  } else {
    translationModelSelection.value = ''
  }
}

const syncOcrSelection = () => {
  const current = localConfig.value?.ocr?.model_id || ''
  if (ocrModels.value.some(model => model.id === current)) {
    ocrModelSelection.value = current
  } else {
    ocrModelSelection.value = ''
  }
}

watch(
  () => localConfig.value?.translation?.model_id,
  () => syncTranslationSelection(),
  { immediate: true }
)

watch(translationModels, () => syncTranslationSelection())

watch(
  () => localConfig.value?.ocr?.model_id,
  () => syncOcrSelection(),
  { immediate: true }
)

watch(ocrModels, () => syncOcrSelection())

const normalizeBaseUrl = (url = '') => {
  return (url || '').trim().replace(/\/+$/, '')
}

const extractModelList = (payload) => {
  if (!payload) return []

  let rawList = []
  if (Array.isArray(payload)) {
    rawList = payload
  } else if (Array.isArray(payload.data)) {
    rawList = payload.data
  } else if (Array.isArray(payload.models)) {
    rawList = payload.models
  } else if (
    payload.data &&
    typeof payload.data === 'object' &&
    !Array.isArray(payload.data)
  ) {
    rawList = Object.values(payload.data)
  }

  return rawList
    .map(item => {
      if (typeof item === 'string') {
        return { id: item, label: item }
      }
      if (!item || typeof item !== 'object') {
        return null
      }
      const id = item.id || item.model || item.name || item.slug
      if (!id) {
        return null
      }
      const ownedBy = item.owned_by || item.organization || item.provider
      const label = ownedBy ? `${id} (${ownedBy})` : id
      return { id, label }
    })
    .filter(Boolean)
}

const fetchModels = async (target) => {
  const isTranslation = target === 'translation'
  const state = isTranslation
    ? {
        modelsRef: translationModels,
        loadingRef: translationModelsLoading,
        errorRef: translationModelsError,
        credentials: localConfig.value?.translation
      }
    : {
        modelsRef: ocrModels,
        loadingRef: ocrModelsLoading,
        errorRef: ocrModelsError,
        credentials: getOcrConfigForFetch()
      }

  const credentials = state.credentials || {}
  if (!credentials.base_url?.trim() || !credentials.api_key?.trim()) {
    state.errorRef.value = '请先输入Base URL和API Key'
    return
  }

  state.loadingRef.value = true
  state.errorRef.value = ''

  try {
    const endpoint = `${normalizeBaseUrl(credentials.base_url)}/models`
    const response = await fetch(endpoint, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${credentials.api_key.trim()}`,
        'Content-Type': 'application/json',
        Accept: 'application/json'
      }
    })

    if (!response.ok) {
      let errorText = ''
      try {
        errorText = await response.text()
      } catch (e) {
        console.warn('无法读取错误响应正文', e)
      }
      throw new Error(errorText || `请求失败: ${response.status}`)
    }

    const payload = await response.json()
    const list = extractModelList(payload)
    if (!list.length) {
      throw new Error('未从接口获取到模型列表')
    }

    state.modelsRef.value = list
  } catch (error) {
    const message =
      (error && error.message) ||
      (typeof error === 'string' ? error : '') ||
      '获取模型列表失败'
    state.errorRef.value = message
  } finally {
    state.loadingRef.value = false
  }
}

const fetchTranslationModels = () => fetchModels('translation')
const fetchOcrModels = () => fetchModels('ocr')

const applyModelSelection = (target, value) => {
  if (!value) return
  if (target === 'translation') {
    localConfig.value.translation.model_id = value
  } else if (target === 'ocr') {
    localConfig.value.ocr.model_id = value
  }
}

const openTranslationModelModal = () => {
  showTranslationModelModal.value = true
  fetchTranslationModels()
}

const openOcrModelModal = () => {
  showOcrModelModal.value = true
  fetchOcrModels()
}

const selectTranslationModel = (modelId) => {
  applyModelSelection('translation', modelId)
}

const selectOcrModel = (modelId) => {
  applyModelSelection('ocr', modelId)
}

const resetToDefaults = () => {
  localConfig.value = JSON.parse(JSON.stringify(defaultConfig))
}

const saveSettings = () => {
  validationError.value = ''
  const payload = JSON.parse(JSON.stringify(localConfig.value || defaultConfig))

  if (!payload.translation?.api_key?.trim()) {
    validationError.value = '请输入翻译API密钥'
    return
  }

  if (
    !payload.ocr?.reuse_translation &&
    !payload.ocr?.api_key?.trim()
  ) {
    validationError.value = '请输入OCR API密钥'
    return
  }

  if (
    payload.proxy?.enabled &&
    payload.proxy?.mode !== 'system' &&
    !payload.proxy?.server?.trim()
  ) {
    validationError.value = '请输入代理地址'
    return
  }

  const tokenLimits = {
    ...defaultConfig.token_limits,
    ...(payload.token_limits || {})
  }
  tokenLimits.user_max_tokens = Number(tokenLimits.user_max_tokens)
  if (!Number.isFinite(tokenLimits.user_max_tokens)) {
    tokenLimits.user_max_tokens = defaultConfig.token_limits.user_max_tokens
  } else {
    tokenLimits.user_max_tokens = Math.floor(tokenLimits.user_max_tokens)
  }
  if (
    tokenLimits.enable_user_max_tokens &&
    tokenLimits.user_max_tokens < MIN_MAX_TOKENS
  ) {
    validationError.value = `自定义最大Token必须不小于${MIN_MAX_TOKENS}`
    return
  }
  tokenLimits.user_max_tokens = Math.max(tokenLimits.user_max_tokens, MIN_MAX_TOKENS)
  payload.token_limits = tokenLimits

  emit('save', payload)
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
  max-width: 700px;
  max-height: 85vh;
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
  display: flex;
  align-items: center;
  justify-content: center;
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
  margin-bottom: 20px;
}

.settings-section .card {
  background: var(--mac-card);
  border: 1px solid var(--mac-border);
  border-radius: 12px;
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px 10px;
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

.card-badge {
  font-size: 10px;
  font-weight: 600;
  color: var(--mac-accent);
  background: rgba(10, 132, 255, 0.15);
  border-radius: 999px;
  padding: 2px 8px;
}

.card-body {
  padding: 16px;
}

.card-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.grid-span-2 {
  grid-column: span 2;
}

.card-grid .setting-label {
  margin-bottom: 6px;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.settings-section h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.setting-item {
  margin-bottom: 0;
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 13px;
  font-weight: 500;
}

.setting-label span {
  opacity: 0.8;
}

.setting-select,
.setting-input {
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 6px;
  padding: 8px 12px;
  color: var(--mac-text);
  font-size: 13px;
  transition: all 0.2s;
  width: 100%;
}

.setting-select:focus,
.setting-input:focus {
  outline: none;
  border-color: var(--mac-accent);
  background: var(--mac-card);
  box-shadow: 0 0 0 2px rgba(10, 132, 255, 0.2);
}

.setting-input[readonly] {
  opacity: 0.5;
  cursor: not-allowed;
}

.setting-hint {
  margin-top: 6px;
  font-size: 11px;
  opacity: 0.5;
  line-height: 1.4;
}

.model-input-group {
  display: flex;
  gap: 8px;
  align-items: center;
}

.model-input-group .setting-input {
  flex: 1;
  min-width: 0;
}

.model-input-group button {
  flex-shrink: 0;
  white-space: nowrap;
  padding: 8px 12px;
  background: var(--mac-accent);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.model-input-group button:hover:not(:disabled) {
  filter: brightness(1.1);
}

.model-input-group button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.setting-hint-error {
  color: #ff453a;
  opacity: 1;
}

/* Toggle Switch */
.toggle-switch {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  user-select: none;
}

.switch-input {
  display: none;
}

.switch-track {
  width: 36px;
  height: 20px;
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 10px;
  position: relative;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.switch-thumb {
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  position: absolute;
  top: 1px;
  left: 1px;
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.switch-input:checked + .switch-track {
  background: var(--mac-accent);
  border-color: var(--mac-accent);
}

.switch-input:checked + .switch-track .switch-thumb {
  transform: translateX(16px);
}

/* Update UI */
.update-status-pill {
  padding: 4px 10px;
  border-radius: 10px;
  font-size: 11px;
  font-weight: 600;
  display: inline-block;
  background: var(--mac-btn-bg);
}

.update-status-pill.info { background: rgba(10, 132, 255, 0.15); color: #0a84ff; }
.update-status-pill.success { background: rgba(48, 209, 88, 0.15); color: #30d158; }
.update-status-pill.warning { background: rgba(255, 159, 10, 0.15); color: #ff9f0a; }
.update-status-pill.error { background: rgba(255, 69, 58, 0.15); color: #ff453a; }

.update-notes {
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid var(--mac-border);
  border-radius: 8px;
  padding: 10px;
  font-size: 12px;
  margin-top: 8px;
  max-height: 100px;
  overflow-y: auto;
}

.update-progress-bar {
  height: 6px;
  background: var(--mac-btn-bg);
  border-radius: 3px;
  overflow: hidden;
  margin: 10px 0;
}

.update-progress-bar__value {
  height: 100%;
  background: var(--mac-accent);
  transition: width 0.3s ease;
}

/* Buttons */
.btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: var(--mac-accent);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
  transform: translateY(-1px);
}

.btn-secondary {
  background: var(--mac-btn-bg);
  color: var(--mac-text);
  border: 1px solid var(--mac-toolbar-border);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--mac-card);
}

.btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.model-fetch-btn {
  background: var(--mac-btn-bg);
  border: 1px solid var(--mac-toolbar-border);
  color: var(--mac-text);
  border-radius: 6px;
  padding: 0 12px;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.model-fetch-btn:hover:not(:disabled) {
  background: var(--mac-card);
  border-color: var(--mac-border);
}

/* Custom styles for HotkeyRecorder if needed */
:deep(.hotkey-recorder) {
  background: var(--mac-btn-bg) !important;
  border: 1px solid var(--mac-toolbar-border) !important;
  color: var(--mac-text) !important;
}

.modal-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--mac-border);
  display: flex;
  align-items: center;
}

.footer-right {
  margin-left: auto;
  display: flex;
  gap: 12px;
}

.footer-reset {
  opacity: 0.6;
}

.footer-reset:hover {
  opacity: 1;
  color: #ff453a;
}

@media (max-width: 768px) {
  .card-grid {
    grid-template-columns: 1fr;
  }
  .grid-span-2 {
    grid-column: span 1;
  }
}
</style>
