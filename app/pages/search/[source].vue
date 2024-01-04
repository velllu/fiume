<script setup lang="ts">
const route = useRoute()
const url = get_url(`/search/${route.params.source}?search_term=${route.query.search_term}`)

const { data } = await useFetch(url, {})
const search_result = data.value as string
const search_results = JSON.parse(search_result) as MediaAndState
</script>

<template>
    <ul>
        <li v-for="media in search_results.media">
            <Card
                :title="media.title"
                :episode_url="
                    '/state/' + route.params.source + '/' + search_results.next_state
                    + '?link=' + media.episode_url
                "
                :image="media.image"
            />
        </li>
    </ul>
</template>