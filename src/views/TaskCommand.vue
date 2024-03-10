<script setup lang="ts">
import { useTaskQueueStore } from "@/stores/TaskQueueStore";
import { ref, watch } from "vue";
import { TaskType, allTaskTypes } from "@/interface/TaskStatus";
import CommandInvoker from "@/CommandInvoker";
import { useToast } from "vue-toast-notification";

const taskQueueStore = useTaskQueueStore();

const toast = useToast();

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
    console.log("queueAction");
    if (taskQueueStore.queueRunning) {
        taskQueueStore.stopQueue();
    } else if (taskQueueStore.hasPendingTasks) {
        taskQueueStore.startQueue();
    } else {
        toast.warning("No task in queue");
    }
}

function addTask(task: TaskType) {
    taskQueueStore.addToQueue(task).catch((err) => {
        toast.error(err.message);
    });
}

function startMiniWindow() {
    CommandInvoker.startMiniWindow().catch((e) => {
        toast.error(e.message);
    });
}
</script>

<template>
    <div class="-mr-3 rounded-lg bg-white p-2 pt-4">
        <md-filled-button class="w-full" @click="queueAction"
            >Start
            <md-icon slot="icon">play_arrow</md-icon>
        </md-filled-button>
        <md-outlined-button class="w-full mt-2" @click="startMiniWindow"
            >Mini Window</md-outlined-button
        >
        <div class="h-2" />
        <md-filled-tonal-button
            secondary
            type="primary"
            class="w-full mb-2"
            v-for="task in allTaskTypes"
            :key="task"
            @click="addTask(task)"
        >
            {{ task }}
        </md-filled-tonal-button>
    </div>
</template>

<style scoped></style>
