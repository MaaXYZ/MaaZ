import { invoke } from "@tauri-apps/api";
import DeviceInfo from "./interface/DeviceInfo";

export default class CommandInvoker {

    /**
     * initDevices
     */
    public static async initDevices():Promise<DeviceInfo[]> {
        return invoke("init_devices");
    }
}