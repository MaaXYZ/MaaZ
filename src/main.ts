import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import DeviceViewModel from "./viewmodel/DeviceViewModel";
import { deviceViewModelInjectKey } from "./InjectKeys";

const app = createApp(App)

const deviceViewModel = new DeviceViewModel()
app.provide(deviceViewModelInjectKey, deviceViewModel)

app.mount("#app");
