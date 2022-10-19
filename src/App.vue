<script setup>
import { usePageStore } from '@/store/page';
import GlobalLoading from '@/components/GlobalLoading.vue';
import MessageContainer from '@/components/Message/MessageContainer.vue';

const pageStore = usePageStore()
</script>

<template>
  <div class="main-wrapper">
    <div class="tabs tabs-active-1">
      <div class="tabs__tab_1 tabs__tab"></div>
      <div class="tabs__tab_2 tabs__tab"></div>
      <div class="tabs__tab_3 tabs__tab"></div>
    </div>
    <template v-if="true">
      <router-view v-slot="{ Component }">
        <div class="main" :class="{disabled: pageStore.isPageLoading}">
          <component :is="Component" />
        </div>
      </router-view>
    </template>
  </div>
  <GlobalLoading :is-loading="pageStore.isPageLoading" />
  <MessageContainer />
</template>

<style scoped lang="scss">
.main-wrapper {
  @apply w-354px h-390px;

  .tabs {
    @apply w-164px h-62px;
    background-image: url(@/assets/images/tabs.png);

    .tabs__tab {
      @apply w-54px h-62px inline-block cursor-pointer;
    }

    &.tabs-active-1 {
      background-position: 0 0;

      .tabs__tab_1 {
        @apply pointer-events-none;
      }
    }

    &.tabs-active-2 {
      background-position: 0 -62px;

      .tabs__tab_2 {
        @apply pointer-events-none;
      }
    }

    &.tabs-active-3 {
      background-position: 0 -124px;

      .tabs__tab_3 {
        @apply pointer-events-none;
      }
    }
  }

  .main {
    @apply transition h-full;

    &.disabled {
      @apply pointer-events-none opacity-0;
    }
  }
}
</style>
