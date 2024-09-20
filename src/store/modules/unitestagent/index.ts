import { defineStore } from 'pinia'
import { getLocalState, setLocalState } from './helper'
import { router } from '@/router'
export enum UnitestagentRule {
  User = 'user',
  System = 'system',
  Assistant = 'assistant',
}
export const useUnitestagentStore = defineStore('unitestagent-store', {
  // state: (): Unitestagent.UnitestagentState => getLocalState(),

  state: (): Unitest.UnitestState => getLocalState(),

  getters: {
    getUnitestagentHistoryByCurrentActive(state: Unitest.UnitestState,
      // Unitestagent.UnitestagentState
    ) {
      const index = state.history.findIndex(item => item.uuid === state.active)
      if (index !== -1)
        return state.history[index]
      return null
    },

    getUnitestagentSessionByUuid(state: Unitest.UnitestState,
      // Unitestagent.UnitestagentState

    ) {
      return (uuid?: number) => {
        if (uuid)
          return state.unitest.find(item => item.uuid === uuid)
        return state.unitest.find(item => item.uuid === state.active)
      }
    },
    getUnitestagentDataByUuid(state: Unitest.UnitestState) {
      return (uuid?: number) => {
        if (uuid)
          return state.unitest.find(item => item.uuid === uuid)?.data ?? []
        return state.unitest.find(item => item.uuid === state.active)?.data ?? []
      }
    },
  },

  actions: {
    addHistory(history: Unitest.History, unitestagentData: Unitest.Unitest[] = []) {
      this.history.unshift(history)
      this.unitest.unshift({ uuid: history.uuid, data: unitestagentData, opt: {} })

      this.active = history.uuid
      this.reloadRoute(history.uuid)
    },

    updateHistory(uuid: number, edit: Partial<Unitest.History>) {
      const index = this.history.findIndex(item => item.uuid === uuid)
      if (index !== -1) {
        this.history[index] = { ...this.history[index], ...edit }
        this.recordState()
      }
    },

    async deleteHistory(index: number) {
      this.history.splice(index, 1)
      // this.unitestagent.splice(index, 1)
      this.unitest.splice(index, 1)

      if (this.history.length === 0) {
        this.active = null
        this.reloadRoute()
        return
      }

      if (index > 0 && index <= this.history.length) {
        const uuid = this.history[index - 1].uuid
        this.active = uuid
        this.reloadRoute(uuid)
        return
      }

      if (index === 0) {
        if (this.history.length > 0) {
          const uuid = this.history[0].uuid
          this.active = uuid
          this.reloadRoute(uuid)
        }
      }

      if (index > this.history.length) {
        const uuid = this.history[this.history.length - 1].uuid
        this.active = uuid
        this.reloadRoute(uuid)
      }
    },

    async setActive(uuid: number) {
      this.active = uuid
      return await this.reloadRoute(uuid)
    },

    getUnitestagentByUuidAndIndex(uuid: number, index: number) {
      if (!uuid || uuid === 0) {
        if (this.unitest.length)
          return this.unitest[0].data[index]
        return null
      }
      const unitestagentIndex = this.unitest.findIndex(item => item.uuid === uuid)
      if (unitestagentIndex !== -1)
        return this.unitest[unitestagentIndex].data[index]
      return null
    },

    addUnitestagentByUuid(uuid: number, unitestagent: Unitest.Unitest,
      // Unitestagent.Unitestagent
    ) {
      if (!uuid || uuid === 0) {
        if (this.history.length === 0) {
          const uuid = Date.now()
          this.history.push({ uuid, title: unitestagent.unitestMessage.testfilePath, isEdit: false })
          this.unitest.push({ uuid, data: [unitestagent], opt: {} })
          this.active = uuid
          this.recordState()
        }
        else {
          this.unitest[0].data.push(unitestagent)
          if (this.history[0].title === 'New Unitestagent')
            this.history[0].title = unitestagent.unitestMessage.testfilePath
          this.recordState()
        }
      }

      const index = this.unitest.findIndex(item => item.uuid === uuid)
      if (index !== -1) {
        this.unitest[index].data.push(unitestagent)
        if (this.history[index].title === 'New Unitestagent')
          this.history[index].title = unitestagent.unitestMessage.testfilePath
        this.recordState()
      }
    },

    updateUnitestagentByUuid(uuid: number, index: number, unitestagent: Unitest.Unitest) {
      if (!uuid || uuid === 0) {
        if (this.unitest.length) {
          this.unitest[0].data[index] = unitestagent
          this.recordState()
        }
        return
      }

      const unitestagentIndex = this.unitest.findIndex(item => item.uuid === uuid)
      if (unitestagentIndex !== -1) {
        this.unitest[unitestagentIndex].data[index] = unitestagent
        this.recordState()
      }
    },

    updateUnitestagentSomeByUuid(uuid: number, index: number, unitestagent: Partial<Unitest.Unitest>) {
      if (!uuid || uuid === 0) {
        if (this.unitest.length) {
          this.unitest[0].data[index] = { ...this.unitest[0].data[index], ...unitestagent }
          this.recordState()
        }
        return
      }

      const unitestagentIndex = this.unitest.findIndex(item => item.uuid === uuid)
      if (unitestagentIndex !== -1) {
        this.unitest[unitestagentIndex].data[index] = { ...this.unitest[unitestagentIndex].data[index], ...unitestagent }
        this.recordState()
      }
    },

    deleteUnitestagentByUuid(uuid: number, index: number) {
      if (!uuid || uuid === 0) {
        if (this.unitest.length) {
          this.unitest[0].data.splice(index, 1)
          this.recordState()
        }
        return
      }

      const unitestagentIndex = this.unitest.findIndex(item => item.uuid === uuid)
      if (unitestagentIndex !== -1) {
        this.unitest[unitestagentIndex].data.splice(index, 1)
        this.recordState()
      }
    },

    clearUnitestagentByUuid(uuid: number) {
      if (!uuid || uuid === 0) {
        if (this.unitest.length) {
          this.unitest[0].data = []
          this.recordState()
        }
        return
      }

      const index = this.unitest.findIndex(item => item.uuid === uuid)
      if (index !== -1) {
        this.unitest[index].data = []
        this.recordState()
      }
    },

    async reloadRoute(uuid?: number) {
      this.recordState()
      await router.push({ name: 'Unitestagent', params: { uuid } })
    },

    recordState() {
      setLocalState(this.$state)
    },
  },
})
