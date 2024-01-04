<script setup>
// TODO: Make the default not your-source
function search() {
    let search_query = document.getElementById("search_input").value
    location.href = "/search/your-source?search_term=" + search_query
}
</script>

<template>
    <nav>
        <a href="/"><h1>Fiume</h1></a>

        <form action="javascript:void(0)">
            <input type="text" placeholder="Search..." id="search_input"/>
            <button @click="search">Search</button>
        </form>
    </nav>
</template>

<style lang="sass" scoped>
h1
    margin-right: 12em

input, button
    padding: 10px

input
    margin-right: $small-margin

nav
    display: flex
    align-items: center
    justify-content: center

    position: sticky
    top: 0
    left: 0

    z-index: 1

    width: 100vw
    height: 4rem

    background-color: $dark-background

    border-bottom: $border
    box-shadow: $box-shadow
</style>