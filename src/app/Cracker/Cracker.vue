<script setup>
  import { usePageStore } from "@/store/page";
  import { onMounted, ref } from "vue";

  const pageStore = usePageStore();
</script>

<template>
  <div class="wrapper"></div>
</template>

<style scoped lang="scss">
  .wrapper {
    @apply w-full h-full;
    background-image: url(@/assets/images/pane1.png);
  }
</style>
