<script setup lang="ts">
import type { Video } from '~/utils' // idk why this wont auto-import

enum StateType {
    Options,
    Video,
}

const route = useRoute()
const url = get_url(`/state/${route.params.source}/${route.params.state}?link=${route.query.link}`)

const { data } = await useFetch(url, {})
const state_result = data.value as string
const state_result_json: JSON = JSON.parse(state_result)

// WARNING:
// These are unsafe to access if the state is not `StateType.Options`
const media_and_state: MediaAndState = (state_result_json as any) as MediaAndState

// and this is unsafe to access if the state is not `StateType.Video`
const video: Video = (state_result_json as any) as Video

let state: StateType;

if ("link" in state_result_json) {
    // Then this state returned a video, and we do not present the user with options
    state = StateType.Video
} else {
    // If there's no "link" key, then Steel returned various options for the user to
    // select
    state = StateType.Options
}
</script>

<template>
    <!-- Video -->
    <Player v-if="state == StateType.Video" :link="video.link"/>

    <!-- Options -->
    <CardGrid v-if="state == StateType.Options">
        <CardsCardButton
            v-if="!should_display_image(media_and_state.media)"
            v-for="media in media_and_state.media"
            :title="media.title"
            :episode_url="
                '/state/' + route.params.source + '/' + media_and_state.next_state
                + '?link=' + media.episode_url
            "
        />

        <CardsCardImage
            v-if="should_display_image(media_and_state.media)"
            v-for="media in media_and_state.media"
            :title="media.title"
            :episode_url="
                '/state/' + route.params.source + '/' + media_and_state.next_state
                + '?link=' + media.episode_url
            "
            :image="media.image"
        />
    </CardGrid>
    <!-- <ul v-if="state == StateType.Options">
        <CardGrid>
            <Card
                v-for="element in media_and_state.media"
                :title="element.title"
                :episode_url="
                    '/state/' + route.params.source + '/' + media_and_state.next_state
                    + '?link=' + element.episode_url
                "
                :image="element.image == 'None' ? null : element.image"
            />
        </CardGrid>
    </ul> -->
</template>