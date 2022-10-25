<script setup>
  import { usePageStore } from "@/store/page";
  import GlobalLoading from "@/components/GlobalLoading.vue";
  import MessageContainer from "@/components/Message/MessageContainer.vue";
  import MainTabs from "@/components/MainTabs.vue";
  import MCFrame from "@/components/MCFrame.vue";
  import { useI18n } from "vue-i18n";
  import Title from "@/components/utils/Title.vue";
  import LangSwitch from "@/components/LangSwitch.vue";
  import { onMounted, provide, ref } from "vue";
  import RNGWorker from "@/worker/rng-worker?worker";
  import init, { get_last_few_seeds } from "@rsw/rng-worker";
  import { useMessage } from "./components/Message";

  const pageStore = usePageStore();

  const { info } = useMessage();

  const { t } = useI18n();

  const cores = ref(
    (navigator.hardwareConcurrency > 12 ? 12 : navigator.hardwareConcurrency) -
      1
  );

  const workerPool = [];

  // init worker pool
  const initWorkerPool = () => {
    // remove old workers
    workerPool.forEach((worker) => {
      worker.terminate();
    });
    workerPool.length = 0;

    // init worker
    for (let i = 0; i < cores.value; i++) {
      const worker = new RNGWorker();
      worker.addEventListener("message", (e) => {
        workerHandler(e.data);
      });
      worker.postMessage({
        type: "initEnv",
        payload: { coreCount: cores.value, coreIndex: i },
      });
      workerPool.push(worker);
    }
  };

  initWorkerPool();

  // abort requested
  const abortRequestedSharedBuf = new SharedArrayBuffer(1);

  const timer = -1;
  let isProgressing = false;

  const firstInput = (bookshelves, slot1, slot2, slot3, progressHandler) => {
    // seed
    const seedSharedBuf = new SharedArrayBuffer(4);
    console.log("seedSharedBuf", seedSharedBuf);
    console.log("abortRequestedSharedBuf", abortRequestedSharedBuf);

    Atomics.store(new Int32Array(seedSharedBuf), 0, -2147483648);
    Atomics.store(new Int8Array(abortRequestedSharedBuf), 0, 0);

    clearInterval(timer);
    isProgressing = true;
    const seedSharedDataView = new Int32Array(seedSharedBuf);
    setInterval(() => {
      if (isProgressing) {
        progressHandler(
          ((Atomics.load(seedSharedDataView, 0) + 2147483648) / 4294967296) *
            100,
          true
        );
      } else {
        // progressHandler((seedSearchedDataView[0] / 4294967296n) * 100n,true);
        clearInterval(timer);
      }
    }, 200);

    // dispatch event to workers
    workerPool.forEach((worker) => {
      worker.postMessage({
        type: "firstInput",
        payload: {
          bookshelves,
          slot1,
          slot2,
          slot3,
          seedSharedBuf,
          abortRequestedSharedBuf,
        },
      });
    });
  };

  const abort = () => {
    // TODO
    Atomics.store(new Int8Array(abortRequestedSharedBuf), 0, 1);
  };

  provide("firstInput", firstInput);
  provide("abort", abort);

  let core_FirstInputDone = 0;
  let count_FirstInputDone = 0;

  const firstInputDoneHandler = ({
    count,
    inputData: { bookshelves, slot1, slot2, slot3, seedSharedBuf },
  }) => {
    core_FirstInputDone++;
    count_FirstInputDone += count;
    if (core_FirstInputDone === cores.value) {
      isProgressing = false;
      console.log("count", count_FirstInputDone);
      {
        // find seed not be validated
        const seedSharedDataView = new Int32Array(seedSharedBuf);
        const seed = Atomics.load(seedSharedDataView, 0);
        const lastFew = get_last_few_seeds(
          seed,
          bookshelves,
          slot1,
          slot2,
          slot3
        );
        console.log("totalCount", count_FirstInputDone + lastFew.length);
        info(t("rngCracker.message.finished"));
      }

      core_FirstInputDone = 0;
      count_FirstInputDone = 0;
    }
  };

  let core_FirstInputAbort = 0;
  const firstInputAbortHandler = () => {
    core_FirstInputAbort++;
    console.log(core_FirstInputAbort);
    if (core_FirstInputAbort === cores.value) {
      isProgressing = false;
      core_FirstInputAbort = 0;
      info(t("rngCracker.message.abort"));

      // 顺便终止worker并重新创建
      initWorkerPool();
    }
  };

  const workerHandler = ({ type, payload }) => {
    switch (type) {
      case "firstInputDone":
        firstInputDoneHandler(payload);
        break;
      case "firstInputAbort":
        firstInputAbortHandler(payload);
        break;
    }
  };

  onMounted(() => {
    init();
  });
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
