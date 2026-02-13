<script setup lang='ts'>
import { defineAsyncComponent, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useMessage } from 'naive-ui'
import { HoverButton, SvgIcon } from '@/components/common'
import { useAuthStore } from '@/store'

const Setting = defineAsyncComponent(() => import('@/components/common/Setting/index.vue'))

const router = useRouter()
const message = useMessage()
const authStore = useAuthStore()
const show = ref(false)

const handleLogout = () => {
  // Clear authentication token
  authStore.removeToken()

  // Show success message
  message.success('Logged out successfully')

  // Redirect to login page
  router.push('/login')
}
</script>

<template>
  <footer class="flex items-center justify-between min-w-0 p-4 overflow-hidden border-t dark:border-neutral-800">
    <div class="flex-1 flex-shrink-0 overflow-hidden">
      <UserAvatar />
    </div>
    <div class="flex items-center space-x-2">
      <HoverButton tooltip="Settings" @click="show = true">
        <span class="text-xl text-[#4f555e] dark:text-white">
          <SvgIcon icon="ri:settings-4-line" />
        </span>
      </HoverButton>
      <HoverButton tooltip="Logout" @click="handleLogout">
        <span class="text-xl text-[#4f555e] dark:text-white">
          <SvgIcon icon="ri:logout-circle-line" />
        </span>
      </HoverButton>
    </div>
    <Setting v-if="show" v-model:visible="show" />
  </footer>
</template>
