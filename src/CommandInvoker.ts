import { invoke } from "@tauri-apps/api";
import DeviceInfo from "./interface/DeviceInfo";

export default class CommandInvoker {

    /**
     * initDevices
     */
    public static async initDevices():Promise<DeviceInfo[]> {
        return invoke("init_devices");
    }

    public static async connectTo(device:DeviceInfo):Promise<void> {
        return invoke("connect_to_device", {device});
    }

    public static async startUpTask():Promise<void> {
        return invoke("start_up");
    }

    public static async initResources(): Promise<void> {
        return invoke("init_resources");
    }
}