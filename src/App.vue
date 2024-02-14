<script setup lang="ts">
import { NFlex, NSplit, NConfigProvider, GlobalThemeOverrides } from "naive-ui";
import DeviceConnection from "./views/DeviceConnection.vue";
import TaskDispatch from "./views/TaskDispatch.vue";
import { onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import CommandInvoker from "./CommandInvoker";
import { useMaaStateStore } from "./stores/MaaStateStore";

const theme: GlobalThemeOverrides = {
    common: {
        primaryColor: "#7F5610",
    },
};

const maaStateStore = useMaaStateStore();

onMounted(() => {
    CommandInvoker.initResources().then(() => {
        console.log("Resources initialized");
        maaStateStore.noteResourceInited();
    });

    listen("callback", (event) => {
        console.log("Callback received: ", event.payload);
    });
});
</script>

<template>
    <n-config-provider :themeOverrides="theme">
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
    </n-config-provider>
</template>
