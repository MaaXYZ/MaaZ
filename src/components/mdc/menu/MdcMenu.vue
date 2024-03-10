<script setup lang="ts" generic="T">
/**
 * Wrapper for MDC Menu
 */
import { onMounted, ref } from 'vue';
import { MenuOption } from './MenuProps';
import { MdMenu } from '@material/web/menu/menu';

const props = defineProps<{
    options?: MenuOption<T>[];
}>();

const emit = defineEmits<{
    selected: [value: T];
}>();

const triggerElement = ref<HTMLElement | null>(null);
const menuElement = ref<MdMenu | null>(null);

function openMenu() {
    if (triggerElement.value && menuElement.value) {
        menuElement.value.anchorElement = triggerElement.value;
        menuElement.value.open = true;
    }
}

onMounted(() => {
    for (const option of props.options || []) {
        const item = document.getElementById('item_' + option.value);
        if (item) {
            item.addEventListener('close-menu', () => {
                emit('selected', option.value);
            });
        }
    }
});

</script>

<template>
    <div class="container">
        <slot name="trigger" ref="triggerElement" @click="openMenu"></slot>
        <md-menu ref="menuElement">
            <md-menu-item v-for="option in options" :id="'item_'+option.value">
                {{ option.label }}
            </md-menu-item>
        </md-menu>
    </div>
</template>