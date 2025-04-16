<script lang="ts" setup>
import { computed, nextTick } from 'vue'
import { HoverButton, SvgIcon } from '@/components/common'
import { useAppStore, useChatStore } from '@/store'

interface Props {
  usingContext: boolean
}

interface Emit {
  (ev: 'export'): void
  (ev: 'toggleUsingContext'): void
  (ev: 'saveFile'): void
}

defineProps<Props>()

const emit = defineEmits<Emit>()

const appStore = useAppStore()
const chatStore = useChatStore()

const collapsed = computed(() => appStore.siderCollapsed)
const currentChatHistory = computed(() => chatStore.getChatHistoryByCurrentActive)

function handleUpdateCollapsed() {
  appStore.setSiderCollapsed(!collapsed.value)
}

function onScrollToTop() {
  const scrollRef = document.querySelector('#scrollRef')
  if (scrollRef)
    nextTick(() => scrollRef.scrollTop = 0)
}

function handleExport() {
  emit('export')
}

function toggleUsingContext() {
  emit('toggleUsingContext')
}

function handleSaveFile() {
  emit('saveFile')
}
</script>

<template>
  <header
    class="sticky top-0 left-0 right-0 z-30 border-b dark:border-neutral-800 bg-white/80 dark:bg-black/20 backdrop-blur"
  >
    <div class="relative flex items-center justify-between min-w-0 overflow-hidden h-14">
      <div class="flex items-center">
        <button
          class="flex items-center justify-center w-11 h-11"
          @click="handleUpdateCollapsed"
        >
          <SvgIcon
            v-if="collapsed"
            class="text-2xl"
            icon="ri:align-justify"
          />
          <SvgIcon
            v-else
            class="text-2xl"
            icon="ri:align-right"
          />
        </button>
      </div>

      <div class="flex items-center space-x-2">
        <slot name="actions">
          <!-- 默认操作按钮 -->
        </slot>
        <HoverButton
          v-if="Array.isArray(currentChatHistory) && currentChatHistory.length > 0"
          @click="onScrollToTop"
        >
          <span class="text-xl text-[#4f555e] dark:text-white">
            <SvgIcon icon="ri:arrow-up-line" />
          </span>
        </HoverButton>
        <HoverButton @click="toggleUsingContext">
          <span class="text-xl" :class="{ 'text-[#4b9e5f]': usingContext, 'text-[#a8071a]': !usingContext }">
            <SvgIcon icon="ri:chat-history-line" />
          </span>
        </HoverButton>
        <HoverButton @click="handleExport">
          <span class="text-xl text-[#4f555e] dark:text-white">
            <SvgIcon icon="ri:download-2-line" />
          </span>
        </HoverButton>
        <HoverButton @click="handleSaveFile">
          <span class="text-xl text-[#4f555e] dark:text-white">
            <SvgIcon icon="ri:save-line" />
          </span>
        </HoverButton>
      </div>
    </div>
  </header>
</template>
