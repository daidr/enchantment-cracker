<script setup>
  import { usePageStore } from "@/store/page";
  import GlobalLoading from "@/components/GlobalLoading.vue";
  import MessageContainer from "@/components/Message/MessageContainer.vue";
  import MainTabs from "@/components/MainTabs.vue";
  import MCFrame from "@/components/MCFrame.vue";
  import { useI18n } from "vue-i18n";
  import Title from "@/components/utils/Title.vue";
  import LangSwitch from "@/components/LangSwitch.vue";
  import { provide, ref } from "vue";
  import { SeedCracker } from "@/utils/SeedCracker";

  const pageStore = usePageStore();

  const { t } = useI18n();

  const cores = ref(
    (navigator.hardwareConcurrency > 12 ? 12 : navigator.hardwareConcurrency) -
      1
  );

  const seedCracker = new SeedCracker({ threadCount: cores.value, i18n: t });

  seedCracker.initCracker();

  provide("seedCracker", seedCracker);
</script>

<template>
  <div class="main-wrapper">
    <Title :title="$route.meta.pageTitle" />
    <div class="toolbar">
      <MCFrame> {{ t("global.core_count") }}{{ cores }} </MCFrame>
      <MCFrame class="lang-switch">
        <LangSwitch />
      </MCFrame>
    </div>

    <MainTabs class="relative z-1" />
    <template v-if="true">
      <router-view v-slot="{ Component }">
        <MCFrame class="main" :class="{ disabled: pageStore.isPageLoading }">
          <p class="select-none font-bold text-16px leading-20px mt-1px">
            {{ t($route.meta.pageTitle || "title.unknown") }}
          </p>
          <KeepAlive>
            <component :is="Component" />
          </KeepAlive>
        </MCFrame>
      </router-view>
    </template>
  </div>
  <GlobalLoading :is-loading="pageStore.isPageLoading" />
  <MessageContainer />
</template>

<style scoped lang="scss">
  .toolbar {
    @apply select-none;
    @apply fixed top-0 right-0 mt-2 mr-2 flex gap-x-2;
    .lang-switch {
      select {
        @apply w-full h-full;
      }
    }
  }

  .main-wrapper {
    @apply w-354px h-390px;

    .main {
      @apply transition w-354px h-338px -mt-10px;

      &.disabled {
        @apply pointer-events-none;
      }

      &:deep(.mcframe__main) {
        @apply flex flex-col;
      }
    }
  }
</style>
