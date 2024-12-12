<template>
  <div class="container">
    <div v-if="!showDashboard" class="welcome-screen">
      <button @click="showDashboard = true" class="start-button">
        Start Car Dashboard
      </button>
    </div>
    <div v-else class="dashboard-container">
      <div class="dashboard-content">
        <Navbar @back="showDashboard = false" />
        <VideoStream :stream-url="streamUrl" :muted="true" class="video-feed" />
        <div class="car-info-wrapper">
          <CarStateInfo />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import CarStateInfo from "./components/CarStateInfo.vue";
import VideoStream from "./components/VideoStream.vue";
import Navbar from "./components/Navbar.vue";

const showDashboard = ref(false);
const streamUrl = ref(
  "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4"
);
</script>

<!--  -->
