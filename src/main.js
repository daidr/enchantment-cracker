import { createApp } from 'vue'
import App from '@/App.vue'
import 'virtual:windi.css'
import 'virtual:windi-devtools'
// 引入全局样式
import './assets/css/global.css'

const app = createApp(App);

import pinia from '@/store/index'
app.use(pinia);

import router from '@/routes/router';
app.use(router);


// i18n
import { createI18n } from 'vue-i18n'
import messages from '@intlify/vite-plugin-vue-i18n/messages';

// 尝试从 localstorage或navigator.language中获取语言
const lang = localStorage.getItem('lang') || navigator.language.toLowerCase();

const i18n = createI18n({
    locale: lang,
    fallbackLocale: 'en',
    messages,
});

app.use(i18n);

app.mount('#app');
