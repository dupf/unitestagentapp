<script setup lang='ts'>
import { computed, defineProps, onMounted, ref } from 'vue'
import type { FormInst, FormRules } from 'naive-ui'
import {
  NButton,
  NCard,
  NDivider,
  NForm,
  NFormItem,
  NGrid,
  NGridItem,
  NInput,
  NInputNumber,
  NSelect,
  NSpace,
  NStep,
  NSteps,
  useMessage,
} from 'naive-ui'
import { dialog, window as tauriWindow } from '@tauri-apps/api'
import { SvgIcon } from '@/components/common'
import { useUnitestStore } from '@/store'

// 定义组件 props
const props = defineProps({
  onClose: { type: Function, required: false },
})

const UnitestStore = useUnitestStore()
const ms = useMessage()
const formRef = ref<FormInst | null>(null)
const currentStep = ref(0)

// 获取当前窗口参数，检查是否是编辑模式
const isEditMode = ref(false)
const editProjectId = ref<string | null>(null)

onMounted(async () => {
  try {
    const label = await tauriWindow.getCurrent().label
    if (label.includes('edit')) {
      isEditMode.value = true
      const match = label.match(/edit-(.*)/)
      if (match && match[1]) {
        editProjectId.value = match[1]
        loadProjectConfig(match[1])
      }
    }
  }
  catch (error) {
    console.error('Failed to get window params:', error)
  }
})

function loadProjectConfig(projectId: string) {
  // 加载项目配置逻辑
}

const UnitestConfig = computed(() => UnitestStore.UnitestConfig)

const unitestModel = ref({
  name: UnitestConfig.value.name || '',
  scanMode: UnitestConfig.value.scanMode || '1',
  sourcefilePath: UnitestConfig.value.sourcefilePath || '',
  testfilePath: UnitestConfig.value.testfilePath || '',
  testfileOutputPath: UnitestConfig.value.testfileOutputPath || '',
  codecoveragereportPath: UnitestConfig.value.codecoveragereportPath || '',
  testCommand: UnitestConfig.value.testCommand || 'npm test',
  testCommandDir: UnitestConfig.value.testCommandDir || '',
  includedFiles: UnitestConfig.value.includedFiles || '**/*.{js,ts,jsx,tsx}',
  coverageType: UnitestConfig.value.coverageType || 'lcov',
  reportFilepath: UnitestConfig.value.reportFilepath || '',
  desiredCoverage: UnitestConfig.value.desiredCoverage || 80,
  maxIterations: UnitestConfig.value.maxIterations || 3,
  additionalInstructions: UnitestConfig.value.additionalInstructions || '',
  model: UnitestConfig.value.model || 'gpt-3.5-turbo',
  isRemote: UnitestConfig.value.isRemote ? 'true' : 'false',
})

// 预设模板
const templates = [
  {
    name: 'JavaScript/Node.js',
    icon: 'ri:javascript-line',
    config: {
      testCommand: 'npm test',
      includedFiles: '**/*.{js,jsx}',
      coverageType: 'lcov',
      testfilePath: 'tests/',
      testfileOutputPath: 'tests/generated/',
    },
  },
  {
    name: 'TypeScript',
    icon: 'ri:file-code-line',
    config: {
      testCommand: 'npm run test',
      includedFiles: '**/*.{ts,tsx}',
      coverageType: 'lcov',
      testfilePath: 'src/__tests__/',
      testfileOutputPath: 'src/__tests__/generated/',
    },
  },
  {
    name: 'Python',
    icon: 'ri:file-code-line',
    config: {
      testCommand: 'pytest',
      includedFiles: '**/*.py',
      coverageType: 'coverage',
      testfilePath: 'tests/',
      testfileOutputPath: 'tests/generated/',
    },
  },
  {
    name: 'Java',
    icon: 'ri:file-code-line',
    config: {
      testCommand: 'mvn test',
      includedFiles: '**/*.java',
      coverageType: 'jacoco',
      testfilePath: 'src/test/java/',
      testfileOutputPath: 'src/test/java/generated/',
    },
  },
]

