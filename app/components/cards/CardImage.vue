<script setup>
const props = defineProps(["title", "episode_url", "image"])
</script>

<template>
    <div class="card">
        <a :href="props.episode_url">
            <div class="shadow"></div>
            <img v-if="props.image" :src="props.image">
            <p>{{props.title}}</p>
        </a>
    </div>
</template>

<style lang="sass" scoped>
$image-size: 250px

.shadow
    width: 100%
    height: 100%

    top: 0
    left: 0

    border-radius: $border-radius

    position: absolute

    opacity: 0
    background: linear-gradient(#00000000 60%, $dark-background 100%)
    transition: opacity calc($animation-duration * 1.5)

img
    width: $image-size
    height: auto

    border-radius: $border-radius

    object-fit: cover
    max-width: 100%
    height: auto
    vertical-align: middle

p
    position: absolute
    bottom: 0
    left: 0

    margin-bottom: 5px
    margin-left: 5px

    opacity: 0
    transform: translateY(30px)
    transition: opacity $animation-duration, transform $animation-duration

a
    &:hover, &:focus
        .shadow
            opacity: 1
        
        p
            opacity: 1
            transform: translateY(0)

.card
    width: $image-size
    flex-basis: $image-size
    height: auto

    border-radius: $border-radius
    border: $border

    box-shadow: $box-shadow

    position: relative
    animation: appear calc($animation-duration * 2)
</style>
