<script setup lang="ts">
import { allTaskTypes } from "@/interface/TaskStatus";
import StartUpSettings from "./settings/StartUpSettings.vue";
import EmptySettings from "./settings/EmptySettings.vue";
import { onMounted, ref, shallowRef } from "vue";
import { MdTabs } from "@material/web/tabs/tabs";

const tabs = {
    StartUp: StartUpSettings,
    Award: EmptySettings,
};

const tabsElement = ref<MdTabs | null>(null);

const tabPaneComponentName = shallowRef(StartUpSettings);

onMounted(() => {
    if (tabsElement.value) {
        tabsElement.value.addEventListener("change", () => {
            const index = tabsElement.value?.activeTabIndex;
            if (index !== undefined) {
                tabPaneComponentName.value = tabs[allTaskTypes[index]];
            }
        });
    }
});
</script>

<template>
    <div class="mx-2 rounded-lg bg-white">
        <md-tabs ref="tabsElement" class="rounded-t-lg">
            <md-secondary-tab v-for="taskType in allTaskTypes">{{
                taskType
            }}</md-secondary-tab>
        </md-tabs>
        <component :is="tabPaneComponentName" class="rounded-lg"/>
    </div>
</template>
