<template>
    <div class="gallery">
        <slot/>
    </div>
</template>

<style lang="sass" scoped>
.gallery
    display: flex
    flex-wrap: wrap
    justify-content: center
    align-items: center
    gap: 15px
</style>