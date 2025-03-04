use crate::recipe::types::{Kind, Recipe};
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use scraper::html::Select;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

impl Recipe {
    fn validate(&self) -> bool {
        let expected_context = String::from("https://schema.org");
        let expect_content_with_s = String::from("http://schema.org");
        let expected_type = String::from("Recipe");

        if let (Some(context), Some(kind)) = (self.context.clone(), self.kind.clone()) {
            let context_bool = context == expected_context || context == expect_content_with_s;
            let kind_bool = match kind {
                Kind::Text(s) => s == expected_type,
                Kind::TextList(l) => l.contains(&expected_type),
            };
            return context_bool && kind_bool;
        }
        false
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum RecipeScript {
    JSONObject(Recipe),
    JSONArray(Vec<Recipe>),
}

fn get_html_document(url: &str) -> Option<Html> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_str("text/html").unwrap());
    // You are a browser, no one will know
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36").unwrap());

    let client = blocking::Client::new();
    let response = client.get(url).headers(headers).send();

    match response.ok()?.text().ok() {
        Some(content) => return Some(scraper::Html::parse_document(&content)),
        None => return None,
    };
}

fn parse_recipe(scripts: &mut Select) -> Option<Recipe> {
    scripts.find_map(|script| {
        let html = script.inner_html();
        match serde_json::from_str(&html) {
            Ok(recipe_script) => match recipe_script {
                RecipeScript::JSONArray(recipe_scripts) => {
                    for recipe_script in recipe_scripts {
                        if recipe_script.validate() {
                            return Some(recipe_script);
                        }
                    }
                    return None;
                }
                RecipeScript::JSONObject(recipe_script) => {
                    if recipe_script.validate() {
                        return Some(recipe_script);
                    }
                    return None;
                }
            },
            Err(error) => {
                println!("Our: {error}");
                return None;
            }
        }
    })
}

pub fn fetch(url: &str) -> Option<Recipe> {
    // fetch
    let document = match get_html_document(url) {
        Some(doc) => doc,
        None => return None,
    };

    let selector: Selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();

    let mut ld_scripts = document.select(&selector);

    parse_recipe(&mut ld_scripts)
}
