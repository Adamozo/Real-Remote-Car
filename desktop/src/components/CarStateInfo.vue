<template>
  <div class="dashboard-grid">
    <div class="grid-cell value-cell">Aktualny bieg</div>
    <div class="grid-cell value-cell">Położenie gazu</div>
    <div class="grid-cell value-cell">Położenie sprzęgła</div>
    <div class="grid-cell value-cell">Położenie hamulca</div>
    <div class="grid-cell value-cell">Prędkość</div>
    <div class="grid-cell value-cell">{{ carState.gear }}</div>
    <div class="grid-cell value-cell">{{ carState.gasPosition }}%</div>
    <div class="grid-cell value-cell">{{ carState.clutchPosition }}%</div>
    <div class="grid-cell value-cell">{{ carState.brakePosition }}%</div>
    <div class="grid-cell value-cell">{{ carState.speed }} km/h</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface CarState {
  gear: string;
  gasPosition: number;
  clutchPosition: number;
  brakePosition: number;
  speed: number;
}

const carState = ref<CarState>({
  gear: "-",
  gasPosition: 0,
  clutchPosition: 0,
  brakePosition: 0,
  speed: 0,
});

let unlisten: any;

onMounted(async () => {
  await invoke("start_car_state_updates");
  unlisten = await listen("car-state-update", (event: any) => {
    carState.value = event.payload;
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});
</script>
