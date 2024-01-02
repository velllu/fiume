<script setup lang="ts">
interface StateResult {
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
    `http://${config.public.apiAddress}:3001/state/${route.params.source}/${route.params.state}?link=${route.query.link}`

const { data } = await useFetch(url, {})
const state_result = data.value as string
const state_results = JSON.parse(state_result) as StateResult
</script>

<template>
    <ul>
        <li v-for="element in state_results.media">
            <Card
                :title="element.title"
                :episode_url="
                    '/state/' + route.params.source + '/' + state_results.next_state
                    + '?link=' + element.episode_url
                "
            />
        </li>
    </ul>
</template>