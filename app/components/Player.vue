<script setup>
const props = defineProps(["link"])
</script>

<template>
    <video autoplay="autoplay" controls>
        <source :src="props.link"/>
    </video>
</template>