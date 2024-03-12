import { createApp } from "vue";
import MiniWindow from "./MiniWindow.vue";
import { createPinia } from "pinia";
import "../styles/styles.css"
import "../styles/theme.css"
import { setupListener } from "@/CallbackListner";

import '@material/web/list/list'
import '@material/web/list/list-item'
import '@material/web/progress/circular-progress'
import '@material/web/checkbox/checkbox'

const app = createApp(MiniWindow);

const pinia = createPinia();
app.use(pinia);

setupListener();

app.mount("#miniwindow");
