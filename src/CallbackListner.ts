import { listen } from "@tauri-apps/api/event";
import { useTaskQueueStore } from "./stores/TaskQueueStore";
import CallbackPayload from "./interface/CallbackPayload";

export const setupListener = () => {

    const taskQueueStore = useTaskQueueStore();

    listen<CallbackPayload>("callback", (event) => {
        console.log("Callback received: ", event.payload);
        taskQueueStore.updateQueue();
    });

    listen("queue-done", (_event) => {
        taskQueueStore.queueRunning = false;
    });
};
