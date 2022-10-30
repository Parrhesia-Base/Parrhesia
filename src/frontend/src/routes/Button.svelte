<script lang="ts">
    let value = 0;

    function increment() {
        value += 1;
    }
</script>

<template>
    <button on:click={ increment }>{ value }</button>
</template>

<style>
    button {
        padding: 12px, 12px, 12px, 12px;
        font-size: 200px;
        aspect-ratio: 1;
        height: 300px;
    }
</style>