<script setup lang="ts">
import { computed, ref } from "vue";
import DeviceInfo from "../interface/DeviceInfo";
import { useDeviceStateStore } from "@/stores/DeviceStateStore";

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
    <div class="px-4 flex justify-between items-center">
        <div class="flex flex-col">
            <p class="truncate max-w-56">{{ props.device.name }}</p>
            <p class="text-gray-500 text-xs -mt-2">
                {{ props.device.adb_serial }}
            </p>
        </div>
        <md-outlined-icon-button
            :disabled="deviceConnected"
            @click="connectToDevice"
        >
            <md-icon color="#55EE55" v-if="deviceConnected"> done </md-icon>
            <md-icon v-else-if="!deviceConnecting"> send </md-icon>
            <md-circular-progress indeterminate v-if="deviceConnecting" />
        </md-outlined-icon-button>
    </div>
</template>
