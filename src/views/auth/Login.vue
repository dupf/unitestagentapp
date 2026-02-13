<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { type FormInst, NButton, NCheckbox, NForm, NFormItem, NInput, useMessage } from 'naive-ui'
import { SvgIcon } from '@/components/common'
import { useAuthStore } from '@/store'
import { fetchLogin } from '@/api'

const router = useRouter()
const message = useMessage()
const authStore = useAuthStore()

const formRef = ref<FormInst>()
const loading = ref(false)

const loginForm = ref({
  username: '',
  password: '',
  rememberMe: false,
})

const rules = {
  username: {
    required: true,
    message: 'Please enter username',
    trigger: ['input', 'blur'],
  },
  password: {
    required: true,
    message: 'Please enter password',
    trigger: ['input', 'blur'],
  },
}

const isFormValid = computed(() => {
  return loginForm.value.username.trim() !== '' && loginForm.value.password.trim() !== ''
})

const handleLogin = async () => {
  if (!formRef.value)
    return

  try {
    await formRef.value.validate()
    loading.value = true

    // Demo mode: If username/password is admin/admin, login directly
    if (loginForm.value.username === 'admin' && loginForm.value.password === 'admin') {
      const mockToken = `demo-token-${Date.now()}`
      authStore.setToken(mockToken)
      message.success('Login successful! (Demo mode)')

      // Navigate to main page
      router.push('/')
      return
    }

    // Try real API login
    try {
      const { data } = await fetchLogin<{ token?: string; message?: string }>({
        username: loginForm.value.username,
        password: loginForm.value.password,
        rememberMe: loginForm.value.rememberMe,
      })

      if (data?.token) {
        authStore.setToken(data.token)
        message.success('Login successful!')

        // Get user session info
        await authStore.getSession()

        // Navigate to main page
        router.push('/')
      }
      else {
        message.error('Login failed, please check username and password')
      }
    }
    catch (apiError) {
      // Handle API call failure
      console.error('API login failed, suggest using demo account:', apiError)
      message.error('Server connection failed, please use demo account: admin/admin')
    }
  }
  catch (error: any) {
    console.error('Login error:', error)
    message.error('An error occurred during login, please try again later')
  }
  finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-8">
        <div class="text-center">
          <div class="flex justify-center mb-4">
            <div class="w-16 h-16 bg-[#4b9e5f] rounded-full flex items-center justify-center">
              <SvgIcon icon="ri:shield-user-line" class="text-3xl text-white" />
            </div>
          </div>
          <h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
            Sign In
          </h2>
          <p class="text-gray-600 dark:text-gray-400">
            Please enter your credentials
          </p>
        </div>

        <!-- Demo Account Info -->
        <div class="mt-6 mb-6 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
          <div class="text-center">
            <p class="text-sm text-blue-600 dark:text-blue-400 font-medium">
              💡 Demo Account
            </p>
            <p class="text-sm text-blue-500 dark:text-blue-300 mt-1">
              Username: <span class="font-mono font-bold">admin</span><br>
              Password: <span class="font-mono font-bold">admin</span>
            </p>
          </div>
        </div>

        <NForm
          ref="formRef"
          :model="loginForm"
          :rules="rules"
          class="space-y-6"
          @keyup.enter="handleLogin"
        >
          <div class="space-y-4">
            <NFormItem path="username" :show-label="false">
              <NInput
                v-model:value="loginForm.username"
                size="large"
                placeholder="Username"
                :input-props="{ autocomplete: 'username' }"
                class="w-full"
              >
                <template #prefix>
                  <SvgIcon icon="ri:user-3-line" class="text-gray-400" />
                </template>
              </NInput>
            </NFormItem>

            <NFormItem path="password" :show-label="false">
              <NInput
                v-model:value="loginForm.password"
                type="password"
                size="large"
                placeholder="Password"
                show-password-on="click"
                :input-props="{ autocomplete: 'current-password' }"
                class="w-full"
              >
                <template #prefix>
                  <SvgIcon icon="ri:lock-2-line" class="text-gray-400" />
                </template>
              </NInput>
            </NFormItem>
          </div>

          <div class="flex items-center justify-between">
            <NCheckbox v-model:checked="loginForm.rememberMe">
              Remember me
            </NCheckbox>
            <NButton text type="primary" size="small">
              Forgot password?
            </NButton>
          </div>

          <div>
            <NButton
              type="primary"
              size="large"
              :loading="loading"
              :disabled="!isFormValid"
              block
              @click="handleLogin"
            >
              <template #icon>
                <SvgIcon icon="ri:login-circle-line" />
              </template>
              Sign In
            </NButton>
          </div>
        </NForm>

        <div class="mt-6 text-center">
          <p class="text-sm text-gray-600 dark:text-gray-400">
            Don't have an account?
            <NButton text type="primary" size="small">
              Sign up now
            </NButton>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.min-h-screen {
  min-height: 100vh;
}
</style>
