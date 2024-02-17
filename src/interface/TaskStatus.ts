export type TaskType = 'StartUp'

export type TaskState = 'Pending' | 'Running' | 'Completed' | 'Failed'

export default interface TaskStatus {
    id?: number,
    taskType: TaskType,
    state: TaskState,
}