<script setup lang='ts'>
import { computed, defineProps, onMounted, ref } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { NButton, NForm, NFormItem, NInput, NSelect, useMessage } from 'naive-ui'
import { dialog, window as tauriWindow } from '@tauri-apps/api'
import { useUnitestStore } from '@/store'
import { t } from '@/locales'

// 定义组件 props
const props = defineProps({
  onClose: { type: Function, required: false },
})

const UnitestStore = useUnitestStore()
const ms = useMessage()
const formRef = ref<FormInst | null>(null)

// 获取当前窗口参数，检查是否是编辑模式
const isEditMode = ref(false)
const editProjectId = ref<string | null>(null)

onMounted(async () => {
  try {
    // 获取窗口参数，检查是否有projectId参数
    const label = await tauriWindow.getCurrent().label
    if (label.includes('edit')) {
      isEditMode.value = true
      // 从label中提取projectId
      const match = label.match(/edit-(.*)/)
      if (match && match[1]) {
        editProjectId.value = match[1]
        // 加载项目配置
        loadProjectConfig(match[1])
      }
    }
  }
  catch (error) {
    console.error('Failed to get window params:', error)
  }
})

// 加载项目配置
function loadProjectConfig(projectId: string) {
  // 这里应该从UnitestStore中获取指定ID的项目配置
  // 目前简化处理，直接使用当前配置
  // 实际实现时，应该根据projectId获取对应的配置
}

const UnitestConfig = computed(() => UnitestStore.UnitestConfig)

const unitestModel = ref({
  name: UnitestConfig.value.name,
  scanMode: UnitestConfig.value.scanMode,

  sourcefilePath: UnitestConfig.value.sourcefilePath,
  testfilePath: UnitestConfig.value.testfilePath,
  testfileOutputPath: UnitestConfig.value.testfileOutputPath,
  codecoveragereportPath: UnitestConfig.value.codecoveragereportPath,
  testCommand: UnitestConfig.value.testCommand,
  testCommandDir: UnitestConfig.value.testCommandDir,
  includedFiles: UnitestConfig.value.includedFiles,
  coverageType: UnitestConfig.value.coverageType,
  reportFilepath: UnitestConfig.value.reportFilepath,
  desiredCoverage: UnitestConfig.value.desiredCoverage,
  maxIterations: UnitestConfig.value.maxIterations,
  additionalInstructions: UnitestConfig.value.additionalInstructions,
  model: UnitestConfig.value.model,
  isRemote: UnitestConfig.value.isRemote ? 'true' : 'false',
})

