<script setup>
function search() {
    let search_query = document.getElementById("search_input").value
    location.href = "/search?search_term=" + search_query
}
</script>

<template>
    <nav>
        <h1>Fiume</h1>

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

    width: 100vw
    height: 3rem

    background-color: #0D2B47
</style>