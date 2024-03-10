import { createApp } from "vue";
import "./styles/styles.css";
import "./styles/theme.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import { setupListener } from "./CallbackListner";
import ToastPlugin from 'vue-toast-notification';
import 'vue-toast-notification/dist/theme-sugar.css';

import '@material/web/icon/icon'
import '@material/web/iconbutton/icon-button'
import '@material/web/iconbutton/outlined-icon-button'
import '@material/web/button/filled-button'
import '@material/web/button/outlined-button'
import '@material/web/button/filled-tonal-button'
import '@material/web/progress/circular-progress'
import '@material/web/list/list'
import '@material/web/list/list-item'
import '@material/web/menu/menu'
import '@material/web/menu/menu-item'
import '@material/web/select/select-option'
import '@material/web/select/filled-select'
import '@material/web/select/outlined-select'
import '@material/web/tabs/tabs'
import '@material/web/tabs/secondary-tab'

const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

app.use(ToastPlugin)

setupListener();

app.mount("#app");
