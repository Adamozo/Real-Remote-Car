<template>
  <div class="video-container">
    <!-- <video
      ref="videoRef"
      class="video-stream"
      :autoplay="true"
      :muted="muted"
      :controls="showControls"
    >
      <source :src="streamUrl" type="video/mp4" />
      Your browser does not support the video element.
    </video>

    <div class="stream-status" :class="{ 'status-error': hasError }">
      {{ statusMessage }}
    </div> -->
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";

interface Props {
  streamUrl: string;
  muted?: boolean;
  showControls?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  muted: true,
  showControls: false,
});

const videoRef = ref<HTMLVideoElement | null>(null);
const hasError = ref(false);
const statusMessage = ref("Connecting...");

const handleVideoError = (error: Event) => {
  console.error("Video stream error:", error);
  hasError.value = true;
  statusMessage.value = "Failed to load video stream";
};

const handleVideoPlaying = () => {
  hasError.value = false;
  statusMessage.value = "Stream connected";
};

onMounted(() => {
  if (videoRef.value) {
    videoRef.value.addEventListener("error", handleVideoError);
    videoRef.value.addEventListener("playing", handleVideoPlaying);
  }
});

onUnmounted(() => {
  if (videoRef.value) {
    videoRef.value.removeEventListener("error", handleVideoError);
    videoRef.value.removeEventListener("playing", handleVideoPlaying);
  }
});

watch(
  () => props.streamUrl,
  () => {
    if (videoRef.value) {
      videoRef.value.load();
    }
  }
);
</script>
