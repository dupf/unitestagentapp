import * as _ from 'lodash'
import { ss } from '@/utils/storage'

const LOCAL_NAME = 'UnitestStorage'

export interface UnitestInfo {
  avatar: string
  name: string | null
}

export interface UnitestConfig {
  sourcefilePath: string
  testfilePath: string
  testfileOutputPath: string
  codecoveragereportPath: string | ''
  testCommand: string | ''
  testCommandDir: string | null
  includedFiles: string | null
  coverageType: string | null
  reportFilepath: string | null
  desiredCoverage: number | null
  maxIterations: number | null
  additionalInstructions: string | null
  model: string | null
  isRemote: boolean | null
}

export interface UnitestState {
  UnitestInfo: UnitestInfo
  UnitestConfig: UnitestConfig
}

// 'deepseek-reasoner',

export function defaultSetting(): UnitestState {
  return {
    UnitestInfo: {
      avatar: '',
      name: null,
    },
    UnitestConfig: {
      sourcefilePath: '/Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/twofen.c',
      testfilePath: '/Users/mac/Documents/work/htzr/ZT2_CPU_SW004_V1.0.0.0_T/4_Source/testJZ20_TZB_AllData.c',
      testfileOutputPath: './',
      codecoveragereportPath: '',
      testCommand: './',
      testCommandDir: '',
      includedFiles: '',
      coverageType: '',
      reportFilepath: '',
      desiredCoverage: 90,
      maxIterations: 1,
      additionalInstructions: '',
      model: 'Pro/deepseek-ai/DeepSeek-R1',
      isRemote: true,
    },
  }
}

export function allModels(): string[] {
  return ['gpt-3.5-turbo', 'gpt-3.5-turbo-0301', 'gpt-4', 'gpt-4-0314', 'gpt-4-32k', 'gpt-4-32k-0314', 'deepseek-reasoner']
}

export function getLocalState(): UnitestState {
  const localSetting: UnitestState | undefined = ss.get(LOCAL_NAME)
  const ds = defaultSetting()
  const state = _.merge(ds, localSetting)
  return state
}

export function setLocalState(setting: UnitestState): void {
  ss.set(LOCAL_NAME, setting)
}
