<script setup lang="ts">
import { onMounted } from "vue";
import { NButton, NFlex, NDivider } from "naive-ui";
import { useMaaStateStore } from "@/stores/MaaStateStore";
import { useTaskQueueStore } from "@/stores/TaskQueueStore";
import { listen } from "@tauri-apps/api/event";
import CallbackPayload from "@/interface/CallbackPayload";
import TaskQueue from "./TaskQueue.vue";
import { TaskType } from "@/interface/TaskStatus";

const maaStateStore = useMaaStateStore();

const taskQueueStore = useTaskQueueStore();

onMounted(() => {
    listen<CallbackPayload>("callback", (event) => {
        console.log(event);
        taskQueueStore.updateQueue();
    });
});

function addTask(task: TaskType) {
    console.log("Adding task");
    taskQueueStore.addToQueue(task);
}
</script>

<template>
    <n-flex vertical v-if="maaStateStore.isMaaReady" class="h-screen">
        <task-queue class="w-full h-1/5" />
        <n-divider />
        <n-button type="primary" class="w-1/4" @click="addTask('StartUp')"
            >StartUp</n-button
        >
        <n-button type="primary" class="w-1/4" @click="taskQueueStore.startQueue()"
            >StartQueue</n-button
            >
    </n-flex>
    <n-flex vertical v-else class="h-1/3" justify="center">
        <p class="text-center text-gray-400 text-4xl">
            MAA is not ready for tasks
        </p>
    </n-flex>
</template>
