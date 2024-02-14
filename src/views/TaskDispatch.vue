<script setup lang="ts">
import { taskViewModelInjectKey } from "@/InjectKeys";
import { inject } from "vue";
import { NButton } from "naive-ui";
import { useMaaStateStore } from "@/stores/MaaStateStore";

const maaStateStore = useMaaStateStore();

const taskViewModel = inject(taskViewModelInjectKey)!;
</script>

<template>
    <div v-if="maaStateStore.isMaaReady">
        <n-button @click="taskViewModel.startUp()"> StartUp </n-button>
    </div>
    <div v-else>
        <p>MAA is not ready for tasks</p>
    </div>
</template>
