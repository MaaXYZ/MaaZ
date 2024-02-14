import { InjectionKey } from "vue";
import TaskViewModel from "./viewmodel/TaskViewModel";

export const taskViewModelInjectKey = Symbol(
    "taskViewModel"
) as InjectionKey<TaskViewModel>;
