import { Ref, reactive, ref } from "vue";
import DeviceInfo from "../interface/DeviceInfo";
import CommandInvoker from "../CommandInvoker";

export default class DeviceViewModel {

    private _devices: DeviceInfo[] = reactive([]);

    private _connectedDevice: Ref<DeviceInfo | null> = ref(null);

    public get devices() : DeviceInfo[] {
        return this._devices
    }

    public get connectedDevice() : DeviceInfo | null {
        return this._connectedDevice.value;
    }

    public constructor() {
        console.log("DeviceViewModel created");
    }

    public async loadDevices() {
        const devices = await CommandInvoker.initDevices()
        this._devices = devices;
    }

    public async connectTo(device:DeviceInfo):Promise<void> {
        CommandInvoker.connectTo(device).then(() => {
            console.log("Connected to device", device);
            this._connectedDevice.value = device;
        });
    }
}