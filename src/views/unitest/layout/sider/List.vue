<script setup lang='ts'>
import { computed, ref } from 'vue'
import { NInput, NPopconfirm, NScrollbar, NTooltip, useMessage } from 'naive-ui'
import { tauri } from '@tauri-apps/api'
import { SvgIcon } from '@/components/common'
import { useAppStore, useChatStore } from '@/store'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { copyText } from '@/utils/format'

const { isMobile } = useBasicLayout()

const appStore = useAppStore()
const chatStore = useChatStore()

const dataSources = computed(() => chatStore.history)

const showUuidTooltip = ref(false)
const currentUuid = ref<number | null>(null)

const message = useMessage()

async function handleSelect({ uuid }: Chat.History) {
  if (isActive(uuid))
    return

  if (chatStore.active)
    chatStore.updateHistory(chatStore.active, { isEdit: false })
  await chatStore.setActive(uuid)

  if (isMobile.value)
    appStore.setSiderCollapsed(true)
}

function handleEdit({ uuid }: Chat.History, isEdit: boolean, event?: MouseEvent) {
  event?.stopPropagation()
  chatStore.updateHistory(uuid, { isEdit })
}

function handleEditProject({ uuid }: Chat.History, event?: MouseEvent) {
  event?.stopPropagation()
  // 打开项目编辑窗口，传递项目ID
  // 将import语句移动到文件顶部

  tauri.invoke('new_window', {
    title: '编辑项目',
    label: `edit-${uuid}`,
    url: '/#/window/project-store',
  })
}

function handleDelete(index: number, event?: MouseEvent | TouchEvent) {
  event?.stopPropagation()
  chatStore.deleteHistory(index)
}

function handleEnter({ uuid }: Chat.History, isEdit: boolean, event: KeyboardEvent) {
  event?.stopPropagation()
  if (event.key === 'Enter')
    chatStore.updateHistory(uuid, { isEdit })
}

function isActive(uuid: number) {
  return chatStore.active === uuid
}

function handleMouseEnter(uuid: number) {
  currentUuid.value = uuid
  showUuidTooltip.value = true
}

function handleMouseLeave() {
  showUuidTooltip.value = false
}

function handleCopyUuid(uuid: number, event: MouseEvent) {
  event.stopPropagation()
  copyText({ text: uuid.toString() })
  message.success('UUID已复制到剪贴板')
}
</script>

<template>
  <NScrollbar class="px-4">
    <div class="flex flex-col gap-2 text-sm">
      <template v-if="!dataSources.length">
        <div class="flex flex-col items-center mt-4 text-center text-neutral-300">
          <SvgIcon icon="ri:inbox-line" class="mb-2 text-3xl" />
          <span>{{ $t('common.noData') }}</span>
        </div>
      </template>
      <template v-else>
        <div v-for="(item, index) of dataSources" :key="index">
          <NTooltip :show="showUuidTooltip && currentUuid === item.uuid" placement="right">
            <template #trigger>
              <a
                class="relative flex items-center gap-3 px-3 py-3 break-all border rounded-md cursor-pointer hover:bg-neutral-100 group dark:border-neutral-800 dark:hover:bg-[#24272e]"
                :class="isActive(item.uuid) && ['border-[#4b9e5f]', 'bg-neutral-100', 'text-[#4b9e5f]', 'dark:bg-[#24272e]', 'dark:border-[#4b9e5f]', 'pr-14']"
                @click="handleSelect(item)"
                @mouseenter="handleMouseEnter(item.uuid)"
                @mouseleave="handleMouseLeave"
              >
                <span>
                  <SvgIcon icon="ri:message-3-line" />
                </span>
                <div class="relative flex-1 overflow-hidden break-all text-ellipsis whitespace-nowrap">
                  <NInput
                    v-if="item.isEdit"
                    v-model:value="item.title"
                    size="tiny"
                    @keypress="handleEnter(item, false, $event)"
                  />
                  <span v-else>{{ item.title }}</span>
                </div>
                <div v-if="isActive(item.uuid)" class="absolute z-10 flex visible right-1">
                  <template v-if="item.isEdit">
                    <button class="p-1" @click="handleEdit(item, false, $event)">
                      <SvgIcon icon="ri:save-line" />
                    </button>
                  </template>
                  <template v-else>
                    <button class="p-1">
                      <SvgIcon icon="ri:edit-line" @click="handleEdit(item, true, $event)" />
                    </button>
                    <button class="p-1" @click="handleEditProject(item, $event)">
                      <SvgIcon icon="ri:settings-line" />
                    </button>
                    <NPopconfirm placement="bottom" @positive-click="handleDelete(index, $event)">
                      <template #trigger>
                        <button class="p-1">
                          <SvgIcon icon="ri:delete-bin-line" />
                        </button>
                      </template>
                      {{ $t('chat.deleteHistoryConfirm') }}
                    </NPopconfirm>
                  </template>
                </div>
              </a>
            </template>
            <div class="flex items-center">
              <span>UUID: {{ item.uuid }}</span>
              <button
                class="ml-2 p-1 text-sm text-blue-500 hover:text-blue-700"
                @click="handleCopyUuid(item.uuid, $event)"
              >
                <SvgIcon icon="ri:file-copy-line" />
              </button>
            </div>
          </NTooltip>
        </div>
      </template>
    </div>
  </NScrollbar>
</template>
