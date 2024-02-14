import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { taskViewModelInjectKey } from "./InjectKeys";
import TaskViewModel from "./viewmodel/TaskViewModel";
import { createPinia } from "pinia";

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

const taskViewModel = new TaskViewModel();
app.provide(taskViewModelInjectKey, taskViewModel);

app.mount("#app");
