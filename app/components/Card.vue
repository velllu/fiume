<script setup>
const props = defineProps(["title", "episode_url", "image"])
</script>

<template>
    <div class="card">
        <img :src="props.image">
        <a :href="props.episode_url">{{props.title}}</a>
    </div>
</template>

<style lang="sass" scoped>
img
    width: 100px
    height: auto

    margin-right: $small-margin

    border-radius: $border-radius

.card
    width: 600px

    display: flex
    align-items: center

    margin-bottom: $small-margin
</style>