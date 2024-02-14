import { invoke } from "@tauri-apps/api";
import DeviceInfo from "./interface/DeviceInfo";

export default class CommandInvoker {

    public static async initMaa(): Promise<void> {
        return invoke("init_maa");
    }

    public static async findDevices():Promise<DeviceInfo[]> {
        return invoke("find_devices");
    }

    public static async connectTo(device:DeviceInfo):Promise<void> {
        return invoke("connect_to_device", {device});
    }

    public static async startUpTask():Promise<void> {
        return invoke("start_up");
    }
}