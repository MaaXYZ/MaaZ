<script setup lang="ts" generic="T">
import { computed, onMounted, ref } from 'vue';
import { SelectOption, SelectVariant } from './SelectProps';
import { MdFilledSelect } from '@material/web/select/filled-select';
import { MdOutlinedSelect } from '@material/web/select/outlined-select';

const props = defineProps<{
    options?: SelectOption[];
    variant?: SelectVariant;
}>();

const value = defineModel<string>()

const selectElement = ref<MdFilledSelect | MdOutlinedSelect | null>(null);

const componentName = computed(() => {
    return props.variant === 'outlined' ? 'md-outlined-select' : 'md-filled-select';
});

onMounted(() => {
    if (selectElement.value) {
        selectElement.value.value = value.value ?? '';
        selectElement.value.addEventListener('closed', ()=>{
            value.value = selectElement.value?.value;
        })
    }
});

</script>

<template>
    <div class="container w-fit">
        <component :is="componentName" ref="selectElement">
            <md-select-option v-for="option in options" :value="option.value">
                {{ option.label }}
            </md-select-option>
        </component>
    </div>
</template>