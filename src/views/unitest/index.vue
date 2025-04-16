<script setup lang='ts'>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { tauri } from '@tauri-apps/api'
import { NButton, useDialog, useMessage, useNotification } from 'naive-ui'
import { save } from '@tauri-apps/api/dialog'
import { appDataDir, join } from '@tauri-apps/api/path'
import { readDir } from '@tauri-apps/api/fs'

import { Message } from './components'
import { useScroll } from './hooks/useScroll'
import { useChat } from './hooks/useChat'
import { useCopyCode } from './hooks/useCopyCode'
import { useUsingContext } from './hooks/useUsingContext'
import HeaderComponent from './components/Header/index.vue'
import { HoverButton, SvgIcon } from '@/components/common'
import { useBasicLayout } from '@/hooks/useBasicLayout'
import { ChatRule, useChatStore, useUnitestStore } from '@/store'
import { fetchUnitestAPIProcess } from '@/api'
import { t } from '@/locales'

const notification = useNotification()
let controller = new AbortController()

const route = useRoute()
const dialog = useDialog()
const ms = useMessage()

const chatStore = useChatStore()

useCopyCode()

const { isMobile } = useBasicLayout()
const { addChat, addUnitest, updateChat, updateChatSome, getSessionConfig, buildRequestMessages } = useChat()
const { scrollRef, scrollToBottom } = useScroll()
const { usingContext, toggleUsingContext } = useUsingContext()
const { uuid } = route.params as { uuid: string }
const dataSources = computed(() => chatStore.getChatDataByUuid(+uuid))
const prompt = ref<string>('')
const loading = ref<boolean>(false)

// 添加单测试
const unitestStore = useUnitestStore()

// const unitest = ref<string>('')

// const { unitestList: unitestTemplate } = storeToRefs<any>(unitestStore)

// 使用storeToRefs，保证store修改后，联想部分能够重新渲染
// const { promptList: promptTemplate } = storeToRefs<any>(promptStore)

async function fetchChatMessage(messages: Chat.RequestMessage[], uuid: number, index: number) {
  const option = getSessionConfig(uuid)

  controller = new AbortController()

  let lastText = ''

  await fetchUnitestAPIProcess(
    messages,
    option,
    (detail: string, _: string) => {
      lastText = lastText + (detail ?? '')
      updateChat(+uuid, index,
        {
          dateTime: new Date().toLocaleString(),
          text: lastText,
          rule: ChatRule.Assistant,
          error: false,
          loading: false,
        },
      )
      scrollToBottom()
    },
    (error: Error) => {
      const errorMessage = error?.message ?? t('common.wrong')
      if (errorMessage === 'canceled') {
        updateChatSome(+uuid, index,
          {
            loading: false,
          },
        )
      }
      else {
        notification.error({
          content: errorMessage,
        })
        updateChat(+uuid, index,
          {
            dateTime: new Date().toLocaleString(),
            text: t('common.wrong'),
            rule: ChatRule.Assistant,
            error: true,
            loading: false,
          },
        )
      }
    },
    controller.signal)
}

async function handleSubmit() {
  let message = Object.values(unitestStore.UnitestConfig).join('|')

  message = `${+uuid}|${message}`

  // alert(message)
  // 如果正在生成，则不进行生成
  if (loading.value)
    return

  if (!message || message.trim() === '')
    return
  addUnitest(
    +uuid,
    {
      dateTime: new Date().toLocaleString(),
      text: message,
      rule: ChatRule.User,
      error: false,
    },
  )
  scrollToBottom()

  loading.value = true
  prompt.value = ''
  const messages: Chat.RequestMessage[] = await buildRequestMessages(+uuid, dataSources.value.length - 1)

  addChat(
    +uuid,
    {
      dateTime: new Date().toLocaleString(),
      text: '',
      loading: true,
      rule: ChatRule.Assistant,
      error: false,
    },
  )
  scrollToBottom()
  // alert(messages)
  // messages[0].content = `${uuid}|${messages[0].content}`
  // alert(messages[0].content)
  await fetchChatMessage(messages, +uuid, dataSources.value.length - 1)
  scrollToBottom()
  loading.value = false
}

