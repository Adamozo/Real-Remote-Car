<template>
  <div class="dashboard-grid">
    <div class="grid-cell value-cell">
      Aktualny bieg: {{ formatGear(carState.current_gear) }}
    </div>
    <div class="grid-cell value-cell">
      Bieg bezpośredni: {{ formatGear(carState.dirrect_gear) }}
    </div>
    <div class="grid-cell value-cell">
      Położenie gazu: {{ carState.pgas.GAS }}%
    </div>
    <div class="grid-cell value-cell">
      Położenie sprzęgła: {{ carState.pclutch.CLUTCH }}%
    </div>
    <div class="grid-cell value-cell">
      Położenie hamulca: {{ carState.pbreak.BREAK }}%
    </div>

    <div class="grid-cell value-cell"></div>
    <div class="grid-cell value-cell">
      Kąt kierownicy: {{ carState.stearing_wheel_angle }}°
    </div>
    <div class="grid-cell value-cell">
      Stan stacyjki: {{ carState.ignition }}
    </div>
    <div class="grid-cell value-cell">Ręczny: {{ carState.handbrake }}%</div>
    <div class="grid-cell value-cell"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

enum Gear {
  REVERSE = "REVERSE",
  NEUTRAL = "NEUTRAL",
}

type GearType = Gear | { FORWARD: number };

function formatGear(gear: GearType): string {
  if (typeof gear === "string") {
    if (gear === Gear.REVERSE) return "R";
    if (gear === Gear.NEUTRAL) return "N";
    return "-";
  }

  if ("FORWARD" in gear) {
    return gear.FORWARD.toString();
  }

  return "-";
}

enum IgnitionState {
  NEUTRAL = "NEUTRAL",
  ON = "ON",
  OFF = "OFF",
}

interface Pedal {
  GAS: number;
  BREAK: number;
  CLUTCH: number;
}

interface CarState {
  current_gear: Gear | { FORWARD: number };
  dirrect_gear: Gear | { FORWARD: number };
  pbreak: { BREAK: number };
  pclutch: { CLUTCH: number };
  pgas: { GAS: number };
  stearing_wheel_angle: number;
  ignition: IgnitionState;
  handbrake: number;
}

const carState = ref<CarState>({
  current_gear: Gear.NEUTRAL,
  dirrect_gear: Gear.NEUTRAL,
  pbreak: { BREAK: 0 },
  pclutch: { CLUTCH: 0 },
  pgas: { GAS: 0 },
  stearing_wheel_angle: 0,
  ignition: IgnitionState.NEUTRAL,
  handbrake: 0,
});

let unlisten: any;

onMounted(async () => {
  await invoke("run_loop");
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
