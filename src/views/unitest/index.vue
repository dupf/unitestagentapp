<script setup lang='ts'>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { tauri } from '@tauri-apps/api'
import { NButton, useDialog, useMessage, useNotification } from 'naive-ui'
import { save } from '@tauri-apps/api/dialog'

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
      lastText = lastText + detail ?? ''
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
  const message = Object.values(unitestStore.UnitestConfig).join('|')

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
  // console.log('messages', messages)
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

// function handleExport() {
//   const d = dialog.warning({
//     title: t('chat.exportImage'),
//     content: t('chat.exportImageConfirm'),
//     positiveText: t('common.yes'),
//     negativeText: t('common.no'),
//     onPositiveClick: async () => {
//       try {
//         d.loading = true
//         const ele = document.getElementById('image-wrapper')
//         const canvas = await html2canvas(ele as HTMLDivElement, {
//           useCORS: true,
//         })
//         const imgData = canvas.toDataURL('image/png')
//         const binaryData = atob(imgData.split('base64,')[1])
//         const data = []
//         for (let i = 0; i < binaryData.length; i++)
//           data.push(binaryData.charCodeAt(i))
//         await invoke('download_img', { name: 'ChatGPT-xxxx.jpg', blob: data })
//         // await invoke('download_report', { name: 'ChatGPT-xxxx.jpg', blob: data })
//         ms.success(t('chat.exportSuccess'))
//         Promise.resolve()
//       }
//       catch (error: any) {
//         ms.error(t('chat.exportFailed'))
//       }
//       finally {
//         d.loading = false
//       }
//     },
//   })
// }

// async function saveexportFile() {
//   // 模拟要保存的文件内
//   // const content = '/Users/mac/Documents/work/htzr/unitest_agent/uapp/unitestagentapp/src-tauri/test_results.html'
//   // '/Users/mac/Documents/work/htzr/unitesttools/unitestool/unitest_agent/test_results.html'
//   // 调用保存对话框，用户选择保存路径
//   const filePath = await save({
//     defaultPath: 'resources/saved.docx',
//   })
//   if (filePath) {
//     try {
//       ms.info('保存文件#####')
//       // 调用 Rust 后端命令，保存文件
//       try {
//         tauri.invoke('convert_html_to_word', {
//           // srcpath: '/Users/mac/Documents/work/htzr/unitest_agent/uapp/unitestagentapp/src-tauri/test_results.html',
//           srcpath: 'resources/test_results.html',
//           destpath: filePath,

//         })
//       }
//       catch (error) {
//         console.error('调用 Rust 后端命令失败:', error)
//       }
//       ms.info(filePath)
//     }
//     catch (error) {
//       console.error('保存文件失败:', error)
//     }
//   }
//   else {
//     alert('未选择保存路径')
//   }
// }

async function saveexportFile() {
  // 模拟要保存的文件内
  // const content = '/Users/mac/Documents/work/htzr/unitest_agent/uapp/unitestagentapp/src-tauri/test_results.html'
  // '/Users/mac/Documents/work/htzr/unitesttools/unitestool/unitest_agent/test_results.html'
  // 调用保存对话框，用户选择保存路径
  const filePath = await save({
    defaultPath: '/saved.html',
  })
  if (filePath) {
    try {
      ms.info('保存文件#####')
      // 调用 Rust 后端命令，保存文件
      try {
        tauri.invoke('download_report', {
          // srcpath: '/Users/mac/Documents/work/htzr/unitest_agent/uapp/unitestagentapp/src-tauri/test_results.html',
          srcpath: 'resources/test_results.html',
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
          <HoverButton v-if="!isMobile" @click="saveexportFile">
            <span class="text-xl text-[#4f555e] dark:text-white">
              <SvgIcon icon="ri:download-2-line" />
            </span>
          </HoverButton>
          <HoverButton v-if="!isMobile" @click="toggleUsingContext">
            <span class="text-xl" :class="{ 'text-[#4b9e5f]': usingContext, 'text-[#a8071a]': !usingContext }">
              <SvgIcon icon="ri:chat-history-line" />
            </span>
          </HoverButton>
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
            </NButton>
          </NTooltip>
        </div>
      </div>
    </footer>
  </div>
</template>
