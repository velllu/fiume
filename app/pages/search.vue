<script setup lang="ts">
interface SearchResult {
    media: Array<Media>
}

interface Media {
    title: string,
    episode_url: string,
    image: string,
}

const route = useRoute()
const config = useRuntimeConfig()
const url =
    `http://${config.public.apiAddress}:3001/search/your-source?search_term=${route.query.search_term}`
console.log(url)

const { data } = await useFetch(url, {})
const search_result = data.value as string
const search_results = JSON.parse(search_result) as SearchResult
const media = search_results.media
</script>

<template>
    <ul>
        <li v-for="element in media">
            <Card
                :title="element.title"
                :episode_url="element.episode_url"
                :image="element.image"
            />
        </li>
    </ul>
</template>

<style lang="sass" scoped>
ul
    display: flexbox
</style>
