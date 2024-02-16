<script setup lang="ts">
import { taskViewModelInjectKey } from "@/InjectKeys";
import { inject, onMounted } from "vue";
import { NButton, NFlex } from "naive-ui";
import { useMaaStateStore } from "@/stores/MaaStateStore";
import { listen } from "@tauri-apps/api/event";
import CallbackPayload from "@/interface/CallbackPayload";

const maaStateStore = useMaaStateStore();

const taskViewModel = inject(taskViewModelInjectKey)!;

onMounted(() => {
    listen<CallbackPayload>("callback",(event)=>{
        
    })
});

</script>

<template>
    <div v-if="maaStateStore.isMaaReady" class="h-screen">
        <n-button @click="taskViewModel.startUp()"> StartUp </n-button>
    </div>
    <n-flex vertical v-else class=" h-1/3" justify="center">
        <p class=" text-center text-gray-400 text-4xl">MAA is not ready for tasks</p>
    </n-flex>
</template>
