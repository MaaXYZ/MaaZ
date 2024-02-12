<script setup lang="ts">
import { computed, inject, onMounted, ref } from "vue";
import { deviceViewModelInjectKey } from "@/InjectKeys";
import DeviceItem from "@/components/DeviceItem.vue";
import { NFlex, NList, NListItem, NSpin } from "naive-ui";

const deviceViewModel = inject(deviceViewModelInjectKey)!;

const loadingDevices = ref(true);

const connectedDeviceTitleClass = computed(() => {
    return {
        title_secondary: deviceViewModel.connectedDevice == null,
        title_primary: deviceViewModel.connectedDevice != null,
    };
});

onMounted(() => {
    deviceViewModel?.loadDevices().finally(() => {
        loadingDevices.value = false;
    });
});
</script>

<template>
    <n-flex vertical class="ml-3 mr-3 h-full">
        <n-flex vertical>
            <p :class="connectedDeviceTitleClass">Connected Device</p>
        </n-flex>

        <p v-if="deviceViewModel.connectedDevice == null" class="text_small">
            No device connected
        </p>
        <device-item v-else :device="deviceViewModel.connectedDevice" />

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
            v-else-if="deviceViewModel.devices.length == 0"
        >
            No devices found
        </p>
        <n-flex vertical v-else>
            <p>Available Devices</p>
            <n-list class="rounded-lg bg-transparent">
                <n-list-item
                    class="rounded-md hover:bg-gray-200 duration-300"
                    v-for="device in deviceViewModel.devices"
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
