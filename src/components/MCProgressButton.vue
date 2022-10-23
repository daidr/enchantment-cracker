<script setup>
  const props = defineProps({
    progressMode: {
      type: Boolean,
      default: false,
    },
    progress: {
      type: Number,
      default: 50,
    },
  });
</script>

<template>
  <div
    class="mcbutton"
    :class="{ 'progress-mode': progressMode }"
    ontouchstart=""
  >
    <div class="mcbutton__main">
      <div
        class="progress"
        :style="{
          width: `${progress}%`,
        }"
      ></div>
      <div class="relative">
        <slot />
      </div>
    </div>
    <div class="mcbutton__tl mcbutton__border"></div>
    <div class="mcbutton__t mcbutton__border"></div>
    <div class="mcbutton__tr mcbutton__border"></div>
    <div class="mcbutton__l mcbutton__border"></div>
    <div class="mcbutton__r mcbutton__border"></div>
    <div class="mcbutton__bl mcbutton__border"></div>
    <div class="mcbutton__b mcbutton__border"></div>
    <div class="mcbutton__br mcbutton__border"></div>
  </div>
</template>

<style lang="scss" scoped>
  .mcbutton {
    @apply relative cursor-pointer;

    .progress {
      @apply opacity-0 absolute bg-[#005000] top-0 left-0 bottom-0 z-0;
    }

    &.progress-mode {
      @apply cursor-default pointer-events-none;

      .progress {
        @apply opacity-100;
      }
    }

    .mcbutton__border {
      @apply absolute;
    }

    .mcbutton__tl {
      background-color: #fff;
      @apply top-0 left-0 w-2px h-2px;
    }

    .mcbutton__t {
      background-color: #fff;
      @apply top-0 left-2px right-2px h-2px;
    }

    .mcbutton__tr {
      background-color: #a8a8a8;
      @apply top-0 right-0 w-2px h-2px;
    }

    .mcbutton__l {
      background-color: #fff;
      @apply top-2px left-0 bottom-2px w-2px;
    }

    .mcbutton__r {
      background-color: #595959;
      @apply top-2px right-0 bottom-2px w-2px;
    }

    .mcbutton__bl {
      background-color: #a8a8a8;
      @apply bottom-0 left-0 w-2px h-2px;
    }

    .mcbutton__b {
      background-color: #595959;
      @apply bottom-0 left-2px right-2px h-2px;
    }

    .mcbutton__br {
      background-color: #595959;
      @apply bottom-0 right-0 w-2px h-2px;
    }

    .mcbutton__main {
      @apply select-none text-center;
      @apply m-0px;
      @apply bg-[#8b8b8b] font-bold;
    }

    &:hover {
      @apply opacity-90;
    }

    &:active,
    &.progress-mode {
      @apply opacity-100;

      .mcbutton__tl {
        background-color: #595959;
      }

      .mcbutton__t {
        background-color: #595959;
      }

      .mcbutton__tr {
        background-color: #a8a8a8;
      }

      .mcbutton__l {
        background-color: #595959;
      }

      .mcbutton__r {
        background-color: #fff;
      }

      .mcbutton__bl {
        background-color: #a8a8a8;
      }

      .mcbutton__b {
        background-color: #fff;
      }

      .mcbutton__br {
        background-color: #fff;
      }

      .mcbutton__main {
        @apply bg-[#555555] text-white relative;
      }
    }
  }
</style>
