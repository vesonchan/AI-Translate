<template>
  <div class="custom-select" ref="dropdownRef">
    <button
      type="button"
      class="custom-select-trigger"
      :class="{ disabled }"
      @click="toggleDropdown"
      :disabled="disabled"
    >
      <transition name="fade-text" mode="out-in">
        <span class="selected-label" :key="selectedLabel">
          {{ selectedLabel }}
        </span>
      </transition>
      <svg
        class="chevron"
        :class="{ open: isOpen }"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path d="M6 9l6 6 6-6" />
      </svg>
    </button>
    <transition name="fade-slide">
      <ul v-if="isOpen" class="custom-select-options">
        <li
          v-for="option in options"
          :key="option.value"
          :class="['custom-option', { active: option.value === modelValue }]"
          @click="selectOption(option.value)"
        >
          {{ option.label }}
        </li>
      </ul>
    </transition>
  </div>
</template>

<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

const props = defineProps({
  modelValue: {
    type: [String, Number, null],
    default: ''
  },
  options: {
    type: Array,
    default: () => []
  },
  placeholder: {
    type: String,
    default: '请选择'
  },
  disabled: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:modelValue', 'change'])

const isOpen = ref(false)
const dropdownRef = ref(null)

const selectedLabel = computed(() => {
  const current = props.options.find((option) => option.value === props.modelValue)
  return current ? current.label : props.placeholder
})

const toggleDropdown = () => {
  if (props.disabled) return
  isOpen.value = !isOpen.value
}

const selectOption = (value) => {
  emit('update:modelValue', value)
  emit('change', value)
  isOpen.value = false
}

const handleClickOutside = (event) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target)) {
    isOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.custom-select {
  position: relative;
}

.custom-select-trigger {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 8px;
  background: var(--mac-btn-bg);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  color: var(--mac-text);
}

.custom-select-trigger:hover:not(.disabled) {
  background: var(--mac-card);
  border-color: rgba(0, 122, 255, 0.3);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.custom-select-trigger:focus-visible {
  outline: none;
  border-color: var(--mac-accent);
  box-shadow: 0 0 0 2px rgba(0, 122, 255, 0.2);
}

.custom-select-trigger.disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.selected-label {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.chevron {
  transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  color: var(--mac-text);
  opacity: 0.3;
}

.chevron.open {
  transform: rotate(180deg);
  color: var(--mac-accent);
  opacity: 1;
}

.custom-select-options {
  position: absolute;
  z-index: 2000;
  width: 100%;
  box-sizing: border-box;
  margin: 6px 0 0;
  padding: 6px;
  background: var(--mac-toolbar-bg);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid var(--mac-toolbar-border);
  border-radius: 12px;
  box-shadow: var(--mac-shadow);
  max-height: 240px;
  overflow-y: auto;
}

.custom-select-options::-webkit-scrollbar {
  width: 4px;
}

.custom-select-options::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 2px;
}

[data-theme='dark'] .custom-select-options::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
}

.custom-option {
  padding: 8px 12px;
  border-radius: 6px;
  font-size: 13px;
  color: var(--mac-text);
  cursor: pointer;
  transition: all 0.15s ease;
  margin-bottom: 2px;
}

.custom-option:last-child {
  margin-bottom: 0;
}

.custom-option:hover {
  background: var(--mac-accent);
  color: white;
}

.custom-option.active {
  background: rgba(0, 122, 255, 0.1);
  color: var(--mac-accent);
  font-weight: 600;
}

.custom-option.active:hover {
  background: var(--mac-accent);
  color: white;
}

.fade-text-enter-active,
.fade-text-leave-active {
  transition: opacity 0.2s ease;
}

.fade-text-enter-from,
.fade-text-leave-to {
  opacity: 0;
}

.fade-slide-enter-active,
.fade-slide-leave-active {
  transition: opacity 0.2s cubic-bezier(0.4, 0, 0.2, 1), transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.fade-slide-enter-from,
.fade-slide-leave-to {
  opacity: 0;
  transform: translateY(-8px) scale(0.98);
}
</style>

