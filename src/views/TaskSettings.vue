<script setup lang="ts">
import { NTabs, NTabPane, NCard } from "naive-ui";
import { allTaskTypes } from "@/interface/TaskStatus";
import StartUpSettings from "./settings/StartUpSettings.vue";
import EmptySettings from "./settings/EmptySettings.vue";

const tabs = {
    StartUp: StartUpSettings,
    Award: EmptySettings
};
</script>

<template>
    <div>
        <n-card class="mx-2 rounded-lg">
            <n-tabs type="bar" animated default-value="StartUp">
                <n-tab-pane
                    v-for="taskType in allTaskTypes"
                    :key="taskType"
                    :name="taskType"
                >
                    <div>
                        <component :is="tabs[taskType]" />
                    </div>
                </n-tab-pane>
            </n-tabs>
        </n-card>
    </div>
</template>
