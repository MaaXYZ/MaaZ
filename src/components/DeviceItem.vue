<script setup lang="ts">
import { computed, ref } from "vue";
import DeviceInfo from "../interface/DeviceInfo";
import { useDeviceStateStore } from "@/stores/DeviceStateStore";
import { NFlex, NButton, NIcon, NTooltip } from "naive-ui";
import { SendAlt, SendAltFilled } from "@vicons/carbon";

const props = defineProps<{
    device: DeviceInfo;
}>();

const deviceStateStore = useDeviceStateStore();

const deviceConnected = computed(() => {
    return (
        deviceStateStore.connectedDevice?.adb_serial === props.device.adb_serial
    );
});

const deviceConnecting = ref(false);

function connectToDevice() {
    deviceConnecting.value = true;
    deviceStateStore.connectTo(props.device).finally(() => {
        deviceConnecting.value = false;
    });
}
</script>

<template>
    <n-flex justify="space-between" align="center" class="px-4">
        <n-flex vertical>
            <n-tooltip :show-arrow="false" placement="bottom">
                <template #trigger>
                    <p>{{ props.device.name }}</p>
                </template>
                <n-flex vertical>
                    <p>{{ device.name }}</p>
                    <p>{{ device.adb_path }}</p>
                    <p>{{ device.adb_serial }}</p>
                    <p>{{ device.controller_type }}</p>
                </n-flex>
            </n-tooltip>
            <p class="text-gray-500 text-xs -mt-2">
                {{ props.device.adb_serial }}
            </p>
        </n-flex>
        <n-button
            :loading="deviceConnecting"
            :disabled="deviceConnected"
            @click="connectToDevice"
        >
            <template #icon>
                <n-icon color="#55EE55" v-if="deviceConnected">
                    <send-alt-filled />
                </n-icon>
                <n-icon v-else-if="!deviceConnecting">
                    <send-alt />
                </n-icon>
            </template>
        </n-button>
    </n-flex>
</template>
