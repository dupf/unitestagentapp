<script setup lang='ts'>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { tauri } from '@tauri-apps/api'
import { NButton, useDialog, useMessage, useNotification } from 'naive-ui'
import { save } from '@tauri-apps/api/dialog'
import { appDataDir, join } from '@tauri-apps/api/path'

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

// 添加单测试
const unitestStore = useUnitestStore()
const testLoading = ref<boolean>(false)
const scanLoading = ref<boolean>(false)

// const unitest = ref<string>('')

// const { unitestList: unitestTemplate } = storeToRefs<any>(unitestStore)

// 使用storeToRefs，保证store修改后，联想部分能够重新渲染
// const { promptList: promptTemplate } = storeToRefs<any>(promptStore)

async function fetchChatMessage(messages: Chat.RequestMessage[], uuid: number, index: number, actionType: 'test' | 'scan' = 'test') {
  const option = getSessionConfig(uuid)

  controller = new AbortController()

  let lastText = ''

  // 添加超时处理
  const timeoutId = setTimeout(() => {
    console.log('请求超时，终止请求')
    controller.abort()
  }, 30000) // 30秒超时

  try {
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
        clearTimeout(timeoutId)
        const errorMessage = error?.message ?? t('common.wrong')
        console.error('API调用错误:', errorMessage, error)

        if (errorMessage === 'canceled') {
          updateChatSome(+uuid, index,
            {
              loading: false,
            },
          )
        }
        else {
          // 检查是否是后端崩溃
          const isBackendCrash = errorMessage.includes('panicked')
                               || errorMessage.includes('index out of bounds')
                               || errorMessage.includes('Connection refused')
                               || errorMessage.includes('broken pipe')

          const displayMessage = isBackendCrash
            ? '后端服务出现错误，请检查项目配置或稍后重试'
            : errorMessage

          notification.error({
            content: displayMessage,
            duration: 5000,
          })
          updateChat(+uuid, index,
            {
              dateTime: new Date().toLocaleString(),
              text: `❌ 执行失败: ${displayMessage}`,
              rule: ChatRule.Assistant,
              error: true,
              loading: false,
            },
          )
        }
      },
      controller.signal)
  }
  catch (error) {
    clearTimeout(timeoutId)
    console.error('fetchChatMessage 执行错误:', error)

    updateChat(+uuid, index, {
      dateTime: new Date().toLocaleString(),
      text: `❌ 系统错误: ${error.message || '未知错误'}`,
      rule: ChatRule.Assistant,
      error: true,
      loading: false,
    })
  }
  finally {
    clearTimeout(timeoutId)
  }
}

// 验证配置是否完整
function validateConfig() {
  const config = unitestStore.UnitestConfig
  const errors = []

  if (!config.sourcefilePath)
    errors.push('源文件路径不能为空')

  if (!config.testCommand)
    errors.push('测试命令不能为空')

  if (!config.model)
    errors.push('AI模型不能为空')

  return {
    isValid: errors.length === 0,
    errors,
  }
}

// 执行测试用例
async function handleRunTests() {
  console.log('开始执行测试用例...')

  // 配置验证
  const validation = validateConfig()
  if (!validation.isValid) {
    notification.error({
      content: `配置不完整: ${validation.errors.join(', ')}`,
      duration: 5000,
    })
    return
  }

  if (testLoading.value || scanLoading.value) {
    console.log('当前有其他任务正在执行，跳过')
    return
  }

  let message = Object.values(unitestStore.UnitestConfig).join('|')
  message = `TEST_EXECUTION|${+uuid}|${message}`
  console.log('测试用例消息:', message)

  if (!message || message.trim() === '') {
    console.log('消息为空，跳过执行')
    return
  }

  try {
    addUnitest(
      +uuid,
      {
        dateTime: new Date().toLocaleString(),
        text: `开始执行测试用例：${unitestStore.UnitestConfig.sourcefilePath}`,
        rule: ChatRule.User,
        error: false,
      },
    )
    scrollToBottom()

    testLoading.value = true
    console.log('开始构建请求消息...')
    const messages: Chat.RequestMessage[] = await buildRequestMessages(+uuid, dataSources.value.length - 1)
    console.log('请求消息构建完成:', messages)

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

    console.log('开始调用 fetchChatMessage...')
    await fetchChatMessage(messages, +uuid, dataSources.value.length - 1, 'test')
    console.log('fetchChatMessage 调用完成')
    scrollToBottom()
  }
  catch (error) {
    console.error('执行测试用例时出错:', error)
    updateChat(+uuid, dataSources.value.length - 1, {
      dateTime: new Date().toLocaleString(),
      text: `执行测试用例时出错: ${error.message}`,
      rule: ChatRule.Assistant,
      error: true,
      loading: false,
    })
  }
  finally {
    testLoading.value = false
    console.log('测试用例执行完成，重置loading状态')
  }
}

