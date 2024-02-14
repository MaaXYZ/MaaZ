<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import DeviceItem from "@/components/DeviceItem.vue";
import { NFlex, NList, NListItem, NSpin, NDivider } from "naive-ui";
import { useDeviceStateStore } from "@/stores/DeviceStateStore";

const loadingDevices = ref(true);

const deviceStateStore = useDeviceStateStore();

const connectedDeviceTitleClass = computed(() => {
    return {
        title_secondary: deviceStateStore.isDeviceConnected,
        title_primary: !deviceStateStore.isDeviceConnected,
    };
});

onMounted(() => {
    deviceStateStore.loadDevices().finally(() => {
        loadingDevices.value = false;
    });
});
</script>

<template>
    <n-flex vertical class="ml-3 mr-3 h-full">
        <n-flex vertical>
            <p :class="connectedDeviceTitleClass">Connected Device</p>
        </n-flex>

        <p v-if="!deviceStateStore.isDeviceConnected" class="text_small">
            No device connected
        </p>
        <device-item v-else :device="deviceStateStore.connectedDevice!" />

        <n-divider />

        <n-flex
            class="title_secondary"
            align="center"
            justify="center"
            v-if="loadingDevices"
        >
            <n-spin size="small"></n-spin>Searching for devices...
        </n-flex>
        <p
            class="title_secondary"
            v-else-if="deviceStateStore.devices.length == 0"
        >
            No devices found
        </p>
        <n-flex vertical v-else>
            <p>Available Devices</p>
            <n-list class="rounded-lg bg-transparent">
                <n-list-item
                    class="rounded-md hover:bg-gray-200 duration-300"
                    v-for="device in deviceStateStore.devices"
                    :key="device.name"
                >
                    <device-item :device="device" />
                </n-list-item>
            </n-list>
        </n-flex>
    </n-flex>
</template>

<style scoped>
.title_secondary {
    font-size: 1.5rem;
    color: #a0aec0;
    text-align: center;
    margin-top: 2rem;
}

.title_primary {
    font-size: 1.5rem;
    color: #2d3748;
    text-align: center;
    margin-top: 2rem;
}

.text_small {
    font-size: 1rem;
    color: #a0aec0;
    text-align: center;
}
</style>