async function handleSelectSourceFile() {
  const selected = await dialog.open({
    directory: false,
    multiple: false,
    title: 'Select Source File',
    filters: [{
      name: 'Source files',
      extensions: ['c', 'cpp', 'py', 'rs', 'java', 'go', 'js', 'ts', 'jsx', 'tsx'],
    }],
  })
  if (typeof selected === 'string')
    unitestModel.value.sourcefilePath = selected
}

async function handleSelectDirectory(field: keyof typeof unitestModel.value) {
  const selected = await dialog.open({
    directory: true,
    multiple: false,
    title: `Select ${field} Directory`,
  })
  if (typeof selected === 'string')
    unitestModel.value[field] = selected
}

function applyTemplate(template: any) {
  Object.assign(unitestModel.value, template.config)
  ms.success(`Applied ${template.name} template`)
}

const rules: FormRules = {
  name: [
    {
      required: true,
      message: 'Project name is required',
      trigger: ['input', 'blur'],
    },
  ],
  sourcefilePath: [
    {
      required: true,
      message: 'Source file path is required',
      trigger: ['input', 'blur'],
    },
  ],
  testCommand: [
    {
      required: true,
      message: 'Test command is required',
      trigger: ['input', 'blur'],
    },
  ],
}

function saveUnitestInfo() {
  formRef.value?.validate((errors) => {
    if (!errors) {
      // 保存配置到store
      Object.assign(UnitestStore.UnitestConfig, {
        ...unitestModel.value,
        isRemote: unitestModel.value.isRemote === 'true',
        desiredCoverage: Number(unitestModel.value.desiredCoverage),
        maxIterations: Number(unitestModel.value.maxIterations),
      })

      UnitestStore.recordState()
      ms.success('Project configuration saved successfully!')

      setTimeout(() => {
        if (props.onClose) {
          props.onClose()
        }
        else {
          try {
            tauriWindow.getCurrent().close()
          }
          catch (error) {
            if (window.history && window.history.length > 1)
              window.history.back()
          }
        }
      }, 1000)
    }
  })
}

function nextStep() {
  if (currentStep.value < 2)
    currentStep.value++
}

function prevStep() {
  if (currentStep.value > 0)
    currentStep.value--
}
</script>

