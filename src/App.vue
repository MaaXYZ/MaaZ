<script setup lang="ts">
import { NFlex, NSplit, NConfigProvider, NMessageProvider } from "naive-ui";
import DeviceConnection from "./views/DeviceConnection.vue";
import TaskDispatch from "./views/TaskDispatch.vue";
import { onMounted } from "vue";
import CommandInvoker from "./CommandInvoker";
import { useMaaStateStore } from "./stores/MaaStateStore";

// const theme: GlobalThemeOverrides = {};

const maaStateStore = useMaaStateStore();

onMounted(() => {
    CommandInvoker.initMaa().then(() => {
        console.log("Maa initialized");
        maaStateStore.noteResourceInited();
    });
});
</script>

<template>
    <n-config-provider>
        <n-message-provider>
            <div class="select-none">
                <n-flex class="h-screen">
                    <n-split
                        direction="horizontal"
                        :max="0.4"
                        :min="0.2"
                        class="h-full"
                        :default-size="0.3"
                    >
                        <template #1>
                            <device-connection />
                        </template>
                        <template #2>
                            <task-dispatch />
                        </template>
                    </n-split>
                </n-flex>
            </div>
        </n-message-provider>
    </n-config-provider>
</template>
