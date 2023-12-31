export interface MediaAndState {
    media: Array<Media>,
    next_state: string,
}

export interface Video {
    link: string
}

export interface Media {
    title: string,
    episode_url: string,
    image: string,
}

export function get_url(path: string): string {
    const config = useRuntimeConfig()
    return `http://${config.public.apiAddress}:3001${path}`
}

export function should_display_image(media: Array<Media>): boolean {
    for (let i = 0; i < media.length; i++) {
        if (media[i].image == "None") {
            return false
        }
    }

    return true
}