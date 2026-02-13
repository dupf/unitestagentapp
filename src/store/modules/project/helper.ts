import { ss } from '@/utils/storage'

const LOCAL_NAME = 'projStore'

export type projectList = []

export interface ProjectStore {
  projectList: projectList
}

export function getLocalProjectList(): ProjectStore {
  const projStore: ProjectStore | undefined = ss.get(LOCAL_NAME)
  return projStore ?? { projectList: [] }
}

// export function setLocalPromptList(promptStore: PromptStore): void {
//   ss.set(LOCAL_NAME, promptStore)
// }

export function setLocalProjectList(projectStore: ProjectStore): void {
  ss.set(LOCAL_NAME, projectStore)
}
