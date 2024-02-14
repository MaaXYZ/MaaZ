import CommandInvoker from "@/CommandInvoker";

export default class TaskViewModel {


    constructor() {

    }

    public async startUp() {
        await CommandInvoker.startUpTask()
    }
}