// 执行代码静态扫描
async function handleStaticScan() {
  console.log('开始执行代码静态扫描...')
  if (testLoading.value || scanLoading.value) {
    console.log('当前有其他任务正在执行，跳过')
    return
  }

  let message = Object.values(unitestStore.UnitestConfig).join('|')
  message = `STATIC_SCAN|${+uuid}|${message}`
  console.log('静态扫描消息:', message)

  if (!message || message.trim() === '') {
    console.log('消息为空，跳过执行')
    return
  }

  try {
    addUnitest(
      +uuid,
      {
        dateTime: new Date().toLocaleString(),
        text: `开始执行代码静态扫描：${unitestStore.UnitestConfig.sourcefilePath}`,
        rule: ChatRule.User,
        error: false,
      },
    )
    scrollToBottom()

    scanLoading.value = true
    console.log('开始构建请求消息...')
    const messages: Chat.RequestMessage[] = await buildRequestMessages(+uuid, dataSources.value.length - 1)
    console.log('请求消息构建完成:', messages)

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

    console.log('开始调用 fetchChatMessage...')
    await fetchChatMessage(messages, +uuid, dataSources.value.length - 1, 'scan')
    console.log('fetchChatMessage 调用完成')
    scrollToBottom()
  }
  catch (error) {
    console.error('执行代码静态扫描时出错:', error)
    updateChat(+uuid, dataSources.value.length - 1, {
      dateTime: new Date().toLocaleString(),
      text: `执行代码静态扫描时出错: ${error.message}`,
      rule: ChatRule.Assistant,
      error: true,
      loading: false,
    })
  }
  finally {
    scanLoading.value = false
    console.log('代码静态扫描执行完成，重置loading状态')
  }
}

async function onRegenerate(index: number) {
  if (testLoading.value || scanLoading.value)
    return

  const messages = await buildRequestMessages(+uuid, index)
  if (!messages || messages.length === 0)
    return

  testLoading.value = true
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

  testLoading.value = false
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
  if (testLoading.value || scanLoading.value)
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
  if (testLoading.value || scanLoading.value)
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
  if (testLoading.value || scanLoading.value) {
    controller.abort()
    testLoading.value = false
    scanLoading.value = false
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
  return (testLoading.value || scanLoading.value) || !unitestStore.UnitestConfig.sourcefilePath
})

const testButtonDisabled = computed(() => {
  return testLoading.value || scanLoading.value || !unitestStore.UnitestConfig.sourcefilePath
})

const scanButtonDisabled = computed(() => {
  return testLoading.value || scanLoading.value || !unitestStore.UnitestConfig.sourcefilePath
})

const isAnyLoading = computed(() => {
  return testLoading.value || scanLoading.value
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
  if (testLoading.value || scanLoading.value)
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
                <NButton v-if="isAnyLoading" type="warning" @click="handleStop">
                  <template #icon>
                    <SvgIcon icon="ri:stop-circle-line" />
                  </template>
                  停止执行...
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

          <div class="flex items-center space-x-3">
            <NTooltip content="执行测试用例">
              <NButton
                type="primary"
                :disabled="testButtonDisabled"
                :loading="testLoading"
                @click="handleRunTests"
              >
                <template #icon>
                  <span class="dark:text-black">
                    <SvgIcon icon="ri:test-tube-line" />
                  </span>
                </template>
                执行测试用例
              </NButton>
            </NTooltip>

            <NTooltip content="执行代码静态扫描">
              <NButton
                type="success"
                :disabled="scanButtonDisabled"
                :loading="scanLoading"
                @click="handleStaticScan"
              >
                <template #icon>
                  <span class="dark:text-black">
                    <SvgIcon icon="ri:bug-line" />
                  </span>
                </template>
                代码静态扫描
              </NButton>
            </NTooltip>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>
