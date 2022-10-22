import path from 'path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import WindiCSS from 'vite-plugin-windicss'
import VueI18n from '@intlify/vite-plugin-vue-i18n'
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig({
  plugins: [
    vue(),
    WindiCSS(),
    VueI18n({
      include: [path.resolve(__dirname, './locale/**')],
    }),
    ViteRsw(),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src')
    }
  }
})