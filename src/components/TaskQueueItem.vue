<script setup lang="ts">
import TaskStatus from "@/interface/TaskStatus";
import { computed, onMounted, ref } from "vue";
import IndeterminedProgressBar from "./IndeterminedProgressBar.vue";
import { MdRipple } from "@material/web/ripple/ripple";

const props = defineProps<{
    index: number;
    task: TaskStatus;
}>();

const showRemoveButton = ref(false);

const ripple = ref<MdRipple | null>(null);
const outer = ref<HTMLDivElement | null>(null);

onMounted(() => {
    if (outer.value && ripple.value ) {
        ripple.value.attach(outer.value)
    }
});

function mouseEnter() {
    if (props.task.state !== "Running") {
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
    <div
        @mouseenter="mouseEnter"
        :style="{ backgroundColor: backgroundColor }"
        @mouseleave="mouseLeave"
        ref="outer"
        class="item mx-1 text-center items-center shadow relative"
    >
    <md-ripple ref="ripple"></md-ripple>
        <div class="flex flex-col w-full h-full layer">
            <p class="text-center">{{ props.task.taskType }}</p>
            <indetermined-progress-bar
                v-if="props.task.state === 'Running'"
                class="w-11/12"
            ></indetermined-progress-bar>
        </div>
    </div>
</template>

<style scope>
.item {
    min-width: 150px;
    width: 150px;
    height: 60px;
    border-radius: 0.5rem;
    background-color: var(--md-ref-palette-neutral95);
}

</style>
