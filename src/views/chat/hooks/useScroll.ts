import type { Ref } from 'vue'
import { nextTick, ref } from 'vue'

// 修改 ScrollElement 的类型定义
type ScrollElement = HTMLDivElement

interface ScrollReturn {
  scrollRef: Ref<ScrollElement | null>
  scrollToBottom: () => Promise<void>
  scrollToTop: () => Promise<void>
  scrollToBottomIfAtBottom: () => Promise<void>
}

export function useScroll(): ScrollReturn {
  const scrollRef = ref<ScrollElement | null>(null)

  const scrollToBottom = async () => {
    await nextTick()
    if (scrollRef.value)
      scrollRef.value.scrollTop = scrollRef.value.scrollHeight
  }

  const scrollToTop = async () => {
    await nextTick()
    if (scrollRef.value)
      scrollRef.value.scrollTop = 0
  }

  const scrollToBottomIfAtBottom = async () => {
    await nextTick()
    if (scrollRef.value) {
      const threshold = 50 // 阈值，表示滚动条到底部的距离阈值
      const distanceToBottom = scrollRef.value.scrollHeight - scrollRef.value.scrollTop - scrollRef.value.clientHeight
      if (distanceToBottom <= threshold)
        scrollRef.value.scrollTop = scrollRef.value.scrollHeight
    }
  }
  return {
    scrollRef: scrollRef as Ref<HTMLDivElement | null>,
    scrollToBottom,
    scrollToTop,
    scrollToBottomIfAtBottom,
  }
}
