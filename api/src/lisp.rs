//! This is where we parse the lisp config file

use crate::SETTINGS_FILE;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use steel::{
    rvals::{FromSteelVal, SteelString},
    steel_vm::{engine::Engine, register_fn::RegisterFn},
    SteelErr, SteelVal,
};
use steel_derive::Steel;

// -- Parsing Results --
#[derive(Clone, Steel, PartialEq, Debug, Serialize, Deserialize)]
pub struct Media {
    title: String,
    episode_url: String,
    image: String,
}

#[derive(Clone, Steel, PartialEq, Debug, Serialize, Deserialize)]
pub struct MediaAndState {
    pub media: Vec<Media>,
    pub next_state: String,
}

impl MediaAndState {
    fn new(
        titles: Vec<String>,
        episode_urls: Vec<String>,
        images: Vec<String>,
        next_state: String,
    ) -> Self {
        let mut media: Vec<Media> = Vec::new();
        for ((title, episode_url), image) in titles.iter().zip(&episode_urls).zip(&images) {
            media.push(Media {
                title: title.clone(),
                episode_url: episode_url.clone(),
                image: image.clone(),
            });
        }

        Self { media, next_state }
    }
}

// -- Parsing Functions --
pub enum State {
    Options(MediaAndState),
    Video(String),
}

// TODO: Make code DRYer
pub fn search(source_name: &str, search_term: &str) -> MediaAndState {
    let mut vm = new_vm();
    let search_results_value = vm
        .call_function_from_struct(
            "source",
            source_name, // this is the name of the source instance
            "on-search",
            vec![SteelVal::StringV(SteelString::from(search_term))],
        )
        .unwrap();

    MediaAndState::from_steelval(&search_results_value).unwrap()
}

pub fn state(source_name: &str, link: &str, state_name: &str) -> State {
    let mut vm = new_vm();
    let state_results_value = vm
        .call_function_from_struct(
            "source",
            source_name,
            state_name,
            vec![SteelVal::StringV(SteelString::from(link))],
        )
        .unwrap();

    let media_and_state = MediaAndState::from_steelval(&state_results_value);
    if let Ok(media_and_state) = media_and_state {
        return State::Options(media_and_state);
    }

    // If the vm returns a Video instead of a MediaAndState, then it's a video
    let video = String::from_steelval(&state_results_value).unwrap();
    State::Video(video)
}

// -- Lisp functions --
fn get(url: String) -> String {
    ureq::get(&url).call().unwrap().into_string().unwrap()
}

fn select(body: String, query: String) -> Vec<String> {
    let mut elements: Vec<String> = Vec::new();
    let document = Html::parse_document(&body);

    for element in document.select(&Selector::parse(&query).unwrap()) {
        elements.push(element.html());
    }

    elements
}

fn select_one(body: String, query: String) -> String {
    let document = Html::parse_document(&body);

    document
        .select(&Selector::parse(&query).unwrap())
        .next()
        .unwrap()
        .html()
}

fn inner_text(element: String) -> String {
    let document = Html::parse_fragment(&element);
    let selector = Selector::parse("html *").unwrap();
    let root_element = document.select(&selector).next().unwrap();

    root_element.text().collect::<String>()
}

fn attribute(element: String, attribute: String) -> String {
    let document = Html::parse_fragment(&element);
    let selector = Selector::parse("html *").unwrap();
    let root_element = document.select(&selector).next().unwrap();

    root_element.value().attr(&attribute).unwrap().to_string()
}

fn split(string: String, split_by: String) -> Vec<String> {
    let mut split_vector: Vec<String> = Vec::new();

    string
        .split(&split_by)
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|x| split_vector.push(x.to_string()));

    split_vector
}

fn replace(string: String, from: String, to: String) -> String {
    string.replace(&from, &to)
}

// -- Engine extension --
// We will need to call functions inside of structs, we add this function into `Engine`
// so we can do that
trait Extension {
    fn call_function_from_struct(
        &mut self,
        struct_type_name: &str,
        struct_name: &str,
        struct_field: &str,
        arguments: Vec<SteelVal>,
    ) -> Result<SteelVal, SteelErr>;
}

impl Extension for Engine {
    fn call_function_from_struct(
        &mut self,
        struct_type_name: &str,
        struct_name: &str,
        struct_field: &str,
        mut arguments: Vec<SteelVal>,
    ) -> Result<SteelVal, SteelErr> {
        let helper_function = format!(
            r#"
(define (caller given-struct param)
  (({}-{} given-struct) param))
        "#,
            struct_type_name, struct_field
        );

        self.run(&helper_function)?;
        self.extract_value(struct_name).and_then(|struct_| {
            let mut args = vec![struct_];
            args.append(&mut arguments);

            self.call_function_by_name_with_args("caller", args)
        })
    }
}

// -- Utility functions --
fn new_vm() -> Engine {
    let mut vm = Engine::new();
    vm.register_type::<MediaAndState>("search-results");
    vm.register_fn("search-results", MediaAndState::new);
    vm.register_fn("get", get);
    vm.register_fn("select", select);
    vm.register_fn("select-one", select_one);
    vm.register_fn("inner-text", inner_text);
    vm.register_fn("attribute", attribute);
    vm.register_fn("split", split);
    vm.register_fn("replace", replace);
    let _ = vm.run(SETTINGS_FILE).unwrap();
    vm
}