async function onRegenerate(index: number) {
  if (loading.value)
    return

  const messages = await buildRequestMessages(+uuid, index)
  if (!messages || messages.length === 0)
    return

  loading.value = true
  updateChat(
    +uuid,
    index,
    {
      dateTime: new Date().toLocaleString(),
      text: '',
      rule: ChatRule.Assistant,
      error: false,
      loading: true,
    },
  )

  await fetchChatMessage(messages, +uuid, index)

  loading.value = false
}

async function saveexportFile() {
  // 调用保存对话框，用户选择保存路径
  const filePath = await save({
    defaultPath: './report.html',
  })
  if (filePath) {
    try {
      ms.info('保存文件')
      // 调用 Rust 后端命令，保存文件
      try {
        // 获取完整路径
        const appData = await appDataDir()
        // ms.info(+uuid)
        const reportsDir = await join(appData, 'reports')
        const fullPath = await join(reportsDir, `combined_report_${+uuid}.html`)
        // const files: any[] = await readDir(reportsDir)
        // const latestFile = files.reduce((latest: any, file: any) => {
        //   return file.modified > latest.modified ? file : latest
        // }, files[0])
        // const fullPath = latestFile.path
        tauri.invoke('download_report', {
          srcpath: fullPath,
          destpath: filePath,
        })
      }
      catch (error) {
        console.error('调用 Rust 后端命令失败:', error)
      }
      ms.info(filePath)
    }
    catch (error) {
      console.error('保存文件失败:', error)
    }
  }
  else {
    alert('未选择保存路径')
  }
}

function handleDelete(index: number) {
  if (loading.value)
    return
  dialog.warning({
    title: t('chat.deleteMessage'),
    content: t('chat.deleteMessageConfirm'),
    positiveText: t('common.yes'),
    negativeText: t('common.no'),
    onPositiveClick: () => {
      chatStore.deleteChatByUuid(+uuid, index)
    },
  })
}

function handleClear() {
  if (loading.value)
    return

  dialog.warning({
    title: t('chat.clearChat'),
    content: t('chat.clearChatConfirm'),
    positiveText: t('common.yes'),
    negativeText: t('common.no'),
    onPositiveClick: () => {
      chatStore.clearChatByUuid(+uuid)
    },
  })
}

function handleStop() {
  if (loading.value) {
    controller.abort()
    loading.value = false
  }
}

// 可优化部分
// 搜索选项计算，这里使用value作为索引项，所以当出现重复value时渲染异常(多项同时出现选中效果)
// 理想状态下其实应该是key作为索引项,但官方的renderOption会出现问题，所以就需要value反renderLabel实现
// const searchOptions = computed(() => {
//   if (prompt.value.startsWith('/')) {
//     return promptTemplate.value.filter((item: { key: string }) => item.key.toLowerCase().includes(prompt.value.substring(1).toLowerCase())).map((obj: { value: any }) => {
//       return {
//         label: obj.value,
//         value: obj.value,
//       }
//     })
//   }
//   else {
//     return []
//   }
// })
// value反渲染key
// const renderOption = (option: { label: string }) => {
//   for (const i of promptTemplate.value) {
//     if (i.value === option.label)
//       return [i.key]
//   }
//   return []
// }

// const placeholder = computed(() => {
//   if (isMobile.value)
//     return t('chat.placeholderMobile')
//   return t('chat.placeholder')
// })

const buttonDisabled = computed(() => {
  // alert(unitestStore.UnitestConfig.sourcefilePath)
  return loading.value || !unitestStore.UnitestConfig.sourcefilePath
})

const footerClass = computed(() => {
  let classes = ['p-4']
  if (isMobile.value)
    classes = ['sticky', 'left-0', 'bottom-0', 'right-0', 'p-2', 'pr-3', 'overflow-hidden']
  return classes
})

onMounted(() => {
  scrollToBottom()
})

onUnmounted(() => {
  if (loading.value)
    controller.abort()
})

async function getLatestFile(dirName = 'reports') {
  try {
    // Get the app data directory path
    const appData = await appDataDir()

    // Join the app data directory with the specified subdirectory
    const targetDir = await join(appData, dirName)

    // Read all files in the directory
    const files = await readDir(targetDir)

    if (!files.length)
      return null // No files found

    // Sort files by modification time (most recent first)
    const sortedFiles = [...files].sort((a, b) => {
      return new Date(b.modifiedAt).getTime() - new Date(a.modifiedAt).getTime()
    })

    // Return the most recent file
    return sortedFiles[0]
  }
  catch (error) {
    console.error('Error getting latest file:', error)
    return null
  }
}