<template>
  <div class="project-config-container">
    <!-- Header -->
    <div class="config-header">
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-blue-500 rounded-lg flex items-center justify-center">
            <SvgIcon icon="ri:settings-3-line" class="text-xl text-white" />
          </div>
          <div>
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
              {{ isEditMode ? 'Edit Project' : 'New Project Configuration' }}
            </h1>
            <p class="text-gray-600 dark:text-gray-400">
              Configure your unit testing project settings
            </p>
          </div>
        </div>
        <NButton type="primary" size="large" @click="saveUnitestInfo">
          <template #icon>
            <SvgIcon icon="ri:save-line" />
          </template>
          Save Configuration
        </NButton>
      </div>
    </div>

    <!-- Progress Steps -->
    <NCard class="mb-6">
      <NSteps :current="currentStep" size="small">
        <NStep title="Basic Info" description="Project name and source files" />
        <NStep title="Test Settings" description="Configure test parameters" />
        <NStep title="Advanced" description="Coverage and AI settings" />
      </NSteps>
    </NCard>

    <!-- Main Form -->
    <NForm ref="formRef" :model="unitestModel" :rules="rules" label-placement="top">
      <!-- Step 0: Basic Information -->
      <NCard v-if="currentStep === 0" title="Basic Project Information">
        <template #header-extra>
          <SvgIcon icon="ri:information-line" class="text-blue-500" />
        </template>

        <NGrid :cols="2" :x-gap="16" :y-gap="16">
          <NGridItem>
            <NFormItem path="name" label="Project Name">
              <NInput
                v-model:value="unitestModel.name"
                placeholder="Enter project name"
                size="large"
              />
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="scanMode" label="Scan Mode">
              <NSelect
                v-model:value="unitestModel.scanMode"
                size="large"
                :options="[
                  { label: 'Single File', value: '1' },
                  { label: 'Project Scan', value: '0' },
                ]"
              />
            </NFormItem>
          </NGridItem>
        </NGrid>

        <NDivider>File Paths</NDivider>

        <NSpace vertical size="large">
          <NFormItem path="sourcefilePath" label="Source File Path">
            <div class="flex space-x-2">
              <NInput
                v-model:value="unitestModel.sourcefilePath"
                placeholder="Select source file to test"
                size="large"
                readonly
              />
              <NButton size="large" @click="handleSelectSourceFile">
                <template #icon>
                  <SvgIcon icon="ri:folder-open-line" />
                </template>
                Browse
              </NButton>
            </div>
          </NFormItem>

          <NGrid :cols="2" :x-gap="16">
            <NGridItem>
              <NFormItem path="testfilePath" label="Test Files Directory">
                <div class="flex space-x-2">
                  <NInput
                    v-model:value="unitestModel.testfilePath"
                    placeholder="e.g., tests/, src/__tests__/"
                    size="large"
                  />
                  <NButton size="large" @click="handleSelectDirectory('testfilePath')">
                    <SvgIcon icon="ri:folder-line" />
                  </NButton>
                </div>
              </NFormItem>
            </NGridItem>

            <NGridItem>
              <NFormItem path="testfileOutputPath" label="Test Output Directory">
                <div class="flex space-x-2">
                  <NInput
                    v-model:value="unitestModel.testfileOutputPath"
                    placeholder="Where to save generated tests"
                    size="large"
                  />
                  <NButton size="large" @click="handleSelectDirectory('testfileOutputPath')">
                    <SvgIcon icon="ri:folder-line" />
                  </NButton>
                </div>
              </NFormItem>
            </NGridItem>
          </NGrid>
        </NSpace>

        <!-- Templates Section -->
        <NDivider>Quick Start Templates</NDivider>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div
            v-for="template in templates"
            :key="template.name"
            class="template-card"
            @click="applyTemplate(template)"
          >
            <div class="template-icon">
              <SvgIcon :icon="template.icon" class="text-2xl" />
            </div>
            <h4 class="template-name">
              {{ template.name }}
            </h4>
          </div>
        </div>
      </NCard>

      <!-- Step 1: Test Settings -->
      <NCard v-if="currentStep === 1" title="Test Configuration">
        <template #header-extra>
          <SvgIcon icon="ri:test-tube-line" class="text-green-500" />
        </template>

        <NGrid :cols="2" :x-gap="16" :y-gap="16">
          <NGridItem>
            <NFormItem path="testCommand" label="Test Command">
              <NInput
                v-model:value="unitestModel.testCommand"
                placeholder="e.g., npm test, pytest, mvn test"
                size="large"
              />
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="testCommandDir" label="Test Command Directory">
              <div class="flex space-x-2">
                <NInput
                  v-model:value="unitestModel.testCommandDir"
                  placeholder="Directory to run test command"
                  size="large"
                />
                <NButton size="large" @click="handleSelectDirectory('testCommandDir')">
                  <SvgIcon icon="ri:folder-line" />
                </NButton>
              </div>
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="includedFiles" label="File Patterns">
              <NInput
                v-model:value="unitestModel.includedFiles"
                placeholder="e.g., **/*.{js,ts}, **/*.py"
                size="large"
              />
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="coverageType" label="Coverage Type">
              <NSelect
                v-model:value="unitestModel.coverageType"
                size="large"
                :options="[
                  { label: 'LCOV', value: 'lcov' },
                  { label: 'Coverage.py', value: 'coverage' },
                  { label: 'JaCoCo', value: 'jacoco' },
                  { label: 'Cobertura', value: 'cobertura' },
                ]"
              />
            </NFormItem>
          </NGridItem>
        </NGrid>

        <NDivider>Coverage Settings</NDivider>

        <NGrid :cols="2" :x-gap="16">
          <NGridItem>
            <NFormItem path="codecoveragereportPath" label="Coverage Report Path">
              <div class="flex space-x-2">
                <NInput
                  v-model:value="unitestModel.codecoveragereportPath"
                  placeholder="Path to coverage report"
                  size="large"
                />
                <NButton size="large" @click="handleSelectDirectory('codecoveragereportPath')">
                  <SvgIcon icon="ri:folder-line" />
                </NButton>
              </div>
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="reportFilepath" label="Final Report Path">
              <NInput
                v-model:value="unitestModel.reportFilepath"
                placeholder="Where to save final report"
                size="large"
              />
            </NFormItem>
          </NGridItem>
        </NGrid>
      </NCard>

      <!-- Step 2: Advanced Settings -->
      <NCard v-if="currentStep === 2" title="Advanced Configuration">
        <template #header-extra>
          <SvgIcon icon="ri:settings-4-line" class="text-purple-500" />
        </template>

        <NGrid :cols="2" :x-gap="16" :y-gap="16">
          <NGridItem>
            <NFormItem path="desiredCoverage" label="Target Coverage (%)">
              <NInputNumber
                v-model:value="unitestModel.desiredCoverage"
                :min="0"
                :max="100"
                size="large"
                class="w-full"
              />
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="maxIterations" label="Max AI Iterations">
              <NInputNumber
                v-model:value="unitestModel.maxIterations"
                :min="1"
                :max="10"
                size="large"
                class="w-full"
              />
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="model" label="AI Model">
              <NSelect
                v-model:value="unitestModel.model"
                size="large"
                :options="UnitestStore.allModels().map(v => ({
                  label: v,
                  value: v,
                }))"
              />
            </NFormItem>
          </NGridItem>

          <NGridItem>
            <NFormItem path="isRemote" label="Remote Testing">
              <NSelect
                v-model:value="unitestModel.isRemote"
                size="large"
                :options="[
                  { label: 'Yes', value: 'true' },
                  { label: 'No', value: 'false' },
                ]"
              />
            </NFormItem>
          </NGridItem>
        </NGrid>

        <NFormItem path="additionalInstructions" label="Additional Instructions">
          <NInput
            v-model:value="unitestModel.additionalInstructions"
            type="textarea"
            placeholder="Any specific instructions for AI test generation..."
            :rows="4"
            size="large"
          />
        </NFormItem>
      </NCard>
    </NForm>

    <!-- Navigation -->
    <div class="flex justify-between mt-6">
      <NButton
        v-if="currentStep > 0"
        size="large"
        @click="prevStep"
      >
        <template #icon>
          <SvgIcon icon="ri:arrow-left-line" />
        </template>
        Previous
      </NButton>
      <div />
      <NButton
        v-if="currentStep < 2"
        type="primary"
        size="large"
        @click="nextStep"
      >
        Next
        <template #icon>
          <SvgIcon icon="ri:arrow-right-line" />
        </template>
      </NButton>
      <NButton
        v-if="currentStep === 2"
        type="primary"
        size="large"
        @click="saveUnitestInfo"
      >
        <template #icon>
          <SvgIcon icon="ri:check-line" />
        </template>
        Complete Setup
      </NButton>
    </div>
  </div>
</template>

<style scoped>
.project-config-container {
  @apply p-6 max-w-6xl mx-auto;
}

.config-header {
  @apply bg-gradient-to-r from-blue-50 to-indigo-50 dark:from-gray-800 dark:to-gray-900 rounded-lg p-6 mb-6;
}

.template-card {
  @apply
    p-4
    border
    border-gray-200
    dark:border-gray-700
    rounded-lg
    cursor-pointer
    hover:border-blue-500
    hover:bg-blue-50
    dark:hover:bg-gray-800
    transition-all
    duration-200
    text-center;
}

.template-icon {
  @apply
    w-12
    h-12
    bg-gray-100
    dark:bg-gray-700
    rounded-lg
    flex
    items-center
    justify-center
    mx-auto
    mb-2;
}

.template-name {
  @apply
    text-sm
    font-medium
    text-gray-700
    dark:text-gray-300;
}

.template-card:hover .template-icon {
  @apply bg-blue-100 dark:bg-blue-900;
}
</style>
