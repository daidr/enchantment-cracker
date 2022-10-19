<script setup>
  import { computed } from "vue";
  import { useRoute } from "vue-router";

  const route = useRoute();
  const routeList = ["cracker", "calc", "about"];

  const currentRouteIndex = computed(() => {
    return routeList.indexOf(route.name) + 1 || 1;
  });
</script>

<template>
  <div :class="`tabs tabs-active-${currentRouteIndex}`">
    <RouterLink to="/" class="tabs__tab_1 tabs__tab">
      <div class="icon icon-1"></div>
      <div class="icon icon-2"></div>
    </RouterLink>
    <RouterLink to="/calc" class="tabs__tab_2 tabs__tab">
      <div class="icon icon-1"></div>
      <div class="icon icon-2"></div>
    </RouterLink>
    <RouterLink to="/about" class="tabs__tab_3 tabs__tab">
      <div class="icon icon-1"></div>
      <div class="icon icon-2"></div>
    </RouterLink>
  </div>
</template>

<style scoped lang="scss">
  .tabs {
    @apply w-164px h-62px;
    @apply flex;
    background-image: url(@/assets/images/tabs.png);

    .tabs__tab {
      @apply w-54px h-56px cursor-pointer;
      @apply relative;

      .icon {
        @apply w-32px h-32px bg-no-repeat bg-cover image-render-pixel;
        @apply absolute top-1/2 left-1/2 transform-gpu -translate-x-1/2 -translate-y-1/2;
        @apply transition;
      }

      .icon-2 {
        @apply opacity-0;
      }

      &.router-link-exact-active {
        .icon-2 {
          @apply opacity-100;
        }
      }
    }

    .tabs__tab_2 {
      .icon-1 {
        background-image: url(@/assets/images/book.png);
      }
      .icon-2 {
        background-image: url(@/assets/images/enchanted_book.png);
      }
    }
    .tabs__tab_3 {
      .icon-1 {
        background-image: url(@/assets/images/apple.png);
      }
      .icon-2 {
        background-image: url(@/assets/images/golden_apple.png);
      }
    }
    .tabs__tab_1 {
      .icon-1 {
        background-image: url(@/assets/images/glass_bottle.png);
      }
      .icon-2 {
        background-image: url(@/assets/images/experience_bottle.png);
      }
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
</style>
