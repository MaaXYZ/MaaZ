import CommandInvoker from "@/CommandInvoker";
import TaskStatus, { TaskType } from "@/interface/TaskStatus";
import { defineStore } from "pinia";

export const useTaskQueueStore = defineStore("task-queue", {
    state: () => {
        return {
            taskQueue: [] as TaskStatus[],
        };
    },
    actions: {
        async removeFromQueue(index: number) {
            CommandInvoker.removeFromQueue(index).then(() => {
                this.taskQueue.splice(index, 1);
            });
        },
        async addToQueue(task: TaskType, append_next: boolean = false) {
            CommandInvoker.addTaskToQueue(task, append_next).then(() => {
                this.updateQueue();
            });
        },
        async updateQueue() {
            CommandInvoker.getQueue().then((queue) => {
                this.taskQueue.splice(0, this.taskQueue.length, ...queue);
            });
        },
        async startQueue() {
            CommandInvoker.startQueue().then(() => {
                this.updateQueue();
            });
        }
    },
});
