import { ss } from '@/utils/storage'

const LOCAL_NAME = 'unitestagentStorage'

export function defaultState(): Unitest.UnitestState {
  const uuid = 1002
  return {
    active: uuid,
    history: [{ uuid, title: 'New Unitestagent', isEdit: false }],
    unitest: [{ uuid, data: [], opt: {} }],
  }
}

export function getLocalState(): Unitest.UnitestState {
  const localState = ss.get(LOCAL_NAME)
  return localState ?? defaultState()
}

export function setLocalState(state: Unitest.UnitestState) {
  ss.set(LOCAL_NAME, state)
}
