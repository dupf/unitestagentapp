<script setup lang='ts'>
import { computed, ref } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { NButton, NForm, NFormItem, NInput, useMessage } from 'naive-ui'
import { useUnitestStore } from '@/store'
import { t } from '@/locales'

const UnitestStore = useUnitestStore()
const ms = useMessage()
const formRef = ref<FormInst | null>(null)
// const userInfo = computed(() => UnitestStore.userInfo)
// const userConfig = computed(() => UnitestStore.userConfig)
const UnitestConfig = computed(() => UnitestStore.UnitestConfig)

const unitestModel = ref({
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
    }
  })

  // formRef.value?.validate((errors) => {
  //   if (!errors) {
  //     // const hostUrl = new URL(model.value.host)
  //     // userInfo.value.name = model.value.name
  //     // userInfo.value.avatar = model.value.avatar
  //     // userConfig.value.apiKey = model.value.apiKey
  //     // userConfig.value.modelName = model.value.modelName
  //     // userConfig.value.proxy = model.value.proxy
  //     // userConfig.value.host = `${hostUrl.protocol}//${hostUrl.host}`

  //     UnitestStore.recordState()

  //     ms.success(t('common.success'))
  //   }
  // })
}
</script>

<template>
  <div class="p-4 space-y-5 min-h-[200px] max-h-[400px] overflow-auto">
    <NForm ref="formRef" :model="unitestModel" :rules="rules">
      <NFormItem path="sourcefilePath" :label="$t('unitestModel.sourcefilePath')">
        <NInput v-model:value="unitestModel.sourcefilePath" :placeholder="$t('unitestModel.sourcefilePath')" />
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
      <NFormItem path="testCommandDir" :label="$t('unitestModel.testCommandDir')">
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
      <!-- <NFormItem path="desiredCoverage" :label="$t('unitestModel.desiredCoverage')">
        <NInput v-model:value="unitestModel.desiredCoverage" :placeholder="$t('unitestModel.desiredCoverage')" />
      </NFormItem> -->
      <!-- <NFormItem path="maxIterations" :label="$t('unitestModel.maxIterations')">
        <NInput v-model:value="unitestModel.maxIterations" :placeholder="$t('unitestModel.maxIterations')" />
      </NFormItem> -->
      <NFormItem path="additionalInstructions" :label="$t('unitestModel.additionalInstructions')">
        <NInput
          v-model:value="unitestModel.additionalInstructions"
          :placeholder="$t('unitestModel.additionalInstructions')"
        />
      </NFormItem>
      <NFormItem path="model" :label="$t('unitestModel.model')">
        <NInput v-model:value="unitestModel.model" :placeholder="$t('unitestModel.model')" />
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