async function handleSelectSourceDirectory() {
  const selected = await dialog.open({
    directory: false,
    multiple: false,
    title: t('unitestModel.selectSourceFile'),
    filters: [{
      name: 'Source files',
      extensions: ['c', 'cpp', 'py', 'rs', 'java', 'go', 'js', 'ts'],
    }],
  })
  if (typeof selected === 'string') {
    if (unitestModel?.value)
      unitestModel.value.sourcefilePath = selected
  }
}
const rules: FormRules = {
  name: [
    {
      required: true,
      message: t('setting.namePlaceholder'),
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error(t('setting.nameNotEmptyError'))

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  scanMode: [
    {
      required: true,
      message: t('unitestModel.scanModeRequired'),
      trigger: ['change'],
    },
  ],
  sourcefilePath: [
    {
      required: true,
      message: t('setting.namePlaceholder'),
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error(t('setting.nameNotEmptyError'))

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  proxy: [{
    required: false,
    validator(rule: FormItemRule, value: string) {
      if (!value || value.length === 0)
        return true

      else if (!/^(socks5):\/\/.+$/.test(value))
        return new Error('Proxy must start with socks5://')
      return true
    },
    trigger: ['input', 'blur'],
  }],
  apiKey: [
    {
      required: true,
      message: '请输入 api-key',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        else if (!/^\w+-\w+$/.test(value))
          return new Error('请输入正确的api-key')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  host: [
    {
      required: true,
      message: '请输入openai api host',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        else if (!/^(http|https):\/\/[^ "]+(:\d+)?$/i.test(value))
          return new Error('请输入正确的host')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
}

function saveUnitestInfo() {
  formRef.value?.validate((errors) => {
    if (!errors) {
      UnitestStore.UnitestConfig.name = unitestModel.value.name
      UnitestStore.UnitestConfig.scanMode = unitestModel.value.scanMode
      UnitestStore.UnitestConfig.sourcefilePath = unitestModel.value.sourcefilePath
      UnitestStore.UnitestConfig.testfilePath = unitestModel.value.testfilePath
      UnitestStore.UnitestConfig.testfileOutputPath = unitestModel.value.testfileOutputPath
      UnitestStore.UnitestConfig.codecoveragereportPath = unitestModel.value.codecoveragereportPath
      UnitestStore.UnitestConfig.testCommand = unitestModel.value.testCommand
      UnitestStore.UnitestConfig.testCommandDir = unitestModel.value.testCommandDir
      UnitestStore.UnitestConfig.includedFiles = unitestModel.value.includedFiles
      UnitestStore.UnitestConfig.coverageType = unitestModel.value.coverageType
      UnitestStore.UnitestConfig.reportFilepath = unitestModel.value.reportFilepath
      UnitestStore.UnitestConfig.desiredCoverage = unitestModel.value.desiredCoverage
      UnitestStore.UnitestConfig.maxIterations = unitestModel.value.maxIterations
      UnitestStore.UnitestConfig.additionalInstructions = unitestModel.value.additionalInstructions
      UnitestStore.UnitestConfig.model = unitestModel.value.model
      UnitestStore.UnitestConfig.isRemote = unitestModel.value.isRemote === 'true'
      UnitestStore.recordState()

      ms.success(t('common.success'))

      // 保存成功后退出
      setTimeout(() => {
        // 如果是在弹窗中，则关闭弹窗
        if (props.onClose) {
          props.onClose()
        }
        else {
          // 否则关闭窗口或返回上一页
          try {
            // 尝试关闭当前 Tauri 窗口
            tauriWindow.getCurrent().close()
          }
          catch (error) {
            // 如果不是 Tauri 窗口或出错，尝试返回上一页
            if (window.history && window.history.length > 1)
              window.history.back()
          }
        }
      }, 500) // 短暂延迟以确保用户看到成功消息
    }
  })
}
</script>

<template>
  <div class="p-4 space-y-5 min-h-[200px] max-h-[400px] overflow-auto">
    <NForm ref="formRef" :model="unitestModel" :rules="rules">
      <div class="flex justify-between items-center mb-4">
        <h2 v-show="false" class="text-lg font-medium">
          {{ $t('unitestModel.name') }}
        </h2>
        <NButton type="primary" @click="saveUnitestInfo">
          {{ $t('common.save') }}
        </NButton>
      </div>
      <NFormItem v-show="false" path="scanMode" :label="$t('unitestModel.scanMode')">
        <NSelect
          v-model:value="unitestModel.scanMode"
          :options="[
            { label: $t('unitestModel.projectScan'), value: '0' },
            { label: $t('unitestModel.fileScan'), value: '1' },
          ]"
        />
      </NFormItem>
      <NFormItem path="sourcefilePath" :label="$t('unitestModel.sourcefilePath')">
        <NButton @click="handleSelectSourceDirectory">
          {{ unitestModel.sourcefilePath || $t('unitestModel.selectFile') }}
        </NButton>
      </NFormItem>

      <NFormItem path="testfilePath" :label="$t('unitestModel.testfilePath')">
        <NInput v-model:value="unitestModel.testfilePath" :placeholder="$t('unitestModel.testfilePath')" />
      </NFormItem>
      <NFormItem path="testfileOutputPath" :label="$t('unitestModel.testfileOutputPath')">
        <NInput v-model:value="unitestModel.testfileOutputPath" :placeholder="$t('unitestModel.testfileOutputPath')" />
      </NFormItem>
      <NFormItem path="codecoveragereportPath" :label="$t('unitestModel.codecoveragereportPath')">
        <NInput
          v-model:value="unitestModel.codecoveragereportPath"
          :placeholder="$t('unitestModel.codecoveragereportPath')"
        />
      </NFormItem>
      <NFormItem path="testCommand" :label="$t('unitestModel.testCommand')">
        <NInput v-model:value="unitestModel.testCommand" :placeholder="$t('unitestModel.testCommand')" />
      </NFormItem>
      <NFormItem
        v-show="unitestModel.scanMode === '0'"
        path="testCommandDir"
        :label="$t('unitestModel.testCommandDir')"
      >
        <NInput v-model:value="unitestModel.testCommandDir" :placeholder="$t('unitestModel.testCommandDir')" />
      </NFormItem>
      <NFormItem path="includedFiles" :label="$t('unitestModel.includedFiles')">
        <NInput v-model:value="unitestModel.includedFiles" :placeholder="$t('unitestModel.includedFiles')" />
      </NFormItem>
      <NFormItem path="coverageType" :label="$t('unitestModel.coverageType')">
        <NInput v-model:value="unitestModel.coverageType" :placeholder="$t('unitestModel.coverageType')" />
      </NFormItem>
      <NFormItem path="reportFilepath" :label="$t('unitestModel.reportFilepath')">
        <NInput v-model:value="unitestModel.reportFilepath" :placeholder="$t('unitestModel.reportFilepath')" />
      </NFormItem>
      <NFormItem path="desiredCoverage" :label="$t('unitestModel.desiredCoverage')">
        <NInput
          :value="unitestModel.desiredCoverage?.toString() || ''"
          :placeholder="$t('unitestModel.desiredCoverage')"
          @update:value="val => unitestModel.desiredCoverage = val ? Number(val) : 0"
        />
      </NFormItem>
      <NFormItem path="maxIterations" :label="$t('unitestModel.maxIterations')">
        <NInput
          :value="unitestModel.maxIterations?.toString() || ''"
          :placeholder="$t('unitestModel.maxIterations')"
          @update:value="val => unitestModel.maxIterations = val ? Number(val) : 0"
        />
      </NFormItem>
      <NFormItem path="additionalInstructions" :label="$t('unitestModel.additionalInstructions')">
        <NInput
          v-model:value="unitestModel.additionalInstructions"
          :placeholder="$t('unitestModel.additionalInstructions')"
        />
      </NFormItem>
      <NFormItem path="model" :label="$t('unitestModel.model')">
        <NSelect
          v-model:value="unitestModel.model"
          :options="UnitestStore.allModels().map(v => ({
            label: v,
            value: v,
          }))"
        />
      </NFormItem>
      <!-- <NFormItem path="model" :label="$t('unitestModel.model')">
        <NInput v-model:value="unitestModel.isRemote" :placeholder="$t('unitestModel.isRemote')" />
      </NFormItem> -->

      <NFormItem path="isRemote" :label="$t('unitestModel.isRemote')">
        <NSelect
          v-model:value="unitestModel.isRemote" :options="[
            { label: 'Yes', value: 'true' },
            { label: 'No', value: 'false' },
          ]"
        />
      </NFormItem>
      <!-- <NFormItem path="isRemote" :label="$t('unitestModel.isRemote')">
        <NCheckbox v-model:checked="unitestModel.model">
          {{ unitestModel.isRemote ? 'yes' : 'no' }}
        </NCheckbox>
      </NFormItem> -->

      <div class="flex items-center justify-end">
        <NButton size="small" @click="saveUnitestInfo">
          {{ $t('unitestModel.saveUnitestInfo') }}
        </NButton>
      </div>
    </NForm>
  </div>
</template>
