<script setup>
const props = defineProps(["title", "episode_url"])
</script>

<template>
    <a :href="props.episode_url">
        <button>{{props.title}}</button>
    </a>
</template>

<style lang="sass" scoped>
button
    min-width: 100px
    height: 40px

    text-align: center
    padding: 0 15px

    box-shadow: $box-shadow

    animation: appear calc($animation-duration * 2)
</style>