import { createApp } from "vue";
import MiniWindow from "./MiniWindow.vue";
import { createPinia } from "pinia";

const app = createApp(MiniWindow)

const pinia = createPinia()
app.use(pinia)

app.mount("#miniwindow")