// Usage example
async function openLatestReport() {
  const latestFile = await getLatestFile()
  if (latestFile)
    console.log('Latest file path:', latestFile.path)
    // You can now open or process this file
    // e.g., open(latestFile.path) if using Tauri's open command
  else
    console.log('No files found')
}
</script>

<template>
  <div class="flex flex-col w-full h-full">
    <HeaderComponent
      v-if="isMobile"
      :using-context="usingContext"
      @export="saveexportFile"
      @toggle-using-context="toggleUsingContext"
    />
    <main class="flex-1 overflow-hidden">
      <div
        id="scrollRef"
        ref="scrollRef"
        class="h-full overflow-hidden overflow-y-auto"
      >
        <div
          id="image-wrapper"
          class="w-full max-w-screen-xl m-auto dark:bg-[#101014]"
          :class="[isMobile ? 'p-2' : 'p-4']"
        >
          <template v-if="!dataSources.length">
            <div class="flex items-center justify-center mt-4 text-center text-neutral-300">
              <SvgIcon icon="ri:bubble-chart-fill" class="mr-2 text-3xl" />
              <span>Unitest Agent~</span>
            </div>
          </template>
          <template v-else>
            <div>
              <Message
                v-for="(item, index) of dataSources"
                :key="index"
                :date-time="item.dateTime"
                :text="item.text"
                :is-bot="item.rule === ChatRule.Assistant"
                :error="item.error"
                :loading="item.loading"
                @regenerate="onRegenerate(index)"
                @delete="handleDelete(index)"
              />
              <div class="sticky bottom-0 left-0 flex justify-center">
                <NButton v-if="loading" type="warning" @click="handleStop">
                  <template #icon>
                    <SvgIcon icon="ri:stop-circle-line" />
                  </template>
                  <!-- Stop Responding -->
                  停止生成。。。
                </NButton>
              </div>
            </div>
          </template>
        </div>
      </div>
    </main>
    <footer :class="footerClass">
      <div class="w-full max-w-screen-xl m-auto">
        <div class="flex items-center justify-between space-x-2">
          <HoverButton @click="handleClear">
            <span class="text-xl text-[#4f555e] dark:text-white">
              <SvgIcon icon="ri:delete-bin-line" />
            </span>
          </HoverButton>

          <!-- <HoverButton v-if="!isMobile" @click="saveexportFile">
            <span class="text-xl text-[#4f555e] dark:text-white">
              <SvgIcon icon="ri:download-2-line" />
            </span>
          </HoverButton> -->

          <NTooltip content="下载检测报告">
            <NButton type="primary" :disabled="buttonDisabled" @click="saveexportFile">
              <template #icon>
                <span class="dark:text-black">
                  <!-- <SvgIcon icon="ri:send-plane-fill" /> -->
                  <SvgIcon icon="ri:download-2-line" />
                </span>
              </template>
              检测报告下载
            </NButton>
          </NTooltip>

          <!-- <HoverButton v-if="!isMobile" @click="toggleUsingContext">
            <span class="text-xl" :class="{ 'text-[#4b9e5f]': usingContext, 'text-[#a8071a]': !usingContext }">
              <SvgIcon icon="ri:chat-history-line" />
            </span>
          </HoverButton> -->

          <!-- <NAutoComplete v-model:value="prompt" :options="searchOptions" :render-label="renderOption">
            <template #default="{ handleInput, handleBlur, handleFocus }">
              <NInput
                v-model:value="prompt"
                type="textarea"
                :placeholder="placeholder"
                :autosize="{ minRows: 1, maxRows: isMobile ? 4 : 8 }"
                @input="handleInput"
                @focus="handleFocus"
                @blur="handleBlur"
                @keypress="handleEnter"
              />
            </template>
          </NAutoComplete> -->
          <!-- <NButton type="primary" :disabled="buttonDisabled" :name="生成测" @click="handleSubmit"> -->
          <!-- <NButton type="primary" :disabled="buttonDisabled"  @click="handleSubmit"> -->

          <NTooltip content="生成单元测试">
            <NButton type="primary" :disabled="buttonDisabled" @click="handleSubmit">
              <template #icon>
                <span class="dark:text-black">
                  <SvgIcon icon="ri:send-plane-fill" />
                </span>
              </template>
              执行用例
            </NButton>
          </NTooltip>
        </div>
      </div>
    </footer>
  </div>
</template>
