<script setup lang="ts">
interface SearchResult {
    media: Array<Media>,
    next_state: string,
}

interface Media {
    title: string,
    episode_url: string,
    image: string,
}

const route = useRoute()
const config = useRuntimeConfig()
const url =
    `http://${config.public.apiAddress}:3001/search/${route.params.source}?search_term=${route.query.search_term}`

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
                :episode_url="
                    '/state/' + route.params.source + '?link=' + element.episode_url
                    + '&state=' + search_results.next_state
                "
                :image="element.image"
            />
        </li>
    </ul>
</template>