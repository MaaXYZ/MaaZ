<script setup lang="ts">
import TaskStatus from '@/interface/TaskStatus';
import { computed, ref } from 'vue';
import { useTaskQueueStore } from '@/stores/TaskQueueStore';
import { NButton, NSpin } from 'naive-ui';

const taskQueueStore = useTaskQueueStore();

const props = defineProps<{
    index: number,
    task: TaskStatus
}>();

const showRemoveButton = ref(false);

function mouseEnter() {
    if (props.task.state!=="Running") {
        showRemoveButton.value = true;
    }
}

function mouseLeave() {
    showRemoveButton.value = false;
}

const backgroundColor = computed(() => {
    if (props.task.state === "Running") {
        return "#FFD700";
    } else if (props.task.state === "Completed") {
        return "#00FF00";
    } else if (props.task.state === "Failed") {
        return "#FF0000";
    } else {
        return "#EBEBE4";
    }
});

</script>

<template>
    <div @mouseenter="mouseEnter" :style="{ backgroundColor:backgroundColor }" @mouseleave="mouseLeave" class="item mx-1 inline-block">
        <p class="text-center"><n-spin v-if="task.state==='Running'"></n-spin>{{ props.task.taskType }}</p>
        <n-button v-if="showRemoveButton" type="error" class="w-full" @click="taskQueueStore.removeFromQueue(props.index)">Remove</n-button>
    </div>
</template>

<style scope>

.item {
    min-width: 150px;
    width: 150px;
    height: 60px;
    border-radius: 0.5rem;
}

</style>