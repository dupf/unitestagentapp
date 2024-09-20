import { defineStore } from 'pinia'
import type { ProjectStore } from './helper'
import { getLocalProjectList, setLocalProjectList } from './helper'

export const useProjectStore = defineStore('project-store', {
  state: (): ProjectStore => getLocalProjectList(),

  actions: {
    updateProjectList(projectList: []) {
      this.$patch({ projectList })
      setLocalProjectList({ projectList })
    },
    getProjList() {
      return this.$state
    },
  },
})
