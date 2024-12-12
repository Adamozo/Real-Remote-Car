<template>
  <div
    style="
      background-color: rgba(128, 128, 128, 0.3);
      width: 100%;
      padding: 0.5rem;
    "
  >
    <button @click="$emit('back')" class="start-button"><- Back</button>
  </div>
</template>

<script setup lang="ts">
defineEmits(["back"]);
</script>
