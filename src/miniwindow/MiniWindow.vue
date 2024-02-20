<script setup lang="ts">
import { useTaskQueueStore } from "@/stores/TaskQueueStore";
import { NFlex, NList, NListItem, NSpin, NIcon, NCheckbox } from "naive-ui";
import { onMounted, ref, watch } from "vue";
import { Error, Checkmark } from "@vicons/carbon";
import { invoke } from "@tauri-apps/api/core";

const taskQueueStore = useTaskQueueStore();

const onTop = ref(false);

onMounted(() => {
    taskQueueStore.updateQueue();
});

watch(onTop, (value) => {
    setWindowOnTop(value);
});

function setWindowOnTop(onTop: boolean) {
    invoke("set_window_on_top", {
        onTop: onTop
     });
}
</script>

<template>
    <div>
        <div
            class="select-none w-full flex items-center justify-center"
            v-if="!taskQueueStore.queueRunning"
        >
            <p class="text-center text-gray-500">Queue Stopped</p>
        </div>
        <n-flex class="select-none" vertical v-else>
            <n-list>
                <n-list-item v-for="task in taskQueueStore.taskQueue">
                    <n-flex justify="center" align="center">
                        <n-spin
                            size="small"
                            v-if="task.state === 'Running'"
                        ></n-spin>
                        <n-icon
                            size="small"
                            v-else-if="task.state === 'Failed'"
                        >
                            <Error />
                        </n-icon>
                        <n-icon
                            size="small"
                            v-else-if="task.state === 'Completed'"
                        >
                            <Checkmark />
                        </n-icon>
                        <p>{{ task.taskType }}</p>
                    </n-flex>
                </n-list-item>
            </n-list>
        </n-flex>

        <div class="flex fixed justify-center items-center w-full bottom-4">
            <n-checkbox v-model:checked="onTop">
                Always on top
            </n-checkbox>
        </div>
    </div>
</template>
