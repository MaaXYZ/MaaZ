import { defineStore } from "pinia";
import { useDeviceStateStore } from "./DeviceStateStore";

export const useMaaStateStore = defineStore("maa-state", {
    state: () => {
        return {
            resourceInited: false,
        };
    },
    getters: {
        isMaaReady(state) {
            const deviceStateStore = useDeviceStateStore();
            return state.resourceInited && deviceStateStore.isDeviceConnected;
        },
    },
    actions: {
        noteResourceInited() {
            this.resourceInited = true;
        },
    },
});
