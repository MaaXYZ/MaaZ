<script setup lang="ts">
import { reactive } from 'vue';
import DeviceInfo from './interface/DeviceInfo';
import CommandInvoker from './CommandInvoker';

const devices = reactive<DeviceInfo[]>([])

CommandInvoker.initDevices().then((deviceInfos: DeviceInfo[]) => {
  devices.push(...deviceInfos)
})

</script>

<template>
  <div class="container">
    <p>{{ devices }}</p>
  </div>
</template>
