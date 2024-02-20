<script setup lang="ts">
import { onMounted } from "vue";
import { NFlex, NDivider, useMessage } from "naive-ui";
import { useMaaStateStore } from "@/stores/MaaStateStore";
import TaskQueue from "./TaskQueue.vue";
import TaskSettings from "./TaskSettings.vue";
import TaskCommand from "./TaskCommand.vue";

const maaStateStore = useMaaStateStore();

const messager = useMessage();

onMounted(() => {
    maaStateStore.getConfig().catch((err) => {
        messager.error("Failed to get MAA config: " + err);
    });
});
</script>

<template>
    <n-flex vertical v-if="maaStateStore.isMaaReady" class="h-screen">
        <task-queue class="w-full h-1/5" />
        <n-divider />
        <n-flex class="w-full">
            <task-command class="w-1/5" />
            <task-settings class="grow" />
        </n-flex>
    </n-flex>
    <n-flex vertical v-else class="h-1/3" justify="center">
        <p class="text-center text-gray-400 text-4xl">
            MAA is not ready for tasks
        </p>
    </n-flex>
</template>
