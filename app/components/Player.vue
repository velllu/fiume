<script setup>
const props = defineProps(["link"])
</script>

<template>
    <video autoplay="autoplay" controls>
        <source :src="props.link"/>
    </video>
</template>

<style lang="sass" scoped>
video
    box-shadow: $box-shadow
    width: 100%
</style>