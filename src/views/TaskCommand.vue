<script setup lang="ts">
import { useTaskQueueStore } from "@/stores/TaskQueueStore";
import { NCard, NIcon, NDivider, NButton, useMessage } from "naive-ui";
import { PlayFilledAlt as Play, StopFilledAlt as Stop } from "@vicons/carbon";
import { ref, watch } from "vue";
import { TaskType, allTaskTypes } from "@/interface/TaskStatus";

const taskQueueStore = useTaskQueueStore();

const messager = useMessage();

const outer = ref<HTMLDivElement | null>(null);
const outerHeight = ref(0);

watch(outer, (el) => {
    if (el) {
        outerHeight.value = el.clientWidth;
    }
});

window.addEventListener("resize", () => {
    if (outer.value) {
        outerHeight.value = outer.value.clientWidth;
    }
});

function queueAction() {
    if (taskQueueStore.queueRunning) {
        taskQueueStore.stopQueue();
    } else if (taskQueueStore.hasPendingTasks) {
        taskQueueStore.startQueue();
    } else {
        messager.warning("No task in queue");
    }
}

function addTask(task: TaskType) {
    taskQueueStore.addToQueue(task).catch((err) => {
        messager.error(err.message);
    });
}
</script>

<template>
    <n-card class="-mr-3 rounded-lg">
        <div
            ref="outer"
            :style="{ height: outerHeight + 'px' }"
            class="w-full flex flex-wrap justify-center items-center bg-slate-100 rounded-lg hover:bg-slate-200 duration-300"
            @click="queueAction"
        >
            <n-icon :size="outerHeight * 0.5 + 'px'">
                <Stop v-if="taskQueueStore.queueRunning" />
                <Play v-else />
            </n-icon>
        </div>
        <n-divider />
        <n-button
            secondary
            type="primary"
            class="w-full mb-2"
            v-for="task in allTaskTypes"
            :key="task"
            @click="addTask(task)"
        >
            {{ task }}
        </n-button>
    </n-card>
</template>

<style scoped></style>
