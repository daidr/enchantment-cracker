<script setup>
  import { createAlertModal } from "@/components/Modal";
  import { usePageStore } from "@/store/page";
  import { inject, onMounted, ref } from "vue";
  import { useI18n } from "vue-i18n";
  import MCButton from "../../components/MCButton.vue";
  import MCProgressButton from "@/components/MCProgressButton.vue";

  const { t } = useI18n();

  const pageStore = usePageStore();

  const firstInput = inject("firstInput");
  const abort = inject("abort");

  const bookshelves = ref("");
  const xpcost1 = ref("");
  const xpcost2 = ref("");
  const xpcost3 = ref("");

  const onCheckBtnClick = () => {
    const bookshelvesValue = parseInt(bookshelves.value);
    const xpcost1Value = parseInt(xpcost1.value);
    const xpcost2Value = parseInt(xpcost2.value);
    const xpcost3Value = parseInt(xpcost3.value);

    // 检查是否能够转换为数字
    if (
      isNaN(bookshelvesValue) ||
      isNaN(xpcost1Value) ||
      isNaN(xpcost2Value) ||
      isNaN(xpcost3Value)
    ) {
      createAlertModal({
        title: t("enchCrack.alert.title"),
        content: t("enchCrack.alert.content_1"),
        hideOk: true,
        cancelText: t("enchCrack.alert.closeText"),
      });
      return;
    }

    // 检查书架数量是否在0~15之间
    if (bookshelvesValue < 0 || bookshelvesValue > 15) {
      createAlertModal({
        title: t("enchCrack.alert.title"),
        content: t("enchCrack.alert.content_2"),
        hideOk: true,
        cancelText: t("enchCrack.alert.closeText"),
      });
      return;
    }

    // 检查第一附魔等级是否在0~30之间
    if (xpcost1Value < 0 || xpcost1Value > 30) {
      createAlertModal({
        title: t("enchCrack.alert.title"),
        content: t("enchCrack.alert.content_3", [1]),
        hideOk: true,
        cancelText: t("enchCrack.alert.closeText"),
      });
      return;
    }

    // 检查第二附魔等级是否在0~30之间
    if (xpcost2Value < 0 || xpcost2Value > 30) {
      createAlertModal({
        title: t("enchCrack.alert.title"),
        content: t("enchCrack.alert.content_3", [2]),
        hideOk: true,
        cancelText: t("enchCrack.alert.closeText"),
      });
      return;
    }

    // 检查第三附魔等级是否在0~30之间
    if (xpcost3Value < 0 || xpcost3Value > 30) {
      createAlertModal({
        title: t("enchCrack.alert.title"),
        content: t("enchCrack.alert.content_3", [3]),
        hideOk: true,
        cancelText: t("enchCrack.alert.closeText"),
      });
      return;
    }

    firstInput(
      bookshelves.value,
      xpcost1.value,
      xpcost2.value,
      xpcost3.value,
      firstInputProgressHandler
    );
  };

  const onAbortBtnClick = () => {
    abort();
  };

  const calcProgress = ref(-1);

  const firstInputProgressHandler = (progress, isProgressing) => {
    if (isProgressing) {
      calcProgress.value = progress;
    } else {
      calcProgress.value = -1;
    }
  };
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
        v-model="bookshelves"
      />
    </div>
    <div class="level_wrapper level_wrapper_1">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.xpCost1_tooltip')"
        v-model="xpcost1"
      />
    </div>
    <div class="level_wrapper level_wrapper_2">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.xpCost2_tooltip')"
        v-model="xpcost2"
      />
    </div>
    <div class="level_wrapper level_wrapper_3">
      <input
        type="text"
        maxlength="2"
        :title="t('enchCrack.xpCost3_tooltip')"
        v-model="xpcost3"
      />
    </div>
    <MCProgressButton
      class="check-btn"
      :title="t('enchCrack.check_tooltip')"
      :progress="calcProgress"
      :progress-mode="calcProgress != -1"
      @click="onCheckBtnClick"
    >
      <template v-if="calcProgress == -1">
        {{ t("enchCrack.check") }}
      </template>
      <template v-else>
        {{ t("enchCrack.progress", [calcProgress.toFixed(2)]) }}
      </template>
    </MCProgressButton>
    <MCButton
      class="reset-btn"
      :title="t('enchCrack.reset_tooltip')"
      @click="onAbortBtnClick"
      >{{ t("enchCrack.reset") }}</MCButton
    >
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
      @apply absolute top-82px w-153px;
    }

    .totalseed_wrapper {
      @apply absolute top-109px left-0;
      input {
        @apply w-153px;
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
