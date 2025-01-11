use reqwest::{blocking};
use reqwest::header::{HeaderMap,HeaderValue, ACCEPT, USER_AGENT};
use scraper::{Selector};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Image {
    URL(String),
    ImageObject(std::collections::HashMap<String, serde_json::Value>)
}

#[derive(Serialize, Deserialize, Debug)]
struct RecipeInstruction {
    #[serde(rename = "@type")]
    kind: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum RecipeYield {
    Text(String),
    Number(u8)
}


#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@type")]
    kind: String,
    #[serde(rename = "cookTime")]
    cook_time: Option<String>,
    #[serde(rename = "datePublished")]
    date_published: Option<String>,
    description: Option<String>,
    image: Option<Image>,
    #[serde(rename = "recipeIngredient")]
    recipe_ingredient: Option<Vec<String>>,
    name: Option<String>,
    #[serde(rename = "recipeInstructions")]
    recipe_instructions: Option<Vec<RecipeInstruction>>,
    #[serde(rename = "recipeYield")]
    recipe_yield: Option<RecipeYield>,
}

fn main() {
    let url = "https://www.bbcgoodfood.com/recipes/spaghetti-meatballs";

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_str("text/html").unwrap());
    // You are a browser, no one will know
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36").unwrap());

    let client = blocking::Client::new();
    let response = client.get(url).headers(headers).send();
    
    let html_content = response.unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&html_content);
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();

    let mut ld_scripts = document.select(&selector);

    
    let recipe_string = ld_scripts.find(|script| {
        let html = script.inner_html();        
        let deserialised: Recipe = serde_json::from_str(&html).expect("cant deserialise");
        if deserialised.context == "https://schema.org" && deserialised.kind == "Recipe" {
            return true;
        } 
        false
    }).expect("No recipe found").inner_html();
    

    let recipe: Recipe = serde_json::from_str(&recipe_string).expect("cant deserialise");

    println!("{:#?}", recipe);



}
