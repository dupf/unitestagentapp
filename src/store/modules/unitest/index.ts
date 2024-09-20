import { defineStore } from 'pinia'
import type { UnitestInfo, UnitestState } from './helper'
import { allModels, defaultSetting, getLocalState, setLocalState } from './helper'

export const useUnitestStore = defineStore('Unitest-store', {
  state: (): UnitestState => getLocalState(),
  actions: {
    updateUnitestInfo(UnitestInfo: Partial<UnitestInfo>) {
      this.UnitestInfo = { ...this.UnitestInfo, ...UnitestInfo }
      this.recordState()
    },

    resetUnitestInfo() {
      this.UnitestInfo = { ...defaultSetting().UnitestInfo }
      this.recordState()
    },

    recordState() {
      // alert('recordState', this.$state)
      setLocalState(this.$state)
    },

    allModels(): string[] {
      return allModels()
    },
  },
})
