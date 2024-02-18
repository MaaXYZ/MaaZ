<script setup lang="ts">
import { NFlex, NRadioGroup, NRadioButton, useMessage } from "naive-ui";
import { ref } from "vue";
import { ClientType, allClientTypes } from "@/interface/StartUpConfig";
import { useMaaStateStore } from "@/stores/MaaStateStore";

const maaStateStore = useMaaStateStore();

const messager = useMessage();

const clientType = ref<ClientType>(
    maaStateStore.config?.startUp.clientType ?? "Official"
);

function setClientType(v: ClientType) {
    clientType.value = v;
    maaStateStore.setClientType(clientType.value).catch((error) => {
        messager.error(error.message);
    });
}
</script>

<template>
    <n-flex vertical>
        <n-flex class="settings_item" justify="space-between">
            <p>Client Type</p>
            <n-radio-group
                :value="clientType"
                @update:value="(v:ClientType)=>{setClientType(v)}"
            >
                <n-radio-button
                    v-for="cType in allClientTypes"
                    :key="cType"
                    :value="cType"
                >
                    {{ cType }}
                </n-radio-button>
            </n-radio-group>
        </n-flex>
    </n-flex>
</template>
