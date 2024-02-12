import { InjectionKey } from "vue";
import DeviceViewModel from "./viewmodel/DeviceViewModel";

export const deviceViewModelInjectKey = Symbol("deviceViewModel") as InjectionKey<DeviceViewModel>
