<script setup lang='ts'>
import { computed, h } from 'vue'
import { useRouter } from 'vue-router'
import { NAvatar, NDropdown, useMessage } from 'naive-ui'
import { useAuthStore, useUserStore } from '@/store'
import { SvgIcon } from '@/components/common'
import defaultAvatar from '@/assets/avatar.jpg'
import { isString } from '@/utils/is'

const router = useRouter()
const message = useMessage()
const userStore = useUserStore()
const authStore = useAuthStore()

const userInfo = computed(() => userStore.userInfo)

const handleLogout = () => {
  // Clear authentication token
  authStore.removeToken()

  // Show success message
  message.success('Logged out successfully')

  // Redirect to login page
  router.push('/login')
}

// Dropdown menu options
const options = [
  {
    label: 'Profile',
    key: 'profile',
    icon: () => h(SvgIcon, { icon: 'ri:user-3-line' }),
  },
  {
    label: 'Settings',
    key: 'settings',
    icon: () => h(SvgIcon, { icon: 'ri:settings-4-line' }),
  },
  {
    type: 'divider',
    key: 'd1',
  },
  {
    label: 'Logout',
    key: 'logout',
    icon: () => h(SvgIcon, { icon: 'ri:logout-circle-line' }),
  },
]

const handleMenuSelect = (key: string) => {
  switch (key) {
    case 'profile':
      message.info('Profile settings coming soon')
      break
    case 'settings':
      message.info('Settings coming soon')
      break
    case 'logout':
      handleLogout()
      break
  }
}
</script>

<template>
  <div class="flex items-center overflow-hidden">
    <NDropdown
      :options="options"
      @select="handleMenuSelect"
    >
      <div class="flex items-center overflow-hidden cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg p-2 transition-colors">
        <div class="w-10 h-10 overflow-hidden rounded-full shrink-0">
          <template v-if="isString(userInfo.avatar) && userInfo.avatar.length > 0">
            <NAvatar
              size="large"
              round
              :src="userInfo.avatar"
              :fallback-src="defaultAvatar"
            />
          </template>
          <template v-else>
            <NAvatar size="large" round :src="defaultAvatar" />
          </template>
        </div>
        <div class="flex-1 min-w-0 ml-2">
          <h2 class="overflow-hidden font-bold text-md text-ellipsis whitespace-nowrap">
            {{ userInfo.name ?? 'User' }}
          </h2>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            Click for options
          </p>
        </div>
        <div class="ml-2 text-gray-400">
          <SvgIcon icon="ri:arrow-down-s-line" class="text-sm" />
        </div>
      </div>
    </NDropdown>
  </div>
</template>
