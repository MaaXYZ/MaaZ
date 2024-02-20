import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import { setupListener } from "./CallbackListner";

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

setupListener();

app.mount("#app");
