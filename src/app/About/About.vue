<script setup>
  import { usePageStore } from "@/store/page";
  import { onMounted, ref } from "vue";

  const pageStore = usePageStore();

  const uid = ref(-1);

  onMounted(async () => {
    // pageStore.setPageLoadingState(true)
    // const _result = await callbackUid(123456)
    // uid.value = _result.data.uid
    // pageStore.setPageLoadingState(false)
  });
</script>

<template>
  <div>about page</div>
</template>

<style scoped lang="scss"></style>
