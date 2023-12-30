// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  vite: {
    css: {
      preprocessorOptions: {
        sass: {
          additionalData: '@import "@/public/global.sass"',
        },
      },
    },
  },
  runtimeConfig: {
    public: {
      apiAddress: "127.0.0.1",
    }
  }
})
