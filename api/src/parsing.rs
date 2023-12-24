use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub sources: Vec<SourceData>,
}

#[derive(Deserialize, Debug)]
pub struct SourceData {
    pub name: String,
    pub search: Search,
    pub episodes: Episode,
    pub play: Play,
}

#[derive(Deserialize, Debug)]
pub struct Search {
    pub url: String,

    pub items: String,
    pub title: String,
    pub episodes_url: String,
    pub image: String,
}

#[derive(Deserialize, Debug)]
pub struct Episode {
    pub skip: bool,

    pub items: String,
    pub title: String,
    pub video_page_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Play {
    pub video_url: String,
}
