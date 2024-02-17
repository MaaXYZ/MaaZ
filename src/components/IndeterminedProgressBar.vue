<script setup lang="ts">

</script>

<template>
    <div class="indeterminate-progress-bar">
        <div class="indeterminate-progress-bar__bar"></div>
    </div>
</template>

<style scoped>

.indeterminate-progress-bar {
    position: relative;
    height: 4px;
    background-color: #f0f0f0;
    border-radius: 2px;
}

.indeterminate-progress-bar__bar {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    background-color: #000;
    border-radius: 2px;
    animation: indeterminate-progress-bar__bar-animation linear 1s infinite;
}

@keyframes indeterminate-progress-bar__bar-animation {
    0% {
        left: 0;
        width: 0;
    }
    41.1% {
        left: 0;
        width: 70%;
    }
    58.8% {
        left: 30%;
        width: 70%;
    }
    100% {
        left: 100%;
        width: 0;
    }
}

</style>