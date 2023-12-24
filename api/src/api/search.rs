use crate::parsing::get_sources_from_names;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use super::{Api, SearchQuery};

#[derive(Deserialize, Serialize)]
struct SearchResponse {
    titles: Vec<String>,
    episode_urls: Vec<String>,
    images: Vec<String>,
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
        let mut search_results = SearchResponse {
            titles: Vec::new(),
            episode_urls: Vec::new(),
            images: Vec::new(),
        };

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
                .for_each(|title| search_results.titles.push(get_attribute(&title, attribute)));

            // Getting the episode urls
            let (query, attribute) = source.search.episodes_url.split_once(":").unwrap();
            container
                .select(&Selector::parse(query).unwrap())
                .for_each(|episode_url| {
                    search_results
                        .episode_urls
                        .push(get_attribute(&episode_url, attribute))
                });

            // Getting the images urls
            let (query, attribute) = source.search.image.split_once(":").unwrap();
            container
                .select(&Selector::parse(query).unwrap())
                .for_each(|image| search_results.images.push(get_attribute(&image, attribute)));
        }

        serde_json::to_string(&search_results).unwrap().to_string()
    }
}
