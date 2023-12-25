use crate::parsing::get_sources_from_names;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use super::{Api, SearchQuery};

#[derive(Deserialize, Serialize)]
struct Media {
    title: String,
    episode_url: String,
    image: String,
}

#[derive(Deserialize, Serialize)]
struct SearchResponse {
    media: Vec<Media>,
}

// TODO: Handle errors, i am focusing firstly on making a working prototype

fn get_attribute(element: &ElementRef, attribute: &str) -> String {
    if attribute == "~text" {
        element.text().next().unwrap().to_string()
    } else {
        element.attr(attribute).unwrap().to_string()
    }
}

impl Api {
    pub async fn search(&self, params: SearchQuery) -> String {
        let sources = get_sources_from_names(&params.sources_name).unwrap();

        let mut titles: Vec<String> = Vec::new();
        let mut episode_urls: Vec<String> = Vec::new();
        let mut images: Vec<String> = Vec::new();

        // We will pack the above `Vec`s into just a vec to make it more fool-proof for
        // the client to parse
        let mut media: Vec<Media> = Vec::new();

        for source in sources {
            let search_url = source
                .search
                .url
                .replace("%SEARCH_TERM%", &params.search_term);

            // We get the HTML of the website
            let request = self.client.get(search_url).build().unwrap();
            let response = self.client.execute(request).await.unwrap();

            // And we parse what we need from it
            let document = Html::parse_document(&response.text().await.unwrap());

            // This contains every title/image/etc...
            let container = document
                .select(&Selector::parse(&source.search.items).unwrap())
                .next()
                .unwrap();

            // Getting the titles
            let (query, attribute) = source.search.title.split_once(":").unwrap();
            container
                .select(&Selector::parse(query).unwrap())
                .for_each(|title| titles.push(get_attribute(&title, attribute)));

            // Getting the episode urls
            let (query, attribute) = source.search.episodes_url.split_once(":").unwrap();
            container
                .select(&Selector::parse(query).unwrap())
                .for_each(|episode_url| episode_urls.push(get_attribute(&episode_url, attribute)));

            // Getting the images urls
            let (query, attribute) = source.search.image.split_once(":").unwrap();
            container
                .select(&Selector::parse(query).unwrap())
                .for_each(|image| images.push(get_attribute(&image, attribute)));
        }

        for ((title, episode_url), image) in titles.iter().zip(episode_urls).zip(images) {
            media.push(Media {
                title: title.clone(),
                episode_url,
                image,
            });
        }

        serde_json::to_string(&SearchResponse { media })
            .unwrap()
            .to_string()
    }
}
