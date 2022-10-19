<script setup>
  import { usePageStore } from "@/store/page";
  import { onMounted, ref } from "vue";
  import { useI18n } from "vue-i18n";
  import MCButton from "../../components/MCButton.vue";

  const { t } = useI18n();

  const pageStore = usePageStore();
</script>

<template>
  <div class="wrapper">
    <div class="xpseed_wrapper xpseed_wrapper_1">
      <span>{{ t("enchCrack.xpSeed1") }}</span>
      <input
        type="text"
        maxlength="8"
        :title="t('enchCrack.xpSeed1_tooltip')"
      />
    </div>
    <div class="xpseed_wrapper xpseed_wrapper_2">
      <span>{{ t("enchCrack.xpSeed2") }}</span>
      <input
        type="text"
        maxlength="8"
        :title="t('enchCrack.xpSeed2_tooltip')"
      />
    </div>
    <MCButton class="calc-btn">{{ t("enchCrack.calculate") }}</MCButton>
    <div class="totalseed_wrapper">
      <input type="text" maxlength="12" />
    </div>
    <div class="bookshelves_wrapper">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.bookshelves_tooltip')"
      />
    </div>
    <div class="level_wrapper level_wrapper_1">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.xpCost1_tooltip')"
      />
    </div>
    <div class="level_wrapper level_wrapper_2">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.xpCost2_tooltip')"
      />
    </div>
    <div class="level_wrapper level_wrapper_3">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.xpCost3_tooltip')"
      />
    </div>
    <MCButton class="check-btn" :title="t('enchCrack.check_tooltip')">{{
      t("enchCrack.check")
    }}</MCButton>
    <MCButton class="reset-btn" :title="t('enchCrack.reset_tooltip')">{{
      t("enchCrack.reset")
    }}</MCButton>
  </div>
</template>

<style scoped lang="scss">
  .wrapper {
    @apply w-full h-full relative;
    background-image: url(@/assets/images/pane1.png);

    .xpseed_wrapper {
      @apply font-bold text-15px absolute;
      span {
        @apply mb-0.5 block;
        @apply select-none;
      }

      input {
        @apply w-110px;
      }

      &.xpseed_wrapper_1 {
        @apply top-3px left-0;
      }
      &.xpseed_wrapper_2 {
        @apply top-42px left-0;
      }
    }

    .calc-btn {
      @apply absolute top-82px w-150px;
    }

    .totalseed_wrapper {
      @apply absolute top-109px left-0;
      input {
        @apply w-150px;
      }
    }

    .bookshelves_wrapper {
      @apply absolute top-45px right-80px;
      input {
        @apply w-35px text-center;
      }
    }

    .level_wrapper {
      @apply absolute;
      input {
        @apply w-30px text-center;
      }

      &.level_wrapper_1 {
        @apply bottom-126px right-18px;
      }
      &.level_wrapper_2 {
        @apply bottom-88px right-18px;
      }
      &.level_wrapper_3 {
        @apply bottom-50px right-18px;
      }
    }

    .check-btn {
      @apply absolute bottom-0 left-0 w-250px;
    }

    .reset-btn {
      @apply absolute bottom-0 right-0 w-80px;
    }
  }
</style>
